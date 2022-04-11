use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute<'a>(pub Row<'a>);

impl<'a> Attribute<'a> {
    pub fn ty(&self) -> TypeRef {
        let AttributeType::MemberRef(member) = self.0.decode(1);
        let MemberRefParent::TypeRef(type_ref) = member.parent();
        type_ref
    }
    pub fn args(&self) -> Vec<(&str, Value)> {
        let AttributeType::MemberRef(member) = self.0.decode(1);
        let mut sig = member.signature();
        let mut values = self.0.blob(2);
        let _prolog = values.read_u16();
        let _this_and_gen_param_count = sig.read_usize();
        let fixed_arg_count = sig.read_usize();
        let _ret_type = sig.read_usize();
        let mut args: Vec<(&str, Value)> = Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {
        }

        args
     }
}
