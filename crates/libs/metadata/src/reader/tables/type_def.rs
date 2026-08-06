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

    pub fn property_map(&self) -> Option<PropertyMap<'a>> {
        self.equal_range(0, self.pos() + 1).next()
    }

    pub fn properties(&self) -> impl Iterator<Item = Property<'a>> {
        self.property_map()
            .into_iter()
            .flat_map(|map| map.properties())
    }

    pub fn event_map(&self) -> Option<EventMap<'a>> {
        self.equal_range(0, self.pos() + 1).next()
    }

    pub fn events(&self) -> impl Iterator<Item = Event<'a>> {
        self.event_map().into_iter().flat_map(|map| map.events())
    }

    pub fn generic_params(&self) -> RowIterator<'a, GenericParam<'a>> {
        self.equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub fn interface_impls(&self) -> RowIterator<'a, InterfaceImpl<'a>> {
        self.equal_range(0, self.pos() + 1)
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
