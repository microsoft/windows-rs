use super::*;

#[derive(Clone)]
pub struct Row<'a> {
    pub scope: &'a Scope<'a>,
    pub key: ScopeKey,
}

impl<'a> Row<'a> {
    pub fn new(scope: &'a Scope, key: ScopeKey) -> Self {
        Self { scope, key }
    }

    pub fn usize(&self, column: usize) -> usize {
        self.scope.usize(&self.key, column)
    }

    pub fn str(&self, column: usize) -> &str {
        self.scope.str(&self.key, column)
    }

    pub fn decode<T: Decode<'a>>(&self, column: usize) -> T {
        self.scope.decode(&self.key, column)
    }
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
