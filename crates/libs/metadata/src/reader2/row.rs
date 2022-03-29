use super::*;

#[derive(Clone)]
pub struct Row<'a> {
    scope: &'a Scope<'a>,
    key: Key,
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Key {
    pub row: u32,
    pub table: u16,
    pub file: u16,
}

impl<'a> PartialEq for Row<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.key.eq(&other.key)
    }
}

impl<'a> PartialOrd for Row<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl<'a> Ord for Row<'a> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.key.cmp(&other.key)
    }
}

impl<'a> Eq for Row<'a> {}
