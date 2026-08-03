use super::*;

/// Direction flags stored on an ECMA-335 `Param` row.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParamDirection {
    Unspecified,
    Input,
    Output,
    InputOutput,
}

impl std::fmt::Debug for MethodParam<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("MethodParam").field(&self.name()).finish()
    }
}

impl MethodParam<'_> {
    pub fn flags(&self) -> ParamAttributes {
        ParamAttributes(self.usize(0).try_into().unwrap())
    }

    /// Returns the direction represented by the `In` and `Out` flags without applying type or
    /// projection defaults.
    pub fn direction(&self) -> ParamDirection {
        let flags = self.flags();
        match (
            flags.contains(ParamAttributes::In),
            flags.contains(ParamAttributes::Out),
        ) {
            (false, false) => ParamDirection::Unspecified,
            (true, false) => ParamDirection::Input,
            (false, true) => ParamDirection::Output,
            (true, true) => ParamDirection::InputOutput,
        }
    }

    /// Returns whether the ECMA-335 `Optional` flag is present.
    pub fn is_optional(&self) -> bool {
        self.flags().contains(ParamAttributes::Optional)
    }

    /// Returns whether `ReservedAttribute` is present.
    pub fn is_reserved(&self) -> bool {
        self.has_attribute("ReservedAttribute")
    }

    /// Returns whether `RetValAttribute` is present.
    pub fn is_retval_attribute(&self) -> bool {
        self.has_attribute("RetValAttribute")
    }

    pub fn sequence(&self) -> u16 {
        self.usize(1).try_into().unwrap()
    }

    pub fn name(&self) -> &str {
        self.str(2)
    }
}
