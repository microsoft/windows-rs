use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum ElementType {
    NotYetSupported,
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
    Object,
    Guid,
    IUnknown,
    ErrorCode,
    Matrix3x2,
    TypeName,
    GenericParam(tables::GenericParam),

    Function(types::Function),
    Constant(types::Constant),
    Class(types::Class),
    Interface(types::Interface),
    ComInterface(types::ComInterface),
    Enum(types::Enum),
    Struct(types::Struct),
    Delegate(types::Delegate),
    Callback(types::Callback),
}

impl ElementType {
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
            0x1c => Some(Self::Object),
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
                let code = TypeDefOrRef::decode(blob.reader, blob.read_unsigned(), blob.file_index);

                match code {
                    TypeDefOrRef::TypeRef(type_ref) => match type_ref.full_name() {
                        ("System", "Guid") | ("Windows.Win32.Com", "Guid") => Self::Guid,
                        ("Windows.Win32.Com", "IUnknown") => Self::IUnknown,
                        ("Windows.Foundation", "HResult") => Self::ErrorCode,
                        ("Windows.Win32.Com", "HRESULT") => Self::ErrorCode,
                        ("Windows.Win32.SystemServices", "LARGE_INTEGER") => Self::I64,
                        ("Windows.Win32.SystemServices", "ULARGE_INTEGER") => Self::U64,
                        ("Windows.Win32.Direct2D", "D2D_MATRIX_3X2_F") => Self::Matrix3x2,
                        ("System", "Type") => Self::TypeName,
                        _ => Self::from_type_def(type_ref.resolve(), Vec::new()).unwrap(),
                    },
                    TypeDefOrRef::TypeDef(type_def) => {
                        // TODO: does this ever happen?
                        Self::from_type_def(type_def, Vec::new()).unwrap()
                    }
                    _ => unexpected!(),
                }
            }
            0x13 => generics[blob.read_unsigned() as usize].clone(),
            0x14 => Self::NotYetSupported, // arrays
            0x15 => {
                let def = GenericType::from_blob(blob, generics);
                match def.def.kind() {
                    TypeKind::Interface => Self::Interface(types::Interface(def)),
                    TypeKind::Delegate => Self::Delegate(types::Delegate(def)),
                    _ => unexpected!(),
                }
            }
            _ => unexpected!(),
        }
    }

    // TODO: this only returns Option<T> instead of just T because the TypeReader's cache still has constracts and attributes
    // that need to be excluded but are hard to do at that layer.
    pub fn from_type_def(def: tables::TypeDef, generics: Vec<Self>) -> Option<Self> {
        match def.kind() {
            TypeKind::Interface => {
                if def.is_winrt() {
                    Some(Self::Interface(types::Interface(
                        GenericType::from_type_def(def, generics),
                    )))
                } else {
                    Some(Self::ComInterface(types::ComInterface(
                        GenericType::from_type_def(def, generics),
                    )))
                }
            }
            TypeKind::Class => Some(Self::Class(types::Class(GenericType::from_type_def(
                def, generics,
            )))),
            TypeKind::Enum => Some(Self::Enum(types::Enum(def))),
            TypeKind::Struct => Some(Self::Struct(types::Struct(def))),
            TypeKind::Delegate => {
                if def.is_winrt() {
                    Some(Self::Delegate(types::Delegate(GenericType::from_type_def(
                        def, generics,
                    ))))
                } else {
                    Some(Self::Callback(types::Callback(def)))
                }
            }
            _ => None,
        }
    }

    pub fn gen_name(&self, gen: Gen) -> TokenStream {
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
                quote! { ::windows::HString }
            }
            Self::Object => {
                quote! { ::windows::Object }
            }
            Self::Guid => {
                quote! { ::windows::Guid }
            }
            Self::IUnknown => {
                quote! { ::windows::IUnknown }
            }
            Self::ErrorCode => {
                quote! { ::windows::ErrorCode }
            }
            Self::Matrix3x2 => {
                let numerics = gen.namespace("Windows.Foundation.Numerics");
                quote! { #numerics Matrix3x2 }
            }
            Self::NotYetSupported => {
                quote! { ::windows::NOT_YET_SUPPORTED_TYPE }
            }
            Self::GenericParam(generic) => generic.gen_name(),
            Self::Function(t) => t.gen_name(),
            Self::Constant(t) => t.gen_name(),
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

    pub fn gen_abi_name(&self, gen: Gen) -> TokenStream {
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
            Self::Object => {
                quote! { ::windows::RawPtr }
            }
            Self::Guid => {
                quote! { ::windows::Guid }
            }
            Self::IUnknown => {
                quote! { ::windows::RawPtr }
            }
            Self::ErrorCode => {
                quote! { ::windows::ErrorCode }
            }
            Self::Matrix3x2 => {
                let numerics = gen.namespace("Windows.Foundation.Numerics");
                quote! { #numerics Matrix3x2 }
            }
            Self::NotYetSupported => {
                quote! { ::windows::NOT_YET_SUPPORTED_TYPE }
            }
            Self::GenericParam(generic) => {
                let name = generic.gen_name();
                quote! { <#name as ::windows::Abi>::Abi }
            }
            Self::Class(_) => {
                quote! { ::windows::RawPtr }
            }
            Self::Interface(_) => {
                quote! { ::windows::RawPtr }
            }
            Self::ComInterface(_) => {
                quote! { ::windows::RawPtr }
            }
            Self::Enum(t) => t.0.gen_name(gen),
            Self::Struct(t) => t.gen_abi_name(gen),
            Self::Delegate(_) => {
                quote! { ::windows::RawPtr }
            }
            Self::Callback(_) => {
                quote! { ::windows::RawPtr }
            }
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
            Self::Object => "cinterface(IInspectable)".to_owned(),
            Self::Guid => "g16".to_owned(),
            Self::Class(t) => t.type_signature(),
            Self::Interface(t) => t.type_signature(),
            Self::Enum(t) => t.type_signature(),
            Self::Struct(t) => t.type_signature(),
            Self::Delegate(t) => t.type_signature(),
            _ => unexpected!(),
        }
    }

    // TODO: this should exclude dependencies interface method parameters so that those methods are only
    // generated if the types they refer to are actually included.
    pub fn dependencies(&self) -> Vec<tables::TypeDef> {
        match self {
            Self::Function(t) => t.dependencies(),
            Self::Class(t) => t.dependencies(),
            Self::Interface(t) => t.dependencies(),
            Self::ComInterface(t) => t.dependencies(),
            Self::Struct(t) => t.dependencies(),
            Self::Delegate(t) => t.dependencies(),
            Self::Callback(t) => t.dependencies(),
            _ => Vec::new(),
        }
    }

    pub fn definition(&self) -> Vec<tables::TypeDef> {
        match self {
            Self::Class(t) => t.definition(),
            Self::Interface(t) => t.definition(),
            Self::ComInterface(t) => t.definition(),
            Self::Struct(t) => t.definition(),
            Self::Delegate(t) => t.definition(),
            Self::Callback(t) => t.definition(),
            Self::Enum(t) => t.definition(),
            // TODO: find a cleaner way to map this dependency
            Self::Matrix3x2 => {
                vec![TypeReader::get().resolve_type_def("Windows.Foundation.Numerics", "Matrix3x2")]
            }
            _ => Vec::new(),
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            Self::Object
            | Self::IUnknown
            | Self::Function(_)
            | Self::Interface(_)
            | Self::Class(_)
            | Self::ComInterface(_)
            | Self::Delegate(_)
            | Self::Callback(_) => true,
            _ => false,
        }
    }

    pub fn is_blittable(&self) -> bool {
        match self {
            Self::String
            | Self::Object
            | Self::IUnknown
            | Self::GenericParam(_)
            | Self::Class(_)
            | Self::Interface(_)
            | Self::ComInterface(_)
            | Self::Delegate(_) => false,
            Self::Struct(def) => def.is_blittable(),
            _ => true,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::String
            | Self::Object
            | Self::Guid
            | Self::IUnknown
            | Self::Matrix3x2
            | Self::GenericParam(_)
            | Self::Class(_)
            | Self::Interface(_)
            | Self::ComInterface(_)
            | Self::Struct(_)
            | Self::Delegate(_) => true,
            _ => false,
        }
    }

    pub fn is_callback(&self) -> bool {
        if let Self::Callback(_) = self {
            true
        } else {
            false
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
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
            | Self::ErrorCode
            | Self::Enum(_) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Self::Guid | Self::Matrix3x2 => true,
            Self::Struct(t) => !t.is_handle(),
            _ => false,
        }
    }

    pub fn is_explicit(&self) -> bool {
        match self {
            Self::Struct(s) => {
                if s.0.flags().explicit() {
                    true
                } else {
                    s.0.fields().any(|f| f.signature().is_explicit())
                }
            }
            _ => false,
        }
    }

    pub fn gen(&self, gen: Gen) -> TokenStream {
        match self {
            Self::Function(t) => t.gen(gen),
            Self::Constant(t) => t.gen(),
            Self::Class(t) => t.gen(gen),
            Self::Interface(t) => t.gen(gen),
            Self::ComInterface(t) => t.gen(gen),
            Self::Enum(t) => t.gen(gen),
            Self::Struct(t) => t.gen(gen),
            Self::Delegate(t) => t.gen(gen),
            Self::Callback(t) => t.gen(gen),
            Self::GenericParam(p) => p.gen_name(),
            Self::Object => {
                quote! { ::windows::Object }
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
        assert_eq!(ElementType::Bool.gen_name(Gen::Absolute).as_str(), "bool");
    }

    #[test]
    fn test_struct() {
        let t = TypeReader::get().resolve_type("Windows.Win32.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "DXGI_FRAME_STATISTICS_MEDIA");

        let d = t.dependencies();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "DXGI_FRAME_PRESENTATION_MODE");
    }

    #[test]
    fn test_enum() {
        let t =
            TypeReader::get().resolve_type("Windows.Win32.Dxgi", "DXGI_FRAME_PRESENTATION_MODE");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "DXGI_FRAME_PRESENTATION_MODE");

        let d = t.dependencies();
        assert_eq!(d.len(), 0);
    }

    #[test]
    fn test_com_interface() {
        let t = TypeReader::get().resolve_type("Windows.Win32.Direct2D", "ID2D1Resource");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "ID2D1Resource");

        let d = t.dependencies();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "ID2D1Factory");
    }

    #[test]
    fn test_winrt_interface() {
        let t = TypeReader::get().resolve_type("Windows.Foundation", "IUriRuntimeClassFactory");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "IUriRuntimeClassFactory");

        let d = t.dependencies();
        assert_eq!(d.len(), 2);
        assert_eq!(d[0].name(), "Uri");
        assert_eq!(d[1].name(), "Uri");
    }

    #[test]
    fn test_winrt_interface2() {
        let t = TypeReader::get().resolve_type("Windows.Foundation", "IAsyncAction");
        let d = t.definition();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].name(), "IAsyncAction");

        let mut d = t.dependencies();
        assert_eq!(d.len(), 3);

        d.sort_by(|a, b| a.name().cmp(b.name()));

        assert_eq!(d[0].name(), "AsyncActionCompletedHandler");
        assert_eq!(d[1].name(), "AsyncActionCompletedHandler");
        assert_eq!(d[2].name(), "IAsyncInfo");
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
        let t = TypeReader::get().resolve_type("Windows.Win32.WindowsAndMessaging", "EnumWindows");
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
        let t = TypeReader::get().resolve_type("Windows.Win32.Dxgi", "DXGI_USAGE_SHADER_INPUT");
        assert_eq!(t.definition().len(), 0);
        assert_eq!(t.dependencies().len(), 0);
    }

    #[test]
    fn test_win32_callback() {
        let t = TypeReader::get().resolve_type("Windows.Win32.MenusAndResources", "WNDENUMPROC");
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

    // #[test]
    // fn test_winrt_class() {
    //     let t = TypeReader::get().resolve_type("Windows.Foundation.Collections", "StringMap");
    //     let d = t.definition().unwrap();
    //     assert_eq!(d.name(), "StringMap");

    //     let d = t.dependencies();
    //     assert_eq!(d.len(), 0);
    // }

    // #[test]
    // fn test_winrt_class2() {
    //     let t = TypeReader::get().resolve_type("Windows.Foundation", "WwwFormUrlDecoder");
    //     let d = t.definition().unwrap();
    //     assert_eq!(d.name(), "WwwFormUrlDecoder");

    //     let d = t.dependencies();
    //     assert_eq!(d.len(), 2);

    //     assert_eq!(d[0].name(), "IWwwFormUrlDecoderRuntimeClass");
    //     assert_eq!(d[1].name(), "IWwwFormUrlDecoderRuntimeClassFactory");
    // }
}
