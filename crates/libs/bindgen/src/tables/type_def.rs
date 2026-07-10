use super::*;

pub trait TypeDefExt {
    fn type_name(&self) -> TypeName;
    fn generics(&self) -> Vec<Type>;
    fn underlying_type_ext(&self, reader: &Reader) -> Type;
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

    fn underlying_type_ext(&self, reader: &Reader) -> Type {
        let field = self.fields().next().unwrap();
        if let Some(constant) = field.constant() {
            constant.constant_type(reader)
        } else {
            field.field_type(None, reader)
        }
    }

    fn is_agile(&self) -> bool {
        for attribute in self.attributes() {
            if attribute.name() == "MarshalingBehaviorAttribute"
                && matches!(
                    attribute.value().first(),
                    Some((_, Value::EnumValue(_, inner))) if matches!(**inner, Value::I32(2))
                )
            {
                return true;
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
