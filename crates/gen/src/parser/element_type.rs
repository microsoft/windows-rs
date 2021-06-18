use super::*;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
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
    TypeDef(tables::TypeDef),
}

impl Default for ElementType {
    fn default() -> Self {
        Self::Void
    }
}

impl From<tables::TypeDef> for ElementType {
    fn from(def: tables::TypeDef) -> Self {
        Self::TypeDef(def.with_generics())
    }
}

impl ElementType {
    pub fn row(&self) -> &Row {
        match self {
            Self::MethodDef(def) => &def.0,
            Self::Field(def) => &def.0,
            Self::TypeDef(def) => &def.row,
            _ => unexpected!(),
        }
    }

    pub fn namespace(&self) -> &'static str {
        match self {
            Self::MethodDef(def) => def.parent().namespace(),
            Self::Field(def) => def.parent().namespace(),
            Self::TypeDef(def) => def.namespace(),
            _ => "",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::MethodDef(def) => def.name(),
            Self::Field(def) => def.name(),
            Self::TypeDef(def) => def.name(),
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
            Self::TypeDef(t) => t.gen_name(gen),
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
            Self::TypeDef(def) => def.gen_abi_type(gen),
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
            Self::TypeDef(t) => t.type_signature(),
            _ => unexpected!(),
        }
    }

    // TODO: all dependencies methods should take a TypeInclude parameter and return a tuple?
    pub fn dependencies(&self, include: TypeInclude) -> Vec<TypeEntry> {
        match self {
            Self::MethodDef(t) => t.dependencies(include),
            Self::TypeDef(t) => t.dependencies(include),
            Self::Field(t) => t.dependencies(include),
            Self::Array((signature, _)) => signature.dependencies(include),
            _ => Vec::new(),
        }
    }

    pub fn definition(&self, include: TypeInclude) -> Vec<TypeEntry> {
        match self {
            Self::TypeDef(t) => t.definition(include),
            Self::Array((signature, _)) => signature.definition(include),
            // TODO: find a cleaner way to map this dependency
            Self::Matrix3x2 => {
                vec![TypeEntry{ include, def: TypeRow::TypeDef(TypeReader::get().resolve_type_def(
                    "Windows.Foundation.Numerics",
                    "Matrix3x2",
                ))}]
            }
            _ => Vec::new(),
        }
    }

    pub fn is_nullable(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_nullable(),
            Self::IInspectable | Self::IUnknown | Self::MethodDef(_) => true,
            _ => false,
        }
    }

    pub fn is_blittable(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_blittable(),
            Self::String | Self::IInspectable | Self::IUnknown | Self::GenericParam(_) => false,
            Self::Array((kind, _)) => kind.is_blittable(),
            _ => true,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_convertible(),
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
        match self {
            Self::TypeDef(def) => def.is_callback(),
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_primitive(),
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
            Self::TypeDef(t) => t.is_udt(),
            Self::Guid | Self::Matrix3x2 => true,
            _ => false,
        }
    }

    pub fn is_explicit(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_explicit(),
            Self::Array((kind, _)) => kind.is_explicit(),
            _ => false,
        }
    }

    // TODO: remove this
    pub fn gen(&self) -> TokenStream {
        match self {
            Self::GenericParam(p) => p.gen_name(),
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
            ElementType::Bool.gen_name(&Gen::absolute()).as_str(),
            "bool"
        );
    }

    // #[test]
    // fn test_struct() {
    //     let t = TypeReader::get()
    //         .resolve_type("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "DXGI_FRAME_STATISTICS_MEDIA");

    //     let d = t.dependencies();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "DXGI_FRAME_PRESENTATION_MODE");
    // }

    // #[test]
    // fn test_enum() {
    //     let t = TypeReader::get().resolve_type(
    //         "Windows.Win32.Graphics.Dxgi",
    //         "DXGI_FRAME_PRESENTATION_MODE",
    //     );
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "DXGI_FRAME_PRESENTATION_MODE");

    //     let d = t.dependencies();
    //     assert_eq!(d.len(), 0);
    // }

    // #[test]
    // fn test_com_interface() {
    //     let t = TypeReader::get().resolve_type("Windows.Win32.Graphics.Direct2D", "ID2D1Resource");
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "ID2D1Resource");

    //     let d = t.dependencies();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "ID2D1Factory");
    // }

    // #[test]
    // fn test_winrt_interface() {
    //     let t = TypeReader::get().resolve_type("Windows.Foundation", "IUriRuntimeClassFactory");
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "IUriRuntimeClassFactory");

    //     let d = t.dependencies();
    //     assert_eq!(d.len(), 2);
    //     assert_eq!(d[0].name(), "Uri");
    //     assert_eq!(d[1].name(), "Uri");
    // }

    // #[test]
    // fn test_winrt_interface2() {
    //     let t = TypeReader::get().resolve_type("Windows.Foundation", "IAsyncAction");
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "IAsyncAction");

    //     let mut d = t.dependencies();
    //     assert_eq!(d.len(), 3);
    //     d.sort_by(|a, b| a.name().cmp(b.name()));
    //     assert_eq!(d[0].name(), "AsyncActionCompletedHandler");
    //     assert_eq!(d[1].name(), "AsyncActionCompletedHandler");
    //     assert_eq!(d[2].name(), "IAsyncInfo");
    // }

    // #[test]
    // fn test_winrt_delegate() {
    //     let t = TypeReader::get().resolve_type("Windows.Foundation", "AsyncActionCompletedHandler");
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "AsyncActionCompletedHandler");

    //     let mut d = t.dependencies();
    //     assert_eq!(d.len(), 2);

    //     d.sort_by(|a, b| a.name().cmp(b.name()));

    //     assert_eq!(d[0].name(), "AsyncStatus");
    //     assert_eq!(d[1].name(), "IAsyncAction");
    // }

    // #[test]
    // fn test_win32_function() {
    //     let t =
    //         TypeReader::get().resolve_type("Windows.Win32.UI.WindowsAndMessaging", "EnumWindows");
    //     assert_eq!(t.definition().len(), 0);

    //     let mut d = t.dependencies();
    //     assert_eq!(d.len(), 3);

    //     d.sort_by(|a, b| a.name().cmp(b.name()));

    //     assert_eq!(d[0].name(), "BOOL");
    //     assert_eq!(d[1].name(), "LPARAM");
    //     assert_eq!(d[2].name(), "WNDENUMPROC");
    // }

    // #[test]
    // fn test_win32_constant() {
    //     let t = TypeReader::get()
    //         .resolve_type("Windows.Win32.Graphics.Dxgi", "DXGI_USAGE_SHADER_INPUT");
    //     assert_eq!(t.definition().len(), 0);
    //     assert_eq!(t.dependencies().len(), 0);
    // }

    // #[test]
    // fn test_win32_callback() {
    //     let t =
    //         TypeReader::get().resolve_type("Windows.Win32.UI.WindowsAndMessaging", "WNDENUMPROC");
    //     let d = t.definition();
    //     assert_eq!(d.len(), 1);
    //     assert_eq!(d[0].name(), "WNDENUMPROC");

    //     let mut d = t.dependencies();
    //     assert_eq!(d.len(), 3);

    //     d.sort_by(|a, b| a.name().cmp(b.name()));

    //     assert_eq!(d[0].name(), "BOOL");
    //     assert_eq!(d[1].name(), "HWND");
    //     assert_eq!(d[2].name(), "LPARAM");
    // }
}
