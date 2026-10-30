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
        self.params
            .last()
            .is_some_and(|param| param.is_retval_candidate(reader))
    }
}

impl Dependencies for Signature {
    fn combine(&self, dependencies: &mut TypeMap, reader: &Reader) {
        self.types().for_each(|ty| ty.combine(dependencies, reader));
    }
}
