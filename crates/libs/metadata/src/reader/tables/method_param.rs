use super::*;

/// Direction flags stored on an ECMA-335 `Param` row.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParamDirection {
    Unspecified,
    Input,
    Output,
    InputOutput,
}

/// A raw buffer-size relationship stored on a parameter attribute.
///
/// Values remain signed because validation against a method signature is projection policy.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BufferRelationship {
    ElementsParam(i16),
    BytesParam(i16),
    ElementsConst(i32),
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

    /// Returns the raw count or byte-size relationship encoded by Win32 metadata attributes.
    ///
    /// This only decodes the attribute. Consumers remain responsible for validating signed values,
    /// parameter positions, element sizes, and whether a public slice or span is appropriate.
    pub fn buffer_relationship(&self) -> Option<BufferRelationship> {
        let mut result = None;

        for attribute in self.attributes() {
            if attribute.namespace() == "Windows.Win32.Metadata"
                && attribute.name() == "BufferExtentAttribute"
            {
                let extent = BufferExtent::from_values(&attribute.value())?;
                if extent.phase != BufferPhase::Entry || extent.indirect {
                    continue;
                }
                let relationship = match (extent.units, extent.count) {
                    (BufferUnits::Elements, BufferCount::Parameter(index)) => {
                        BufferRelationship::ElementsParam(index.try_into().ok()?)
                    }
                    (BufferUnits::Bytes, BufferCount::Parameter(index)) => {
                        BufferRelationship::BytesParam(index.try_into().ok()?)
                    }
                    (BufferUnits::Elements, BufferCount::Constant(value)) => {
                        BufferRelationship::ElementsConst(value.try_into().ok()?)
                    }
                    _ => return None,
                };
                if extent.offset != 0 || extent.null_fallback.is_some() || extent.count_on_entry {
                    return None;
                }
                if result.is_some_and(|previous| previous != relationship) {
                    return None;
                }
                result = Some(relationship);
                continue;
            }
            for (name, value) in attribute.value() {
                let relationship = match (attribute.name(), name.as_str(), value) {
                    ("NativeArrayInfoAttribute", "CountParamIndex", Value::I16(value)) => {
                        BufferRelationship::ElementsParam(value)
                    }
                    ("NativeArrayInfoAttribute", "CountConst", Value::I32(value)) => {
                        BufferRelationship::ElementsConst(value)
                    }
                    ("MemorySizeAttribute", "BytesParamIndex", Value::I16(value)) => {
                        BufferRelationship::BytesParam(value)
                    }
                    ("NativeArrayInfoAttribute", "CountParamIndex" | "CountConst", _)
                    | ("MemorySizeAttribute", "BytesParamIndex", _) => return None,
                    _ => continue,
                };

                if result.replace(relationship).is_some() {
                    return None;
                }
            }
        }

        result
    }

    pub fn sequence(&self) -> u16 {
        self.usize(1).try_into().unwrap()
    }

    pub fn name(&self) -> &str {
        self.str(2)
    }
}
