use super::*;

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct ClassLayout<'a>(pub Row<'a>);

impl<'a> ClassLayout<'a> {
    pub fn packing_size(&self) -> usize {
        self.0.usize(0)
    }
}
