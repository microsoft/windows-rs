use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature {
    pub call_flags: MethodCallAttributes,
    pub return_type: Type,
    pub params: Vec<Param>,
}

impl Signature {
    pub fn size(&self) -> usize {
        self.params
            .iter()
            .fold(0, |sum, param| sum + std::cmp::max(4, param.size()))
    }

    pub fn dependencies(&self, dependencies: &mut TypeMap) {
        self.types().for_each(|ty| ty.dependencies(dependencies));
    }

    pub fn types(&self) -> impl Iterator<Item = &Type> + '_ {
        std::iter::once(&self.return_type)
            .chain(self.params.iter().map(|param| &param.ty))
            .map(|ty| ty.decay())
    }

    pub fn is_retval(&self) -> bool {
        // First we check whether there's an actual retval parameter.
        if let Some(param) = self.params.last() {
            if param.def.has_attribute("RetValAttribute") {
                return true;
            }
        }

        if let Some(param) = self.params.last() {
            if param.is_retval() {
                return self.params[..self.params.len() - 1]
                    .iter()
                    .all(|param| param.is_input());
            }
        }

        false
    }
}
