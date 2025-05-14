use super::*;

impl std::fmt::Debug for Field<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Field").field(&self.name()).finish()
    }
}

impl Field<'_> {
    pub fn flags(&self) -> FieldAttributes {
        FieldAttributes(self.usize(0).try_into().unwrap())
    }

    pub fn name(&self) -> &str {
        self.str(1)
    }

    pub fn ty(&self) -> Type {
        let mut blob = self.blob(2);
        let prolog = blob.read_u8();
        debug_assert_eq!(prolog, 0x6);

        blob.read_type_signature(&[])
    }

    pub fn constant(&self) -> Option<Constant> {
        self.equal_range(1, HasConstant::Field(*self).encode())
            .next()
    }
}
