use super::*;

impl std::fmt::Debug for TypeDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeDef({}.{})", self.namespace(), self.name())
    }
}

impl TypeDef {
    pub fn flags(&self) -> TypeAttributes {
        TypeAttributes(self.usize(0) as u32)
    }

    pub fn name(&self) -> &'static str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &'static str {
        self.str(2)
    }

    pub fn extends(&self) -> Option<TypeDefOrRef> {
        let extends = self.usize(3);

        if extends == 0 {
            return None;
        }

        Some(TypeDefOrRef::decode(self.file(), extends))
    }

    pub fn methods(&self) -> RowIterator<MethodDef> {
        self.list(5)
    }

    pub fn fields(&self) -> RowIterator<Field> {
        self.list(4)
    }

    pub fn generics(&self) -> Vec<Type> {
        self.file()
            .equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
            .map(|generic: GenericParam| Type::Param(generic.name()))
            .collect()
    }

    pub fn interface_impls(&self) -> RowIterator<InterfaceImpl> {
        self.file().equal_range(0, self.index() + 1)
    }

    pub fn enclosing_type(&self) -> Option<TypeDef> {
        self.file()
            .equal_range::<NestedClass>(0, self.index() + 1)
            .next()
            .map(|row| TypeDef(row.row(1)))
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.file().equal_range(2, self.index() + 1).next()
    }

    pub fn underlying_type(&self) -> Type {
        let field = self
            .fields()
            .next()
            .expect("windows-bindgen: field not found");
        if let Some(constant) = field.constant() {
            constant.ty()
        } else {
            field.ty(None)
        }
    }

    // pub fn size(&self) -> usize {
    //     match self.kind() {
    //         TypeKind::Struct => {
    //             if self.flags().contains(TypeAttributes::ExplicitLayout) {
    //                 self.fields()
    //                     .map(|field| field.ty(Some(*self)).size())
    //                     .max()
    //                     .unwrap_or(1)
    //             } else {
    //                 let mut sum = 0;
    //                 for field in self.fields() {
    //                     let ty = field.ty(Some(*self));
    //                     let size = ty.size();
    //                     let align = ty.align();
    //                     sum = (sum + (align - 1)) & !(align - 1);
    //                     sum += size;
    //                 }
    //                 sum
    //             }
    //         }
    //         TypeKind::Enum => self.underlying_type().size(),
    //         _ => 4,
    //     }
    // }

    // pub fn align(&self) -> usize {
    //     match self.kind() {
    //         TypeKind::Struct => self
    //             .fields()
    //             .map(|field| field.ty(Some(*self)).align())
    //             .max()
    //             .unwrap_or(1),
    //         TypeKind::Enum => self.underlying_type().align(),
    //         _ => 4,
    //     }
    // }
}
