use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub call_flags: MethodCallAttributes,
    pub return_type: (Type, Option<Param>),
    pub params: Vec<(Type, Param)>,
}

impl Signature {
    pub fn size(&self) -> usize {
        self.params
            .iter()
            .fold(0, |sum, param| sum + std::cmp::max(4, param.0.size()))
    }

    pub fn dependencies(&self, dependencies: &mut TypeMap) {
        self.return_type.0.dependencies(dependencies);
        self.params
            .iter()
            .for_each(|(ty, _)| ty.dependencies(dependencies));
    }

    pub fn types(&self) -> impl Iterator<Item = &Type> + '_ {
        std::iter::once(&self.return_type.0)
            .chain(self.params.iter().map(|(ty, _)| ty))
            .map(|ty| ty.decay())
    }
}
