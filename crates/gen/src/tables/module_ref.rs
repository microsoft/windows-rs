macros::table!(ModuleRef);

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 0)
    }
}

impl std::fmt::Debug for ModuleRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
