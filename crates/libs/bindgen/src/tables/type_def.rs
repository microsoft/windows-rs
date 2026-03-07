use super::*;

pub trait TypeDefExt {
    fn type_name(&self) -> TypeName;
    fn generics(&self) -> Vec<Type>;
    fn nested(&self) -> Option<NestedClass>;
    fn underlying_type(&self, reader: &Reader) -> Type;
    fn invalid_values(&self) -> Vec<i64>;
    fn free_function(&self, reader: &Reader) -> Option<CppFn>;
    fn is_agile(&self) -> bool;
    fn is_async(&self) -> bool;
}

impl TypeDefExt for TypeDef {
    fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), trim_tick(self.name()))
    }

    fn generics(&self) -> Vec<Type> {
        self.generic_params().map(Type::Generic).collect()
    }

    fn nested(&self) -> Option<NestedClass> {
        self.equal_range(0, self.pos() + 1).next()
    }

    fn underlying_type(&self, reader: &Reader) -> Type {
        let field = self.fields().next().unwrap();
        if let Some(constant) = field.constant() {
            constant.constant_type(reader)
        } else {
            field.field_type(None, reader)
        }
    }

    fn invalid_values(&self) -> Vec<i64> {
        let mut values = Vec::new();
        for attribute in self.attributes() {
            if attribute.name() == "InvalidHandleValueAttribute" {
                if let Some((_, Value::I64(value))) = attribute.value().first() {
                    values.push(*value);
                }
            }
        }
        values
    }

    fn free_function(&self, reader: &Reader) -> Option<CppFn> {
        if let Some(attribute) = self.find_attribute("RAIIFreeAttribute") {
            if let Some((_, Value::Utf8(name))) = attribute.value().first() {
                if let Some(Type::CppFn(ty)) = reader.with_full_name(self.namespace(), name).next()
                {
                    return Some(ty);
                }
            }
        }
        None
    }

    fn is_agile(&self) -> bool {
        for attribute in self.attributes() {
            match attribute.name() {
                "AgileAttribute" => return true,
                "MarshalingBehaviorAttribute" => {
                    if matches!(
                        attribute.value().first(),
                        Some((_, Value::EnumValue(_, inner))) if matches!(**inner, Value::I32(2))
                    ) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        self.is_async()
    }

    fn is_async(&self) -> bool {
        matches!(
            TypeName(self.namespace(), trim_tick(self.name())),
            TypeName::IAsyncAction
                | TypeName::IAsyncActionWithProgress
                | TypeName::IAsyncOperation
                | TypeName::IAsyncOperationWithProgress
        )
    }
}
