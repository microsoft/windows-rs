use super::*;

/// `Param` rows associated with a method signature by ECMA-335 `Param.Sequence`.
#[derive(Clone, Debug)]
pub struct MethodParamMap<'a> {
    return_param: Option<MethodParam<'a>>,
    params: Vec<Option<MethodParam<'a>>>,
}

impl<'a> MethodParamMap<'a> {
    /// Returns the `Sequence == 0` return row, when present.
    pub fn return_param(&self) -> Option<MethodParam<'a>> {
        self.return_param
    }

    /// Returns one optional row for each signature parameter.
    pub fn params(&self) -> &[Option<MethodParam<'a>>] {
        &self.params
    }
}

/// A malformed ECMA-335 `Param.Sequence` association.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MethodParamSequenceError {
    DuplicateSequence {
        sequence: u16,
    },
    SequenceOutOfRange {
        sequence: u16,
        parameter_count: usize,
    },
}

impl std::fmt::Display for MethodParamSequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DuplicateSequence { sequence } => {
                write!(f, "duplicate Param.Sequence {sequence}")
            }
            Self::SequenceOutOfRange {
                sequence,
                parameter_count,
            } => write!(
                f,
                "Param.Sequence {sequence} is out of range for {parameter_count} signature \
                 parameters"
            ),
        }
    }
}

impl std::error::Error for MethodParamSequenceError {}

impl std::fmt::Debug for MethodDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("MethodDef").field(&self.name()).finish()
    }
}

impl<'a> MethodDef<'a> {
    pub fn impl_flags(&self) -> MethodImplAttributes {
        MethodImplAttributes(self.usize(1).try_into().unwrap())
    }

    pub fn flags(&self) -> MethodAttributes {
        MethodAttributes(self.usize(2).try_into().unwrap())
    }

    pub fn name(&self) -> &'a str {
        self.str(3)
    }

    pub fn signature(&self, generics: &[Type]) -> Signature {
        self.blob(4).read_method_signature(generics)
    }

    pub fn generic_params(&self) -> RowIterator<'a, GenericParam<'a>> {
        self.equal_range(2, TypeOrMethodDef::MethodDef(*self).encode())
    }

    /// Iterates the method's `Param` rows in physical table order.
    ///
    /// Use [`Self::params_by_sequence`] when associating rows with signature positions.
    pub fn params(&self) -> RowIterator<'a, MethodParam<'a>> {
        self.list(5)
    }

    /// Associates `Param` rows with `parameter_count` signature positions.
    ///
    /// Sequence zero is returned separately, nonzero sequences are one-based, and missing rows
    /// remain `None`. Sparse and out-of-order rows are valid. Duplicate sequences and nonzero
    /// sequences outside the signature are reported as errors. If several rows are invalid, the
    /// first invalid row in physical table order is reported.
    pub fn params_by_sequence(
        &self,
        parameter_count: usize,
    ) -> Result<MethodParamMap<'a>, MethodParamSequenceError> {
        let mut return_param = None;
        let mut params = vec![None; parameter_count];

        for param in self.params() {
            let sequence = param.sequence();
            if sequence == 0 {
                if return_param.replace(param).is_some() {
                    return Err(MethodParamSequenceError::DuplicateSequence { sequence });
                }
                continue;
            }

            let position = sequence as usize - 1;
            let Some(slot) = params.get_mut(position) else {
                return Err(MethodParamSequenceError::SequenceOutOfRange {
                    sequence,
                    parameter_count,
                });
            };
            if slot.replace(param).is_some() {
                return Err(MethodParamSequenceError::DuplicateSequence { sequence });
            }
        }

        Ok(MethodParamMap {
            return_param,
            params,
        })
    }

    pub fn parent(&self) -> MemberRefParent<'a> {
        MemberRefParent::TypeDef(self.parent_row(5))
    }

    pub fn impl_map(&self) -> Option<ImplMap<'a>> {
        self.equal_range(1, MemberForwarded::MethodDef(*self).encode())
            .next()
    }

    pub fn calling_convention(&self) -> &'static str {
        self.impl_map().map_or("", |map| {
            let flags = map.flags();

            if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
                "system"
            } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
                "C"
            } else {
                ""
            }
        })
    }
}
