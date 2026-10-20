use super::*;

impl std::fmt::Debug for TypeDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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

    /// Returns the outer namespace and slash-separated enclosing type names.
    pub fn full_name(&self) -> TypeName {
        if let Some(nested) = self.equal_range::<NestedClass>(0, self.pos() + 1).next() {
            let mut name = nested.outer().full_name();
            name.name.push('/');
            name.name.push_str(self.name());
            name
        } else {
            TypeName::named(self.namespace(), self.name())
        }
    }

    pub fn extends(&self) -> Option<TypeDefOrRef<'a>> {
        if self.usize(3) == 0 {
            return None;
        }

        Some(self.decode(3))
    }

    pub fn fields(&self) -> RowIterator<'a, Field<'a>> {
        self.list(4)
    }

    pub fn methods(&self) -> RowIterator<'a, MethodDef<'a>> {
        self.list(5)
    }

    pub fn generic_params(&self) -> RowIterator<'a, GenericParam<'a>> {
        self.equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub fn interface_impls(&self) -> RowIterator<'a, InterfaceImpl<'a>> {
        self.equal_range(0, self.pos() + 1)
    }

    pub fn properties(&self) -> RowIterator<'a, Property<'a>> {
        // PropertyMap is not a sorted table, so find the type's row by scanning.
        self.map_rows::<PropertyMap>()
            .find(|map| map.parent() == *self)
            .map_or_else(|| self.empty_rows(), |map| map.properties())
    }

    pub fn events(&self) -> RowIterator<'a, Event<'a>> {
        // EventMap is not a sorted table, so find the type's row by scanning.
        self.map_rows::<EventMap>()
            .find(|map| map.parent() == *self)
            .map_or_else(|| self.empty_rows(), |map| map.events())
    }

    fn map_rows<R: AsRow<'a>>(&self) -> RowIterator<'a, R> {
        let row = self.to_row();
        RowIterator::new(row.index, row.file, self.file().rows(R::TABLE))
    }

    fn empty_rows<R: AsRow<'a>>(&self) -> RowIterator<'a, R> {
        let row = self.to_row();
        RowIterator::new(row.index, row.file, 0..0)
    }

    pub fn class_layout(&self) -> Option<ClassLayout<'a>> {
        self.equal_range(2, self.pos() + 1).next()
    }

    pub fn underlying_type(&self) -> Option<Type> {
        // An enum's backing integer is its sole non-literal (instance) field; the members
        // are static literal fields. A constant typed as the enum encodes against that
        // integer, so resolve it here even when the enum carries members.
        if self.category() == TypeCategory::Enum {
            return self
                .fields()
                .find(|field| field.constant().is_none())
                .map(|field| field.ty());
        }

        let mut fields = self.fields();

        if fields.len() == 1 {
            let field = fields.next().unwrap();
            if let Some(constant) = field.constant() {
                Some(constant.ty())
            } else {
                Some(field.ty())
            }
        } else {
            None
        }
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
