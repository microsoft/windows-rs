#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct TypeName {
    pub namespace: String,
    pub name: String,
}

impl TypeName {
    pub fn new(namespace: &str, name: &str) -> Self {
        Self { namespace: namespace.to_string(), name: name.to_string() }
    }

    pub fn system_value_type() -> Self {
        Self::new("System", "ValueType")
    }

    pub fn system_enum() -> Self {
        Self::new("System", "Enum")
    }

    pub fn system_delegate() -> Self {
        Self::new("System", "MulticastDelegate")
    }
}
