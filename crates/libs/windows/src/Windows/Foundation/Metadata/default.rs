impl ::core::default::Default for AttributeTargets {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AttributeTargets {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AttributeTargets").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AttributeTargets {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AttributeTargets {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AttributeTargets {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AttributeTargets {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AttributeTargets {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CompositionType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CompositionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeprecationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeprecationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeprecationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for FeatureStage {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FeatureStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FeatureStage").field(&self.0).finish()
    }
}
impl ::core::default::Default for GCPressureAmount {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GCPressureAmount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GCPressureAmount").field(&self.0).finish()
    }
}
impl ::core::default::Default for MarshalingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MarshalingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MarshalingType").field(&self.0).finish()
    }
}
impl ::core::default::Default for Platform {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Platform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Platform").field(&self.0).finish()
    }
}
impl ::core::default::Default for ThreadingModel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ThreadingModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThreadingModel").field(&self.0).finish()
    }
}
