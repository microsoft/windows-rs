use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub call_flags: MethodCallAttributes,
    pub return_type: Type,
    pub params: Vec<Param>,
}

impl Signature {
    pub fn types(&self) -> impl Iterator<Item = &Type> + '_ {
        std::iter::once(&self.return_type)
            .chain(self.params.iter().map(|param| &param.ty))
            .map(|ty| ty.decay())
    }

    pub fn is_retval(&self, reader: &Reader) -> bool {
        // First we check whether there's an explicit retval parameter.
        if let Some(param) = self.params.last()
            && param.is_retval_attribute()
        {
            return param.is_explicit_retval_candidate(reader);
        }

        // Otherwise we check heuristically for additional candidates.
        if let Some(param) = self.params.last()
            && param.is_heuristic_retval_candidate(reader)
        {
            return self.params[..self.params.len() - 1]
                .iter()
                .all(Param::is_input_only);
        }

        false
    }
}

impl Dependencies for Signature {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        self.types().for_each(|ty| ty.combine(dependencies, reader));
    }
}
