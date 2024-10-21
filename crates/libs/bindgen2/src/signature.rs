use super::*;

#[derive(Debug)]
pub struct Signature {
    pub call_flags: MethodCallAttributes,
    pub return_type: (Type, Option<Param>),
    pub params: Vec<(Type, Param)>,
}

impl Signature {
    // pub fn size(&self) -> usize {
    //     self.params
    //         .iter()
    //         .fold(0, |sum, param| sum + std::cmp::max(4, param.0.size()))
    // }

    pub fn dependencies(&self, dependencies: &mut Dependencies) {
        self.return_type.0.dependencies(dependencies);
        self.params
            .iter()
            .for_each(|(ty, _)| ty.dependencies(dependencies));
    }
}

// TODO: put WrapperSignature here? better name...

// Maybe just call it a CppSignature if its only useful to non-WinRT functions/methods
