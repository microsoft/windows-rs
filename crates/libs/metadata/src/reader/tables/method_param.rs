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

/// Valid byte extent on successful return for a non-null buffer, not its capacity.
///
/// With `dereference`, `parameter` names an output integer pointer. Otherwise it names
/// a by-value integer. Consumers must validate the signature and the API's success contract.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BytesWritten {
    pub parameter: i16,
    pub dereference: bool,
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

    /// Decodes a byte-buffer postcondition separately from its capacity relationship.
    pub fn bytes_written(&self) -> Option<BytesWritten> {
        let mut attributes = self.attributes().filter(|attribute| {
            attribute.name() == "MemoryWrittenAttribute"
                && attribute.ctor().parent().namespace() == "Windows.Win32.Metadata"
        });
        let attribute = attributes.next()?;
        if attributes.next().is_some() {
            return None;
        }
        let mut parameter = None;
        let mut dereference = None;
        for (name, value) in attribute.value() {
            match (name.as_str(), value) {
                ("BytesParamIndex", Value::I16(value)) if parameter.is_none() => {
                    parameter = Some(value);
                }
                ("Dereference", Value::Bool(value)) if dereference.is_none() => {
                    dereference = Some(value);
                }
                _ => return None,
            }
        }
        Some(BytesWritten {
            parameter: parameter?,
            dereference: dereference?,
        })
    }

    pub fn sequence(&self) -> u16 {
        self.usize(1).try_into().unwrap()
    }

    pub fn name(&self) -> &str {
        self.str(2)
    }
}
