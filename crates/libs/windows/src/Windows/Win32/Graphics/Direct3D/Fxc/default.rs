impl ::core::default::Default for D3DCOMPILER_STRIP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3DCOMPILER_STRIP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3DCOMPILER_STRIP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_BLOB_PART {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_BLOB_PART {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_BLOB_PART").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D_SHADER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pBytecode == other.pBytecode && self.BytecodeLength == other.BytecodeLength
    }
}
impl ::core::cmp::Eq for D3D_SHADER_DATA {}
impl ::core::fmt::Debug for D3D_SHADER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D_SHADER_DATA").field("pBytecode", &self.pBytecode).field("BytecodeLength", &self.BytecodeLength).finish()
    }
}
