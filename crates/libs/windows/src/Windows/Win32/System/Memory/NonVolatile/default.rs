impl ::core::default::Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NV_MEMORY_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.BaseAddress == other.BaseAddress && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for NV_MEMORY_RANGE {}
impl ::core::fmt::Debug for NV_MEMORY_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NV_MEMORY_RANGE").field("BaseAddress", &self.BaseAddress).field("Length", &self.Length).finish()
    }
}
