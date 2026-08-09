use windows_metadata2::BuildError;

/// An error encountered while compiling an RDL document.
#[derive(Debug)]
pub enum Error {
    /// Metadata construction failed.
    Metadata(BuildError),
    /// Two source definitions have the same namespace and name.
    DuplicateType { namespace: String, name: String },
    /// An enum uses a non-integer storage type.
    InvalidEnumUnderlying { namespace: String, name: String },
    /// An enum value does not match the enum storage type.
    EnumValueMismatch {
        namespace: String,
        name: String,
        variant: String,
    },
    /// A field refers to a type that is not declared in the document.
    UndefinedType {
        namespace: String,
        name: String,
        field: String,
        target_namespace: String,
        target_name: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metadata(error) => error.fmt(formatter),
            Self::DuplicateType { namespace, name } => {
                write!(formatter, "duplicate source type `{namespace}.{name}`")
            }
            Self::InvalidEnumUnderlying { namespace, name } => {
                write!(
                    formatter,
                    "enum `{namespace}.{name}` has a non-integer underlying type"
                )
            }
            Self::EnumValueMismatch {
                namespace,
                name,
                variant,
            } => write!(
                formatter,
                "enum value `{namespace}.{name}.{variant}` does not match its underlying type"
            ),
            Self::UndefinedType {
                namespace,
                name,
                field,
                target_namespace,
                target_name,
            } => write!(
                formatter,
                "field `{namespace}.{name}.{field}` refers to undefined type \
                 `{target_namespace}.{target_name}`"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Metadata(error) => Some(error),
            _ => None,
        }
    }
}

impl From<BuildError> for Error {
    fn from(error: BuildError) -> Self {
        Self::Metadata(error)
    }
}
