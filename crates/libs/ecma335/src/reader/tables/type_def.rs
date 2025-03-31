use super::*;

impl std::fmt::Debug for TypeDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeDef({}.{})", self.namespace(), self.name())
    }
}

impl<'a> TypeDef<'a> {
    pub fn flags(&self) -> TypeAttributes {
        TypeAttributes(self.usize(0).try_into().unwrap())
    }

    pub fn name(&self) -> &'a str {
        self.str(1)
    }

    pub fn namespace(&self) -> &'a str {
        self.str(2)
    }

    pub fn extends(&self) -> Option<TypeDefOrRef> {
        if self.usize(3) == 0 {
            return None;
        }

        Some(self.decode(3))
    }

    pub fn fields(&self) -> RowIterator<Field> {
        self.list(4)
    }

    pub fn methods(&self) -> RowIterator<MethodDef> {
        self.list(5)
    }

    pub fn generic_params(&self) -> RowIterator<GenericParam> {
        self.equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub fn interface_impls(&self) -> RowIterator<InterfaceImpl> {
        self.equal_range(0, self.pos() + 1)
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.equal_range(2, self.pos() + 1).next()
    }

    pub fn category(&self) -> TypeCategory {
        if let Some(extends) = self.extends() {
            if extends.namespace() == "System" {
                match extends.name() {
                    "Enum" => TypeCategory::Enum,
                    "MulticastDelegate" => TypeCategory::Delegate,
                    "ValueType" => TypeCategory::Struct,
                    "Attribute" => TypeCategory::Attribute,
                    _ => TypeCategory::Class,
                }
            } else {
                TypeCategory::Class
            }
        } else {
            TypeCategory::Interface
        }
    }
}
