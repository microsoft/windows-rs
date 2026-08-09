use std::fmt::{Display, Formatter};

/// An error encountered while reading an ECMA-335 metadata image.
#[derive(Debug)]
pub enum Error {
    /// The input could not be read.
    Io(std::io::Error),
    /// The input is structurally invalid at the given byte offset.
    Invalid {
        /// The byte offset at which validation failed.
        offset: usize,
        /// A short description of the violated structural rule.
        message: &'static str,
    },
    /// The metadata violates a rule that is not tied to one encoded byte.
    InvalidMetadata {
        /// A short description of the violated rule.
        message: &'static str,
    },
    /// A required metadata stream is absent.
    MissingStream(&'static str),
    /// A metadata stream name appears more than once.
    DuplicateStream(String),
    /// A structurally known table is not supported by the current reader layer.
    UnsupportedTable {
        /// The ECMA table name.
        table: &'static str,
        /// The number of rows present.
        rows: u32,
    },
    /// A row-owned value failed structural decoding.
    Row {
        /// The ECMA table name.
        table: &'static str,
        /// The one-based row number.
        row: u32,
        /// The underlying decoding error.
        source: Box<Self>,
    },
    /// A referenced type definition is not present in the database.
    UnresolvedType {
        /// Type namespace.
        namespace: String,
        /// Metadata type name.
        name: String,
    },
    /// Two Param rows use the same sequence within one method.
    DuplicateParameterSequence {
        /// The duplicated sequence.
        sequence: u16,
    },
    /// A Param row names a signature position that does not exist.
    ParameterSequenceOutOfRange {
        /// The encoded one-based sequence.
        sequence: u16,
        /// Number of parameters in the method signature.
        parameter_count: usize,
    },
}

impl Error {
    pub(crate) fn invalid(offset: usize, message: &'static str) -> Self {
        Self::Invalid { offset, message }
    }

    pub(crate) fn invalid_metadata(message: &'static str) -> Self {
        Self::InvalidMetadata { message }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),
            Self::Invalid { offset, message } => {
                write!(formatter, "invalid metadata at byte {offset}: {message}")
            }
            Self::InvalidMetadata { message } => write!(formatter, "invalid metadata: {message}"),
            Self::MissingStream(name) => write!(formatter, "missing metadata stream `{name}`"),
            Self::DuplicateStream(name) => {
                write!(formatter, "duplicate metadata stream `{name}`")
            }
            Self::UnsupportedTable { table, rows } => {
                write!(
                    formatter,
                    "unsupported metadata table `{table}` has {rows} rows"
                )
            }
            Self::Row { table, row, source } => {
                write!(formatter, "{table} row {row}: {source}")
            }
            Self::UnresolvedType { namespace, name } => {
                write!(formatter, "unresolved metadata type `{namespace}.{name}`")
            }
            Self::DuplicateParameterSequence { sequence } => {
                write!(formatter, "duplicate Param.Sequence {sequence}")
            }
            Self::ParameterSequenceOutOfRange {
                sequence,
                parameter_count,
            } => write!(
                formatter,
                "Param.Sequence {sequence} is out of range for {parameter_count} signature \
                 parameters"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Row { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
