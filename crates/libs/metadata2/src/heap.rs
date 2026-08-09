/// An offset into the `#Strings` heap.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StringId(u32);

impl StringId {
    /// Creates a string heap identifier from its encoded offset.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the encoded heap offset.
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// An offset into the `#Blob` heap.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BlobId(u32);

impl BlobId {
    /// Creates a blob heap identifier from its encoded offset.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the encoded heap offset.
    pub const fn value(self) -> u32 {
        self.0
    }
}

/// A one-based index into the `#GUID` heap, or zero for no GUID.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct GuidId(u32);

impl GuidId {
    /// Creates a GUID heap identifier from its encoded index.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the encoded heap index.
    pub const fn value(self) -> u32 {
        self.0
    }
}
