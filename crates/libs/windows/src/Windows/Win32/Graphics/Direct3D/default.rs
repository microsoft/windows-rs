impl ::core::default::Default for D3DVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3DVECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for D3DVECTOR {}
impl ::core::fmt::Debug for D3DVECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3DVECTOR").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::core::default::Default for D3D_CBUFFER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_CBUFFER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_CBUFFER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_DRIVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_DRIVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_DRIVER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_FEATURE_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_INCLUDE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_INCLUDE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_INCLUDE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_MIN_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_MIN_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_MIN_PRECISION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_NAME").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_PARAMETER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_PARAMETER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_PARAMETER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_PRIMITIVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_PRIMITIVE_TOPOLOGY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_PRIMITIVE_TOPOLOGY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_PRIMITIVE_TOPOLOGY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_REGISTER_COMPONENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_REGISTER_COMPONENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_REGISTER_COMPONENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_RESOURCE_RETURN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_RESOURCE_RETURN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_RESOURCE_RETURN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_CBUFFER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_CBUFFER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_CBUFFER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_INPUT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_INPUT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_INPUT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_INPUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_INPUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_INPUT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_MACRO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D_SHADER_MACRO {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Definition == other.Definition
    }
}
impl ::core::cmp::Eq for D3D_SHADER_MACRO {}
impl ::core::fmt::Debug for D3D_SHADER_MACRO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D_SHADER_MACRO").field("Name", &self.Name).field("Definition", &self.Definition).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_VARIABLE_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_VARIABLE_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_VARIABLE_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_VARIABLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_VARIABLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_VARIABLE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_VARIABLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_VARIABLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_VARIABLE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SRV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SRV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SRV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_TESSELLATOR_DOMAIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_TESSELLATOR_DOMAIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_TESSELLATOR_DOMAIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_TESSELLATOR_OUTPUT_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_TESSELLATOR_OUTPUT_PRIMITIVE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_TESSELLATOR_PARTITIONING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_TESSELLATOR_PARTITIONING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_TESSELLATOR_PARTITIONING").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3DBlob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DBlob {}
impl ::core::fmt::Debug for ID3DBlob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DBlob").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3DDestructionNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DDestructionNotifier {}
impl ::core::fmt::Debug for ID3DDestructionNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DDestructionNotifier").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3DInclude {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3DInclude {}
impl ::core::fmt::Debug for ID3DInclude {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3DInclude").field(&self.0).finish()
    }
}
