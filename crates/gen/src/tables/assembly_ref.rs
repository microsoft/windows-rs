macros::table!(AssemblyRef);

impl std::fmt::Debug for AssemblyRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AssemblyRef")
            .field("row", &self.row)
            .finish()
    }
}
