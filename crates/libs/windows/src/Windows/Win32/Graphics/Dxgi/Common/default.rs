impl ::core::default::Default for DXGI_ALPHA_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ALPHA_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_COLOR_SPACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_COLOR_SPACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_COLOR_SPACE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_GAMMA_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_GAMMA_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.Scale == other.Scale && self.Offset == other.Offset && self.GammaCurve == other.GammaCurve
    }
}
impl ::core::cmp::Eq for DXGI_GAMMA_CONTROL {}
impl ::core::fmt::Debug for DXGI_GAMMA_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_GAMMA_CONTROL").field("Scale", &self.Scale).field("Offset", &self.Offset).field("GammaCurve", &self.GammaCurve).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.ScaleAndOffsetSupported == other.ScaleAndOffsetSupported && self.MaxConvertedValue == other.MaxConvertedValue && self.MinConvertedValue == other.MinConvertedValue && self.NumGammaControlPoints == other.NumGammaControlPoints && self.ControlPointPositions == other.ControlPointPositions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_GAMMA_CONTROL_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_GAMMA_CONTROL_CAPABILITIES").field("ScaleAndOffsetSupported", &self.ScaleAndOffsetSupported).field("MaxConvertedValue", &self.MaxConvertedValue).field("MinConvertedValue", &self.MinConvertedValue).field("NumGammaControlPoints", &self.NumGammaControlPoints).field("ControlPointPositions", &self.ControlPointPositions).finish()
    }
}
impl ::core::default::Default for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::core::cmp::Eq for DXGI_JPEG_AC_HUFFMAN_TABLE {}
impl ::core::fmt::Debug for DXGI_JPEG_AC_HUFFMAN_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_JPEG_AC_HUFFMAN_TABLE").field("CodeCounts", &self.CodeCounts).field("CodeValues", &self.CodeValues).finish()
    }
}
impl ::core::default::Default for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.CodeCounts == other.CodeCounts && self.CodeValues == other.CodeValues
    }
}
impl ::core::cmp::Eq for DXGI_JPEG_DC_HUFFMAN_TABLE {}
impl ::core::fmt::Debug for DXGI_JPEG_DC_HUFFMAN_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_JPEG_DC_HUFFMAN_TABLE").field("CodeCounts", &self.CodeCounts).field("CodeValues", &self.CodeValues).finish()
    }
}
impl ::core::default::Default for DXGI_JPEG_QUANTIZATION_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_JPEG_QUANTIZATION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Elements == other.Elements
    }
}
impl ::core::cmp::Eq for DXGI_JPEG_QUANTIZATION_TABLE {}
impl ::core::fmt::Debug for DXGI_JPEG_QUANTIZATION_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_JPEG_QUANTIZATION_TABLE").field("Elements", &self.Elements).finish()
    }
}
impl ::core::default::Default for DXGI_MODE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_MODE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling
    }
}
impl ::core::cmp::Eq for DXGI_MODE_DESC {}
impl ::core::fmt::Debug for DXGI_MODE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MODE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).finish()
    }
}
impl ::core::default::Default for DXGI_MODE_ROTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_MODE_ROTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MODE_ROTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_MODE_SCALING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_MODE_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MODE_SCALING").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_MODE_SCANLINE_ORDER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_MODE_SCANLINE_ORDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MODE_SCANLINE_ORDER").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_RATIONAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_RATIONAL {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for DXGI_RATIONAL {}
impl ::core::fmt::Debug for DXGI_RATIONAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RATIONAL").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for DXGI_RGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_RGB {
    fn eq(&self, other: &Self) -> bool {
        self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue
    }
}
impl ::core::cmp::Eq for DXGI_RGB {}
impl ::core::fmt::Debug for DXGI_RGB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RGB").field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).finish()
    }
}
impl ::core::default::Default for DXGI_SAMPLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_SAMPLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::core::cmp::Eq for DXGI_SAMPLE_DESC {}
impl ::core::fmt::Debug for DXGI_SAMPLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SAMPLE_DESC").field("Count", &self.Count).field("Quality", &self.Quality).finish()
    }
}
