use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum ElementType {
    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    Guid,
    IUnknown,
    IInspectable,
    HRESULT,
    Matrix3x2,
    TypeName,
    GenericParam(tables::GenericParam),
    Array((Box<Signature>, u32)),
    MethodDef(tables::MethodDef),
    Field(tables::Field),

    // TODO: these should all be TypeDef(TypeDef)
    Class(types::Class),
    Interface(types::Interface),
    ComInterface(types::ComInterface),
    Enum(types::Enum),
    Struct(types::Struct),
    Delegate(types::Delegate),
    Callback(types::Callback),
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Void
    }
}

impl From<tables::TypeDef> for ElementType {
    fn from(def: tables::TypeDef) -> Self {
        match def.kind() {
            TypeKind::Interface => {
                if def.is_winrt() {
                    Self::Interface(types::Interface(def.with_generics()))
                } else {
                    Self::ComInterface(types::ComInterface(def.clone()))
                }
            }
            TypeKind::Class => {
                Self::Class(types::Class(def.clone()))
            }
            TypeKind::Enum => Self::Enum(types::Enum(def.clone())),
            TypeKind::Struct => Self::Struct(types::Struct(def.clone())),
            TypeKind::Delegate => {
                if def.is_winrt() {
                    Self::Delegate(types::Delegate(def.with_generics()))
                } else {
                    Self::Callback(types::Callback(def.clone()))
                }
            }
        }
    }
}

impl ElementType {
    pub fn row(&self) -> Row {
        match self {
            Self::MethodDef(def) => def.0,
            Self::Field(def) => def.0,
            Self::Class(def) => *def.0.row(),
            Self::Interface(def) => *def.0.row(),
            Self::ComInterface(def) => *def.0.row(),
            Self::Enum(def) => *def.0.row(),
            Self::Struct(def) => *def.0.row(),
            Self::Delegate(def) => *def.0.row(),
            Self::Callback(def) => *def.0.row(),
            _ => unexpected!(),
        }
    }

    pub fn namespace(&self) -> &'static str {
        match self {
            Self::MethodDef(def) => def.parent().namespace(),
            Self::Field(def) => def.parent().namespace(),
            Self::Class(def) => def.0.namespace(),
            Self::Interface(def) => def.0.namespace(),
            Self::ComInterface(def) => def.0.namespace(),
            Self::Enum(def) => def.0.namespace(),
            Self::Struct(def) => def.0.namespace(),
            Self::Delegate(def) => def.0.namespace(),
            Self::Callback(def) => def.0.namespace(),
            _ => "",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::MethodDef(def) => def.name(),
            Self::Field(def) => def.name(),
            Self::Class(def) => def.0.name(),
            Self::Interface(def) => def.0.name(),
            Self::ComInterface(def) => def.0.name(),
            Self::Enum(def) => def.0.name(),
            Self::Struct(def) => def.0.name(),
            Self::Delegate(def) => def.0.name(),
            Self::Callback(def) => def.0.name(),
            _ => "",
        }
    }

    pub fn from_code(code: u32) -> Option<Self> {
        match code {
            0x01 => Some(Self::Void),
            0x02 => Some(Self::Bool),
            0x03 => Some(Self::Char),
            0x04 => Some(Self::I8),
            0x05 => Some(Self::U8),
            0x06 => Some(Self::I16),
            0x07 => Some(Self::U16),
            0x08 => Some(Self::I32),
            0x09 => Some(Self::U32),
            0x0a => Some(Self::I64),
            0x0b => Some(Self::U64),
            0x0c => Some(Self::F32),
            0x0d => Some(Self::F64),
            0x18 => Some(Self::ISize),
            0x19 => Some(Self::USize),
            0x0e => Some(Self::String),
            0x1c => Some(Self::IInspectable),
            _ => None,
        }
    }

    pub fn from_blob(blob: &mut Blob, generics: &[Self]) -> Self {
        let code = blob.read_unsigned();

        if let Some(code) = Self::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => {
                let code = TypeDefOrRef::decode(blob.file, blob.read_unsigned());

                match code {
                    TypeDefOrRef::TypeRef(type_ref) => match type_ref.full_name() {
                        ("System", "Guid") => Self::Guid,
                        ("Windows.Win32.System.Com", "IUnknown") => Self::IUnknown,
                        ("Windows.Foundation", "HResult") => Self::HRESULT,
                        ("Windows.Win32.System.Com", "HRESULT") => Self::HRESULT,
                        ("Windows.Win32.System.WinRT", "HSTRING") => Self::String,
                        ("Windows.Win32.System.WinRT", "IInspectable") => Self::IInspectable,
                        ("Windows.Win32.System.SystemServices", "LARGE_INTEGER") => Self::I64,
                        ("Windows.Win32.System.SystemServices", "ULARGE_INTEGER") => Self::U64,
                        ("Windows.Win32.Graphics.Direct2D", "D2D_MATRIX_3X2_F") => Self::Matrix3x2,
                        ("System", "Type") => Self::TypeName,
                        _ => type_ref.resolve().into(),
                    },
                    TypeDefOrRef::TypeDef(type_def) => 
                        // Need to "re-resolve" the TypeDef as it may point to an arch-specific
                        // definition. This lets the TypeTree be built for a specific architecture
                        // without accidentally pulling in the wrong definition.
                        TypeReader::get().resolve_type_def(type_def.namespace(), type_def.name()).into(),
                    _ => unexpected!(),
                }
            }
            0x13 => generics[blob.read_unsigned() as usize].clone(),
            0x14 => {
                let kind = Signature::from_blob(blob, generics).unwrap();
                let _rank = blob.read_unsigned();
                let _bounds_count = blob.read_unsigned();
                let bounds = blob.read_unsigned();
                Self::Array((Box::new(kind), bounds))
            }
            0x15 => {
                let def = tables::TypeDef::from_blob(blob, generics);
                match def.kind() {
                    TypeKind::Interface => Self::Interface(types::Interface(def)),
                    TypeKind::Delegate => Self::Delegate(types::Delegate(def)),
                    _ => unexpected!(),
                }
            }
            _ => unexpected!(),
        }
    }

    pub fn gen_name(&self, gen: &Gen) -> TokenStream {
        match self {
            Self::Void => quote! { ::std::ffi::c_void },
            Self::Bool => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::String => {
                quote! { ::windows::HSTRING }
            }
            Self::IInspectable => {
                quote! { ::windows::IInspectable }
            }
            Self::Guid => {
                quote! { ::windows::Guid }
            }
            Self::IUnknown => {
                quote! { ::windows::IUnknown }
            }
            Self::HRESULT => {
                quote! { ::windows::HRESULT }
            }
            Self::Matrix3x2 => {
                let numerics = gen.namespace("Windows.Foundation.Numerics");
                quote! { #numerics Matrix3x2 }
            }
            Self::Array((kind, len)) => {
                let name = kind.gen_win32(gen);
                let len = Literal::u32_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::GenericParam(generic) => generic.gen_name(),
            Self::MethodDef(t) => t.gen_name(gen),
            Self::Field(t) => t.gen_name(),
            Self::Class(t) => t.0.gen_name(gen),
            Self::Interface(t) => t.0.gen_name(gen),
            Self::ComInterface(t) => t.0.gen_name(gen),
            Self::Enum(t) => t.0.gen_name(gen),
            Self::Struct(t) => t.0.gen_name(gen),
            Self::Delegate(t) => t.0.gen_name(gen),
            Self::Callback(t) => t.0.gen_name(gen),
            _ => unexpected!(),
        }
    }

    pub fn gen_abi_type(&self, gen: &Gen) -> TokenStream {
        match self {
            Self::Void => quote! { ::std::ffi::c_void },
            Self::Bool => quote! { bool },
            Self::Char => quote! { u16 },
            Self::I8 => quote! { i8 },
            Self::U8 => quote! { u8 },
            Self::I16 => quote! { i16 },
            Self::U16 => quote! { u16 },
            Self::I32 => quote! { i32 },
            Self::U32 => quote! { u32 },
            Self::I64 => quote! { i64 },
            Self::U64 => quote! { u64 },
            Self::F32 => quote! { f32 },
            Self::F64 => quote! { f64 },
            Self::ISize => quote! { isize },
            Self::USize => quote! { usize },
            Self::String => {
                quote! { ::windows::RawPtr }
            }
            Self::IInspectable => {
                quote! { ::windows::RawPtr }
            }
            Self::Guid => {
                quote! { ::windows::Guid }
            }
            Self::IUnknown => {
                quote! { ::windows::RawPtr }
            }
            Self::HRESULT => {
                quote! { ::windows::HRESULT }
            }
            Self::Matrix3x2 => {
                let numerics = gen.namespace("Windows.Foundation.Numerics");
                quote! { #numerics Matrix3x2 }
            }
            Self::Array((kind, len)) => {
                let name = kind.gen_win32_abi(gen);
                let len = Literal::u32_unsuffixed(*len);
                quote! { [#name; #len] }
            }
            Self::GenericParam(generic) => {
                let name = generic.gen_name();
                quote! { <#name as ::windows::Abi>::Abi }
            }
            Self::Class(def) => def.0.gen_abi_type(gen),
            Self::Interface(def) => def.0.gen_abi_type(gen),
            Self::ComInterface(def) => def.0.gen_abi_type(gen),
            Self::Enum(def) => def.0.gen_abi_type(gen),
            Self::Struct(def) => def.0.gen_abi_type(gen),
            Self::Delegate(def) => def.0.gen_abi_type(gen),
            Self::Callback(def) => def.0.gen_abi_type(gen),
            _ => unexpected!(),
        }
    }

    pub fn gen_default(&self) -> TokenStream {
        match self {
            Self::Bool => quote! { false },
            Self::Char
            | Self::I8
            | Self::U8
            | Self::I16
            | Self::U16
            | Self::I32
            | Self::U32
            | Self::I64
            | Self::U64
            | Self::ISize
            | Self::USize => quote! { 0 },
            Self::F32 | Self::F64 => quote! { 0.0 },
            Self::Array((kind, len)) => {
                let default = kind.gen_win32_default();
                let len = Literal::u32_unsuffixed(*len);
                quote! { [#default; #len] }
            }
            _ => quote! { ::std::default::Default::default() },
        }
    }

    pub fn type_signature(&self) -> String {
        match self {
            Self::Bool => "b1".to_owned(),
            Self::Char => "c2".to_owned(),
            Self::I8 => "i1".to_owned(),
            Self::U8 => "u1".to_owned(),
            Self::I16 => "i2".to_owned(),
            Self::U16 => "u2".to_owned(),
            Self::I32 => "i4".to_owned(),
            Self::U32 => "u4".to_owned(),
            Self::I64 => "i8".to_owned(),
            Self::U64 => "u8".to_owned(),
            Self::F32 => "f4".to_owned(),
            Self::F64 => "f8".to_owned(),
            Self::String => "string".to_owned(),
            Self::IInspectable => "cinterface(IInspectable)".to_owned(),
            Self::Guid => "g16".to_owned(),
            Self::Class(t) => t.0.type_signature(),
            Self::Interface(t) => t.0.type_signature(),
            Self::Enum(t) => t.0.type_signature(),
            Self::Struct(t) => t.0.type_signature(),
            Self::Delegate(t) => t.0.type_signature(),
            _ => unexpected!(),
        }
    }

    pub fn dependencies(&self) -> Vec<ElementType> {
        match self {
            Self::MethodDef(t) => t.dependencies(&[]),
            Self::Class(t) => t.0.dependencies(),
            Self::Interface(t) => t.0.dependencies(),
            Self::ComInterface(t) => t.0.dependencies(),
            Self::Struct(t) => t.0.dependencies(),
            Self::Delegate(t) => t.0.dependencies(),
            Self::Callback(t) => t.0.dependencies(),
            Self::Field(t) => t.dependencies(),
            Self::Array((signature, _)) => signature.dependencies(),
            _ => Vec::new(),
        }
    }

    pub fn definition(&self) -> Vec<ElementType> {
        match self {
            Self::Class(t) => t.0.definition(),
            Self::Interface(t) => t.0.definition(),
            Self::ComInterface(t) => t.0.definition(),
            Self::Struct(t) => t.0.definition(),
            Self::Delegate(t) => t.0.definition(),
            Self::Callback(t) => t.0.definition(),
            Self::Enum(t) => t.0.definition(),
            Self::Array((signature, _)) => signature.definition(),
            // TODO: find a cleaner way to map this dependency
            Self::Matrix3x2 => {
                vec![TypeReader::get().resolve_type("Windows.Foundation.Numerics", "Matrix3x2")]
            }
            _ => Vec::new(),
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            Self::Class(t) => t.0.is_nullable(),
            Self::Interface(t) => t.0.is_nullable(),
            Self::ComInterface(t) => t.0.is_nullable(),
            Self::Struct(t) => t.0.is_nullable(),
            Self::Delegate(t) => t.0.is_nullable(),
            Self::Callback(t) => t.0.is_nullable(),
            Self::Enum(t) => t.0.is_nullable(),
            Self::IInspectable
                | Self::IUnknown
                | Self::MethodDef(_) => true,
            _ => false,
        }
    }

    pub fn is_blittable(&self) -> bool {
        match self {
            Self::Class(t) => t.0.is_blittable(),
            Self::Interface(t) =>t.0.is_blittable(),
            Self::ComInterface(t) => t.0.is_blittable(),
            Self::Enum(t) => t.0.is_blittable(),
            Self::Struct(t) => t.0.is_blittable(),
            Self::Delegate(t) => t.0.is_blittable(),
            Self::Callback(t) => t.0.is_blittable(),     
            Self::String
            | Self::IInspectable
            | Self::IUnknown
            | Self::GenericParam(_) => false,
            Self::Array((kind, _)) => kind.is_blittable(),
            _ => true,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::Class(t) => t.0.is_convertible(),
            Self::Interface(t) =>t.0.is_convertible(),
            Self::ComInterface(t) => t.0.is_convertible(),
            Self::Enum(t) => t.0.is_convertible(),
            Self::Struct(t) => t.0.is_convertible(),
            Self::Delegate(t) => t.0.is_convertible(),
            Self::Callback(t) => t.0.is_convertible(),            
            Self::String
                | Self::IInspectable
                | Self::Guid
                | Self::IUnknown
                | Self::Matrix3x2
                | Self::GenericParam(_) => true,
                _ => false,
        }
    }

    pub fn is_callback(&self) -> bool {
        matches!(self, Self::Callback(_))
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::Class(t) => t.0.is_primitive(),
            Self::Interface(t) =>t.0.is_primitive(),
            Self::ComInterface(t) => t.0.is_primitive(),
            Self::Enum(t) => t.0.is_primitive(),
            Self::Struct(t) => t.0.is_primitive(),
            Self::Delegate(t) => t.0.is_primitive(),
            Self::Callback(t) => t.0.is_primitive(),            
            Self::Bool
                | Self::Char
                | Self::I8
                | Self::U8
                | Self::I16
                | Self::U16
                | Self::I32
                | Self::U32
                | Self::I64
                | Self::U64
                | Self::F32
                | Self::F64
                | Self::ISize
                | Self::USize
                | Self::HRESULT => true,
                _ => false,
        }
    }

    pub fn is_udt(&self) -> bool {
        match self {
            Self::Class(t) => t.0.is_udt(),
            Self::Interface(t) =>t.0.is_udt(),
            Self::ComInterface(t) => t.0.is_udt(),
            Self::Enum(t) => t.0.is_udt(),
            Self::Struct(t) => t.0.is_udt(),
            Self::Delegate(t) => t.0.is_udt(),
            Self::Callback(t) => t.0.is_udt(),
            Self::Guid | Self::Matrix3x2 => true,
            _ => false,
        }
    }

    pub fn is_explicit(&self) -> bool {
        match self {
            Self::Class(t) => t.0.is_explicit(),
            Self::Interface(t) =>t.0.is_explicit(),
            Self::ComInterface(t) => t.0.is_explicit(),
            Self::Enum(t) => t.0.is_explicit(),
            Self::Struct(t) => t.0.is_explicit(),
            Self::Delegate(t) => t.0.is_explicit(),
            Self::Callback(t) => t.0.is_explicit(),
            Self::Array((kind, _)) => kind.is_explicit(),
            _ => false,
        }
    }

    pub fn gen(&self, gen: &Gen) -> TokenStream {
        match self {
            Self::MethodDef(t) => t.gen(gen),
            Self::Field(t) => t.gen(gen),
            Self::Class(t) => t.0.gen(gen),
            Self::Interface(t) => t.0.gen(gen),
            Self::ComInterface(t) => t.0.gen(gen),
            Self::Enum(t) => t.0.gen(gen),
            Self::Struct(t) => t.0.gen(gen),
            Self::Delegate(t) => t.0.gen(gen),
            Self::Callback(t) => t.0.gen(gen),
            Self::GenericParam(p) => p.gen_name(),
            Self::IInspectable => {
                quote! { ::windows::IInspectable }
            }
            _ => unexpected!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool() {
        assert_eq!(
            ElementType::Bool
                .gen_name(&Gen::absolute(&TypeTree::from_namespace("")))
                .as_str(),
            "bool"
        );
    }

    #[test]
    fn test_struct() {
        let t = TypeReader::get()
            .resolve_type("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "DXGI_FRAME_STATISTICS_MEDIA");

        let d = t.dependencies();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "DXGI_FRAME_PRESENTATION_MODE");
    }

    #[test]
    fn test_enum() {
        let t = TypeReader::get().resolve_type(
            "Windows.Win32.Graphics.Dxgi",
            "DXGI_FRAME_PRESENTATION_MODE",
        );
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "DXGI_FRAME_PRESENTATION_MODE");

        let d = t.dependencies();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_com_interface() {
        let t = TypeReader::get().resolve_type("Windows.Win32.Graphics.Direct2D", "ID2D1Resource");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "ID2D1Resource");

        let d = t.dependencies();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_winrt_interface() {
        let t = TypeReader::get().resolve_type("Windows.Foundation", "IUriRuntimeClassFactory");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "IUriRuntimeClassFactory");

        let d = t.dependencies();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_winrt_interface2() {
        let t = TypeReader::get().resolve_type("Windows.Foundation", "IAsyncAction");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "IAsyncAction");

        let d = t.dependencies();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "IAsyncInfo");
    }

    #[test]
    fn test_winrt_delegate() {
        let t = TypeReader::get().resolve_type("Windows.Foundation", "AsyncActionCompletedHandler");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "AsyncActionCompletedHandler");

        let mut d = t.dependencies();
        assert_eq!(d.len(), 2);

        d.sort_by(|a, b| a.name().cmp(b.name()));

        assert_eq!(d[0].name(), "AsyncStatus");
        assert_eq!(d[1].name(), "IAsyncAction");
    }

    #[test]
    fn test_win32_function() {
        let t =
            TypeReader::get().resolve_type("Windows.Win32.UI.WindowsAndMessaging", "EnumWindows");
        assert_eq!(t.definition().len(), 0);

        let mut d = t.dependencies();
        assert_eq!(d.len(), 3);

        d.sort_by(|a, b| a.name().cmp(b.name()));

        assert_eq!(d[0].name(), "BOOL");
        assert_eq!(d[1].name(), "LPARAM");
        assert_eq!(d[2].name(), "WNDENUMPROC");
    }

    #[test]
    fn test_win32_constant() {
        let t = TypeReader::get()
            .resolve_type("Windows.Win32.Graphics.Dxgi", "DXGI_USAGE_SHADER_INPUT");
        assert_eq!(t.definition().len(), 0);
        assert_eq!(t.dependencies().len(), 0);
    }

    #[test]
    fn test_win32_callback() {
        let t =
            TypeReader::get().resolve_type("Windows.Win32.UI.WindowsAndMessaging", "WNDENUMPROC");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "WNDENUMPROC");

        let mut d = t.dependencies();
        assert_eq!(d.len(), 3);

        d.sort_by(|a, b| a.name().cmp(b.name()));

        assert_eq!(d[0].name(), "BOOL");
        assert_eq!(d[1].name(), "HWND");
        assert_eq!(d[2].name(), "LPARAM");
    }
}
