use super::*;

impl std::fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Attribute")
            .field(&self.ty().parent().name())
            .finish()
    }
}

impl Attribute<'_> {
    pub fn parent(&self) -> HasAttribute {
        self.decode(0)
    }

    pub fn ty(&self) -> AttributeType {
        self.decode(1)
    }

    pub fn signature(&self) -> Blob {
        self.blob(2)
    }
}
