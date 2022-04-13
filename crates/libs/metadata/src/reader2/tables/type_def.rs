use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TypeDef(pub Row);

// impl TypeDef {


//     pub fn underlying_type(&self, scope: &Reader) -> Type {
//         if let Some(field) = self.fields(scope).next() {
//             if let Some(constant) = field.constant(scope) {
//                 return constant.ty(scope);
//             } else {
//                 return field.ty(scope, Some(*self));
//             }
//         }

//         unimplemented!();
//     }
//     pub fn stdcall(&self, scope:&Reader) -> usize {
//         if self.kind(scope) == TypeDefKind::Struct {
//             if self.flags(scope).union() {
//                 self.fields(scope).map(|field| field.ty(scope, Some(*self)).stdcall(scope)).max().unwrap_or(1)
//             } else {
//                 self.fields(scope).fold(0, |sum, field| sum + field.ty(scope, Some(*self)).stdcall(scope))
//             }
//         } else {
//             4
//         }
//     }
//     pub fn kind(&self, scope: &Reader) -> TypeDefKind {
//         if self.flags(scope).interface() {
//             TypeDefKind::Interface
//         } else {
//             match self.extends(scope).type_name(scope) {
//                 TypeName::Enum => TypeDefKind::Enum,
//                 TypeName::Delegate => TypeDefKind::Delegate,
//                 TypeName::Struct => TypeDefKind::Struct,
//                 _ => TypeDefKind::Class,
//             }
//         }
//     }
// }
