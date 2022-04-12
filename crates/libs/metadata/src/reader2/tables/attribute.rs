use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Attribute(pub ScopeKey);

impl Attribute {
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        let AttributeType::MemberRef(member) = scope.decode(self.0, 1);
        let MemberRefParent::TypeRef(type_ref) = member.parent(scope);
        type_ref.name(scope)
    }
    pub fn args(&self, scope: &Scope) -> Vec<(&str, Value)> {
        let AttributeType::MemberRef(member) = scope.decode(self.0, 1);
        let mut sig = member.signature(scope);
        let mut values = scope.blob(self.0, 2);
        let _prolog = values.read_u16();
        let _this_and_gen_param_count = sig.read_usize();
        let fixed_arg_count = sig.read_usize();
        let _ret_type = sig.read_usize();
        let mut args: Vec<(&str, Value)> = Vec::with_capacity(fixed_arg_count as usize);

        for _ in 0..fixed_arg_count {}

        args
    }
}
