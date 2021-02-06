macros::table!(ModuleRef);

impl ModuleRef {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 0)
    }
}
