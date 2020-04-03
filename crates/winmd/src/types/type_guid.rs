use crate::tables::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TypeGuid(pub [GuidConstant; 11]);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GuidConstant {
    U32(u32),
    U16(u16),
    U8(u8),
}

impl GuidConstant {
    fn from_arg(arg: &AttributeArg) -> GuidConstant {
        match arg {
            AttributeArg::U32(value) => GuidConstant::U32(*value),
            AttributeArg::U16(value) => GuidConstant::U16(*value),
            AttributeArg::U8(value) => GuidConstant::U8(*value),
            _ => panic!("Invalid Guid argument"),
        }
    }
}

// TODO: get rid of this
impl Default for GuidConstant {
    fn default() -> Self {
        Self::U8(0)
    }
}

impl TypeGuid {
    pub fn new() -> TypeGuid {
        Default::default()
    }

    pub fn from_args(args: Vec<(String, AttributeArg)>) -> TypeGuid {
        TypeGuid([
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
}
