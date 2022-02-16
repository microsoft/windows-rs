use super::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
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
    GUID,
    IUnknown,
    IInspectable,
    HRESULT,
    TypeName,             // Used for parsing attribute blobs
    GenericParam(String), // TODO: can be &'static str?
    MethodDef(MethodDef),
    Field(Field),
    TypeDef(TypeDef),
    MutPtr((Box<Self>, usize)),
    ConstPtr((Box<Self>, usize)),
    Win32Array((Box<Self>, u32)),
    WinrtArray(Box<Self>),
    WinrtArrayRef(Box<Self>),
    WinrtConstRef(Box<Self>),
}

impl Default for Type {
    fn default() -> Self {
        Self::Void
    }
}

impl From<TypeDef> for Type {
    fn from(def: TypeDef) -> Self {
        Self::TypeDef(def.with_generics())
    }
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Self::I64 | Self::U64 | Self::F64 => 2,
            Self::GUID => 4,
            Self::TypeDef(def) => def.size(),
            _ => 1,
        }
    }

    pub fn row(&self) -> &Row {
        match self {
            Self::MethodDef(def) => &def.0,
            Self::Field(def) => &def.0,
            Self::TypeDef(def) => &def.row,
            _ => unimplemented!(),
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

    pub fn type_name(&self) -> TypeName {
        match self {
            Self::TypeDef(def) => def.type_name(),
            _ => unimplemented!(),
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
            Self::GUID => "g16".to_owned(),
            Self::HRESULT => "struct(Windows.Foundation.HResult;i4)".to_owned(),
            Self::TypeDef(t) => t.type_signature(),
            _ => unimplemented!(),
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
            Self::Win32Array((kind, _)) => kind.is_blittable(),
            _ => true,
        }
    }

    pub fn is_convertible(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_convertible(),
            Self::String | Self::IInspectable | Self::GUID | Self::IUnknown | Self::GenericParam(_) => true,
            Self::WinrtConstRef(kind) => kind.is_convertible(),
            _ => false,
        }
    }

    pub fn is_callback(&self) -> bool {
        match self {
            // TODO: do we need to know there's a callback behind the pointer?
            Self::ConstPtr((kind, _)) | Self::MutPtr((kind, _)) => kind.is_callback(),
            Self::TypeDef(def) => def.is_callback(),
            _ => false,
        }
    }

    pub fn is_callback_array(&self) -> bool {
        match self {
            Self::Win32Array((kind, _)) => kind.is_callback(),
            _ => false,
        }
    }

    pub fn is_primitive(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_primitive(),
            Self::Bool | Self::Char | Self::I8 | Self::U8 | Self::I16 | Self::U16 | Self::I32 | Self::U32 | Self::I64 | Self::U64 | Self::F32 | Self::F64 | Self::ISize | Self::USize | Self::HRESULT | Self::ConstPtr(_) | Self::MutPtr(_) => true,
            _ => false,
        }
    }

    pub fn is_udt(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.is_udt(),
            Self::GUID => true,
            _ => false,
        }
    }

    pub fn has_union(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.has_union(),
            Self::Win32Array((kind, _)) => kind.has_union(),
            _ => false,
        }
    }

    pub fn has_pack(&self) -> bool {
        match self {
            Self::TypeDef(t) => t.has_pack(),
            Self::Win32Array((kind, _)) => kind.has_pack(),
            _ => false,
        }
    }

    pub fn is_handle(&self) -> bool {
        match self {
            Self::TypeDef(def) => def.is_handle(),
            _ => false,
        }
    }

    pub fn is_generic(&self) -> bool {
        matches!(self, Self::GenericParam(_))
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::ConstPtr(_) | Self::MutPtr(_))
    }

    pub fn is_void(&self) -> bool {
        match self {
            // TODO: do we care about void behind pointers?
            Self::ConstPtr((kind, _)) | Self::MutPtr((kind, _)) => kind.is_void(),
            Self::Void => true,
            _ => false,
        }
    }

    pub fn deref(&self) -> Self {
        match self {
            Self::ConstPtr((kind, 1)) => *kind.clone(),
            Self::MutPtr((kind, 1)) => *kind.clone(),
            Self::ConstPtr((kind, pointers)) => Self::ConstPtr((kind.clone(), pointers - 1)),
            Self::MutPtr((kind, pointers)) => Self::MutPtr((kind.clone(), pointers - 1)),
            _ => unimplemented!(),
        }
    }

    pub fn to_const(self) -> Self {
        match self {
            Self::MutPtr((kind, pointers)) => Self::ConstPtr((kind, pointers)),
            _ => self,
        }
    }

    pub fn is_winrt_array(&self) -> bool {
        matches!(self, Self::WinrtArray(_))
    }

    pub fn is_winrt_array_ref(&self) -> bool {
        matches!(self, Self::WinrtArrayRef(_))
    }

    pub fn is_winrt_const_ref(&self) -> bool {
        matches!(self, Self::WinrtConstRef(_))
    }

    #[must_use]
    pub fn underlying_type(&self) -> Self {
        match self {
            Self::TypeDef(def) => def.underlying_type(),
            Self::HRESULT => Self::I32,
            _ => self.clone(),
        }
    }

    pub fn has_replacement(&self) -> bool {
        match self {
            Self::HRESULT => true,
            Self::TypeDef(def) => def.is_handle(),
            _ => false,
        }
    }

    pub fn cfg(&self) -> Cfg {
        let mut cfg = Cfg::new();
        self.combine_cfg(&mut cfg);
        cfg.remove_feature(self.type_name().namespace);
        cfg
    }

    pub(crate) fn combine_cfg(&self, cfg: &mut Cfg) {
        match self {
            Self::MethodDef(def) => def.combine_cfg(cfg),
            Self::Field(def) => def.combine_cfg(None, cfg),
            Self::TypeDef(def) => def.combine_cfg(cfg),
            Self::Win32Array((def, _)) => def.combine_cfg(cfg),
            Self::ConstPtr((def, _)) => def.combine_cfg(cfg),
            Self::MutPtr((def, _)) => def.combine_cfg(cfg),
            Self::WinrtArray(def) => def.combine_cfg(cfg),
            Self::WinrtArrayRef(def) => def.combine_cfg(cfg),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfg() {
        let def = TypeReader::get().get_type(("Windows.Foundation", "IStringable")).unwrap();
        let namespaces = def.cfg().namespaces();
        assert_eq!(namespaces.len(), 0);

        let def = TypeReader::get().get_type(("Windows.Devices.Display.Core", "DisplayPresentationRate")).unwrap();
        let namespaces = def.cfg().namespaces();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], "Windows.Foundation.Numerics");

        let def = TypeReader::get().get_type(("Windows.Graphics.DirectX.Direct3D11", "Direct3DSurfaceDescription")).unwrap();
        let namespaces = def.cfg().namespaces();
        assert_eq!(namespaces.len(), 0);

        let def = TypeReader::get().get_type(("Windows.Win32.Security.Authorization.UI", "EFFPERM_RESULT_LIST")).unwrap();
        let namespaces = def.cfg().namespaces();
        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0], "Windows.Win32.Foundation");
    }
}
