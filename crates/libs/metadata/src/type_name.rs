use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeName {
    /// Namespace of the outermost type.
    pub namespace: String,
    /// Type name, including slash-separated enclosing names for nested types.
    pub name: String,
    pub generics: Vec<Type>,
}

impl TypeName {
    pub fn named(namespace: &str, name: &str) -> Self {
        Self {
            namespace: namespace.to_string(),
            name: name.to_string(),
            generics: vec![],
        }
    }

    /// Resolves an intrinsic `System.Type` attribute value without a type definition.
    pub fn primitive(&self) -> Option<Type> {
        if self.namespace != "System" || !self.generics.is_empty() {
            return None;
        }
        Some(match self.name.as_str() {
            "Void" => Type::Void,
            "Boolean" => Type::Bool,
            "Char" => Type::Char,
            "SByte" => Type::I8,
            "Byte" => Type::U8,
            "Int16" => Type::I16,
            "UInt16" => Type::U16,
            "Int32" => Type::I32,
            "UInt32" => Type::U32,
            "Int64" => Type::I64,
            "UInt64" => Type::U64,
            "Single" => Type::F32,
            "Double" => Type::F64,
            "IntPtr" => Type::ISize,
            "UIntPtr" => Type::USize,
            "String" => Type::String,
            "Object" => Type::Object,
            _ => return None,
        })
    }
}

impl PartialEq<(&str, &str)> for &TypeName {
    fn eq(&self, other: &(&str, &str)) -> bool {
        self.namespace == other.0 && self.name == other.1
    }
}
