/// An error encountered while building a metadata image.
#[derive(Debug)]
pub enum BuildError {
    /// A builder operation violates the metadata construction state.
    Invalid {
        /// The violated builder rule.
        message: &'static str,
    },
    /// The bounded writer cannot encode a resource at its current index width.
    LimitExceeded {
        /// The resource that exceeded the current writer limit.
        resource: &'static str,
    },
    /// Metadata container size arithmetic overflowed.
    Overflow {
        /// The value being calculated.
        resource: &'static str,
    },
}

impl BuildError {
    pub(crate) const fn invalid(message: &'static str) -> Self {
        Self::Invalid { message }
    }

    pub(crate) const fn limit(resource: &'static str) -> Self {
        Self::LimitExceeded { resource }
    }

    pub(crate) const fn overflow(resource: &'static str) -> Self {
        Self::Overflow { resource }
    }
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid { message } => formatter.write_str(message),
            Self::LimitExceeded { resource } => {
                write!(
                    formatter,
                    "initial metadata builder {resource} limit exceeded"
                )
            }
            Self::Overflow { resource } => write!(formatter, "{resource} overflow"),
        }
    }
}

impl std::error::Error for BuildError {}
