impl ::core::default::Default for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_ALPHA_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ALPHA_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_BEZIER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2 && self.point3 == other.point3
    }
}
impl ::core::cmp::Eq for D2D1_BEZIER_SEGMENT {}
impl ::core::fmt::Debug for D2D1_BEZIER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BEZIER_SEGMENT").field("point1", &self.point1).field("point2", &self.point2).field("point3", &self.point3).finish()
    }
}
impl ::core::default::Default for D2D1_BLEND_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BLEND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_BORDER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_BORDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLORMATRIX_ALPHA_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COLORMATRIX_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMATRIX_ALPHA_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D1_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D2D1_COLOR_F {}
impl ::core::fmt::Debug for D2D1_COLOR_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::core::default::Default for D2D1_COMPOSITE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_COMPOSITE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COMPOSITE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FIGURE_BEGIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FIGURE_BEGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FIGURE_BEGIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FIGURE_END {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FIGURE_END {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FIGURE_END").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D1_PATH_SEGMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_PATH_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PATH_SEGMENT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_PATH_SEGMENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_PATH_SEGMENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D2D1_PIXEL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D2D1_PIXEL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format && self.alphaMode == other.alphaMode
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D2D1_PIXEL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_PIXEL_FORMAT").field("format", &self.format).field("alphaMode", &self.alphaMode).finish()
    }
}
impl ::core::default::Default for D2D1_TURBULENCE_NOISE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D2D1_TURBULENCE_NOISE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TURBULENCE_NOISE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D2D_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D2D_COLOR_F {}
impl ::core::fmt::Debug for D2D_COLOR_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::core::default::Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D2D_POINT_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_POINT_2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_POINT_2F {}
impl ::core::fmt::Debug for D2D_POINT_2F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_POINT_2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for D2D_POINT_2U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_POINT_2U {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_POINT_2U {}
impl ::core::fmt::Debug for D2D_POINT_2U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_POINT_2U").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for D2D_RECT_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_RECT_F {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for D2D_RECT_F {}
impl ::core::fmt::Debug for D2D_RECT_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_RECT_F").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::default::Default for D2D_RECT_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_RECT_U {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for D2D_RECT_U {}
impl ::core::fmt::Debug for D2D_RECT_U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_RECT_U").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::core::default::Default for D2D_SIZE_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_SIZE_F {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D_SIZE_F {}
impl ::core::fmt::Debug for D2D_SIZE_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_SIZE_F").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::default::Default for D2D_SIZE_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_SIZE_U {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D_SIZE_U {}
impl ::core::fmt::Debug for D2D_SIZE_U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_SIZE_U").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::default::Default for D2D_VECTOR_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_VECTOR_2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_2F {}
impl ::core::fmt::Debug for D2D_VECTOR_2F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_VECTOR_2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for D2D_VECTOR_3F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_VECTOR_3F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_3F {}
impl ::core::fmt::Debug for D2D_VECTOR_3F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_VECTOR_3F").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::core::default::Default for D2D_VECTOR_4F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D2D_VECTOR_4F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_4F {}
impl ::core::fmt::Debug for D2D_VECTOR_4F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_VECTOR_4F").field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w).finish()
    }
}
impl ::core::cmp::PartialEq for ID2D1SimplifiedGeometrySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID2D1SimplifiedGeometrySink {}
impl ::core::fmt::Debug for ID2D1SimplifiedGeometrySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID2D1SimplifiedGeometrySink").field(&self.0).finish()
    }
}
