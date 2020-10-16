use squote::{quote, Literal, TokenStream};

#[derive(Clone, Default, PartialEq)]
pub struct TypeGuid(pub [GuidConstant; 11]);

#[derive(Clone, PartialEq)]
pub enum GuidConstant {
    U32(u32),
    U16(u16),
    U8(u8),
}

impl std::fmt::Debug for GuidConstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::U32(value) => write!(f, "{:08x?}", value),
            Self::U16(value) => write!(f, "{:04x?}", value),
            Self::U8(value) => write!(f, "{:02x?}", value),
        }
    }
}

impl GuidConstant {
    fn from_arg(arg: &winmd::AttributeArg) -> GuidConstant {
        match *arg {
            winmd::AttributeArg::U32(value) => GuidConstant::U32(value),
            winmd::AttributeArg::U16(value) => GuidConstant::U16(value),
            winmd::AttributeArg::U8(value) => GuidConstant::U8(value),
            _ => panic!("Invalid Guid argument"),
        }
    }
}

impl std::fmt::Debug for TypeGuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}-{:?}-{:?}-{:?}{:?}-{:?}{:?}{:?}{:?}{:?}{:?}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10]
        )
    }
}

impl Default for GuidConstant {
    fn default() -> Self {
        Self::U8(0)
    }
}

impl TypeGuid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_type_def(reader: &winmd::TypeReader, def: winmd::TypeDef) -> Self {
        let args = def
            .attribute(reader, ("Windows.Foundation.Metadata", "GuidAttribute"))
            .args(reader);

        Self([
            GuidConstant::from_arg(&args[0].1),
            GuidConstant::from_arg(&args[1].1),
            GuidConstant::from_arg(&args[2].1),
            GuidConstant::from_arg(&args[3].1),
            GuidConstant::from_arg(&args[4].1),
            GuidConstant::from_arg(&args[5].1),
            GuidConstant::from_arg(&args[6].1),
            GuidConstant::from_arg(&args[7].1),
            GuidConstant::from_arg(&args[8].1),
            GuidConstant::from_arg(&args[9].1),
            GuidConstant::from_arg(&args[10].1),
        ])
    }

    pub fn gen(&self) -> TokenStream {
        let mut iter = self.0.iter().map(|value| match value {
            GuidConstant::U32(value) => Literal::u32_unsuffixed(*value),
            GuidConstant::U16(value) => Literal::u16_unsuffixed(*value),
            GuidConstant::U8(value) => Literal::u8_unsuffixed(*value),
        });

        let three = iter.by_ref().take(3);

        quote! {
            #(#three,)* [#(#iter),*],
        }
    }
}
