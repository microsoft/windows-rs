/// An error encountered while selecting, lowering, or rendering metadata.
#[derive(Debug)]
pub enum Error {
    /// The metadata could not be read.
    Metadata(windows_metadata2::Error),
    /// Two projected value types have the same full name.
    DuplicateValue(String),
    /// A projected type definition is structurally invalid.
    InvalidType {
        /// The full metadata type name.
        name: String,
        /// The violated requirement.
        message: &'static str,
    },
    /// A type shape does not yet have projection policy.
    UnsupportedType {
        /// The value type containing the unsupported shape.
        name: String,
        /// The unsupported projected shape.
        shape: String,
    },
    /// Value types form an invalid recursive value cycle.
    RecursiveValue(String),
    /// Native interfaces form an invalid inheritance cycle.
    RecursiveInterface(String),
    /// A requested Win32 item does not exist.
    MissingWin32Item {
        /// The metadata namespace.
        namespace: String,
        /// The item name.
        name: String,
    },
    /// Flat output contains the same generated name from different namespaces.
    FlatNameCollision {
        /// Colliding generated item name.
        name: String,
        /// Namespace that first contributed the name.
        first_namespace: String,
        /// Namespace that later contributed the name.
        second_namespace: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Metadata(error) => error.fmt(formatter),
            Self::DuplicateValue(name) => write!(formatter, "duplicate value type `{name}`"),
            Self::InvalidType { name, message } => {
                write!(formatter, "invalid type `{name}`: {message}")
            }
            Self::UnsupportedType { name, shape } => {
                write!(formatter, "unsupported type in `{name}`: {shape}")
            }
            Self::RecursiveValue(name) => write!(formatter, "recursive value type `{name}`"),
            Self::RecursiveInterface(name) => {
                write!(formatter, "recursive native interface `{name}`")
            }
            Self::MissingWin32Item { namespace, name } => {
                write!(
                    formatter,
                    "Win32 item `{namespace}.{name}` was not selected"
                )
            }
            Self::FlatNameCollision {
                name,
                first_namespace,
                second_namespace,
            } => write!(
                formatter,
                "flat item `{name}` is defined by both `{first_namespace}` and \
                 `{second_namespace}`"
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

impl From<windows_metadata2::Error> for Error {
    fn from(value: windows_metadata2::Error) -> Self {
        Self::Metadata(value)
    }
}
