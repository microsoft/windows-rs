impl ::core::default::Default for OPERATION_END_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPERATION_END_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.OperationId == other.OperationId && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for OPERATION_END_PARAMETERS {}
impl ::core::fmt::Debug for OPERATION_END_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPERATION_END_PARAMETERS").field("Version", &self.Version).field("OperationId", &self.OperationId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for OPERATION_END_PARAMETERS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPERATION_END_PARAMETERS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPERATION_END_PARAMETERS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPERATION_END_PARAMETERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPERATION_END_PARAMETERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPERATION_END_PARAMETERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for OPERATION_START_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OPERATION_START_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPERATION_START_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for OPERATION_START_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPERATION_START_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPERATION_START_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPERATION_START_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPERATION_START_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for OPERATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OPERATION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.OperationId == other.OperationId && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for OPERATION_START_PARAMETERS {}
impl ::core::fmt::Debug for OPERATION_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OPERATION_START_PARAMETERS").field("Version", &self.Version).field("OperationId", &self.OperationId).field("Flags", &self.Flags).finish()
    }
}
