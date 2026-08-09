/// Encoded ECMA TypeAttributes bits.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TypeAttributes(u32);

impl TypeAttributes {
    pub const PUBLIC: Self = Self(0x0000_0001);
    pub const NESTED_PUBLIC: Self = Self(0x0000_0002);
    pub const SEQUENTIAL_LAYOUT: Self = Self(0x0000_0008);
    pub const EXPLICIT_LAYOUT: Self = Self(0x0000_0010);
    pub const SEALED: Self = Self(0x0000_0100);
    pub const WINDOWS_RUNTIME: Self = Self(0x0000_4000);

    /// Creates attributes from their encoded bits.
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Returns the encoded bits.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Returns whether all requested attributes are present.
    pub const fn contains(self, value: Self) -> bool {
        self.0 & value.0 == value.0
    }
}

impl std::ops::BitOr for TypeAttributes {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOrAssign for TypeAttributes {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}

/// Encoded ECMA FieldAttributes bits.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FieldAttributes(u16);

impl FieldAttributes {
    pub const PRIVATE: Self = Self(0x0001);
    pub const PUBLIC: Self = Self(0x0006);
    pub const STATIC: Self = Self(0x0010);
    pub const LITERAL: Self = Self(0x0040);
    pub const SPECIAL_NAME: Self = Self(0x0200);
    pub const RT_SPECIAL_NAME: Self = Self(0x0400);

    /// Creates attributes from their encoded bits.
    pub const fn from_bits(bits: u16) -> Self {
        Self(bits)
    }

    /// Returns the encoded bits.
    pub const fn bits(self) -> u16 {
        self.0
    }

    /// Returns whether all requested attributes are present.
    pub const fn contains(self, value: Self) -> bool {
        self.0 & value.0 == value.0
    }
}

impl std::ops::BitOr for FieldAttributes {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}

impl std::ops::BitOrAssign for FieldAttributes {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0;
    }
}
