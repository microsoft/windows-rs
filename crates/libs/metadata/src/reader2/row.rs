use super::*;

#[derive(Clone)]
pub struct Row<'a> {
    scope: &'a Scope<'a>,
    id: RowId,
}

impl<'a> Row<'a> {
    pub fn usize(&self, column: usize) -> usize {
        self.scope.usize(&self.id, column)
    }

    pub fn str(&self, column: usize) -> &str {
        self.scope.str(&self.id, column)
    }
}



impl<'a> PartialEq for Row<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<'a> PartialOrd for Row<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<'a> Ord for Row<'a> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'a> Eq for Row<'a> {}
