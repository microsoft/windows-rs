macros::table!(GenericParam);

impl GenericParam {
    pub fn name(&self) -> &'static str {
        self.reader.str(self.row, 3)
    }
}
