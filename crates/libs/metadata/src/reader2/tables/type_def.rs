use super::*;

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct TypeDef(pub ScopeKey);

impl TypeDef {
    pub fn flags(&self, scope: &Scope) -> TypeAttributes {
        TypeAttributes(scope.usize(self.0, 0))
    }
    pub fn name<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 1)
    }
    pub fn namespace<'a>(&self, scope: &'a Scope) -> &'a str {
        scope.str(self.0, 2)
    }
    pub fn type_name<'a>(&self, scope: &'a Scope) -> TypeName<'a> {
        TypeName::new(self.namespace(scope), self.name(scope))
    }
    pub fn extends(&self, scope: &Scope) -> TypeDefOrRef {
        scope.decode(self.0, 3)
    }
    pub fn fields(&self, scope: &Scope) -> impl Iterator<Item = Field> {
        scope.list(self.0, TABLE_FIELD, 4).map(Field)
    }
    pub fn methods(&self, scope: &Scope) -> impl Iterator<Item = MethodDef> {
        scope.list(self.0, TABLE_METHODDEF, 5).map(MethodDef)
    }
    pub fn attributes(&self, scope: &Scope) -> impl Iterator<Item = Attribute> {
        scope.attributes(self.0, HasAttribute::TypeDef(*self))
    }
    pub fn generic_params(&self, scope: &Scope) -> impl Iterator<Item = GenericParam> {
        scope.equal_range(self.0, TABLE_GENERICPARAM, 2, TypeOrMethodDef::TypeDef(self.clone()).encode()).map(GenericParam)
    }
    pub fn interface_impls(&self, scope: &Scope) -> impl Iterator<Item = InterfaceImpl> {
        scope.equal_range(self.0, TABLE_INTERFACEIMPL, 0, (self.0.row + 1) as _).map(InterfaceImpl)
    }
    pub fn underlying_type(&self, scope: &Scope) -> Type {
        if let Some(field) = self.fields(scope).next() {
            if let Some(constant) = field.constant(scope) {
                return constant.ty(scope);
            } else {
                return field.ty(scope, Some(*self));
            }
        }

        unimplemented!();
    }
    pub fn stdcall(&self, scope:&Scope) -> usize {
        if self.kind(scope) == TypeDefKind::Struct {
            if self.flags(scope).union() {
                self.fields(scope).map(|field| field.ty(scope, Some(*self)).stdcall(scope)).max().unwrap_or(1)
            } else {
                self.fields(scope).fold(0, |sum, field| sum + field.ty(scope, Some(*self)).stdcall(scope))
            }
        } else {
            4
        }
    }
    pub fn kind(&self, scope: &Scope) -> TypeDefKind {
        if self.flags(scope).interface() {
            TypeDefKind::Interface
        } else {
            match self.extends(scope).type_name(scope) {
                TypeName::Enum => TypeDefKind::Enum,
                TypeName::Delegate => TypeDefKind::Delegate,
                TypeName::Struct => TypeDefKind::Struct,
                _ => TypeDefKind::Class,
            }
        }
    }
}
