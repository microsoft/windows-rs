impl ::core::default::Default for ABC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ABC {
    fn eq(&self, other: &Self) -> bool {
        self.abcA == other.abcA && self.abcB == other.abcB && self.abcC == other.abcC
    }
}
impl ::core::cmp::Eq for ABC {}
impl ::core::fmt::Debug for ABC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABC").field("abcA", &self.abcA).field("abcB", &self.abcB).field("abcC", &self.abcC).finish()
    }
}
impl ::core::default::Default for ABCFLOAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ABCFLOAT {
    fn eq(&self, other: &Self) -> bool {
        self.abcfA == other.abcfA && self.abcfB == other.abcfB && self.abcfC == other.abcfC
    }
}
impl ::core::cmp::Eq for ABCFLOAT {}
impl ::core::fmt::Debug for ABCFLOAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABCFLOAT").field("abcfA", &self.abcfA).field("abcfB", &self.abcfB).field("abcfC", &self.abcfC).finish()
    }
}
impl ::core::default::Default for ABORTPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ABORTPATH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr
    }
}
impl ::core::cmp::Eq for ABORTPATH {}
impl ::core::fmt::Debug for ABORTPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ABORTPATH").field("emr", &self.emr).finish()
    }
}
impl ::core::default::Default for ARC_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ARC_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ARC_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for AXESLISTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AXESLISTA {
    fn eq(&self, other: &Self) -> bool {
        self.axlReserved == other.axlReserved && self.axlNumAxes == other.axlNumAxes && self.axlAxisInfo == other.axlAxisInfo
    }
}
impl ::core::cmp::Eq for AXESLISTA {}
impl ::core::fmt::Debug for AXESLISTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXESLISTA").field("axlReserved", &self.axlReserved).field("axlNumAxes", &self.axlNumAxes).field("axlAxisInfo", &self.axlAxisInfo).finish()
    }
}
impl ::core::default::Default for AXESLISTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AXESLISTW {
    fn eq(&self, other: &Self) -> bool {
        self.axlReserved == other.axlReserved && self.axlNumAxes == other.axlNumAxes && self.axlAxisInfo == other.axlAxisInfo
    }
}
impl ::core::cmp::Eq for AXESLISTW {}
impl ::core::fmt::Debug for AXESLISTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXESLISTW").field("axlReserved", &self.axlReserved).field("axlNumAxes", &self.axlNumAxes).field("axlAxisInfo", &self.axlAxisInfo).finish()
    }
}
impl ::core::default::Default for AXISINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AXISINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.axMinValue == other.axMinValue && self.axMaxValue == other.axMaxValue && self.axAxisName == other.axAxisName
    }
}
impl ::core::cmp::Eq for AXISINFOA {}
impl ::core::fmt::Debug for AXISINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXISINFOA").field("axMinValue", &self.axMinValue).field("axMaxValue", &self.axMaxValue).field("axAxisName", &self.axAxisName).finish()
    }
}
impl ::core::default::Default for AXISINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AXISINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.axMinValue == other.axMinValue && self.axMaxValue == other.axMaxValue && self.axAxisName == other.axAxisName
    }
}
impl ::core::cmp::Eq for AXISINFOW {}
impl ::core::fmt::Debug for AXISINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AXISINFOW").field("axMinValue", &self.axMinValue).field("axMaxValue", &self.axMaxValue).field("axAxisName", &self.axAxisName).finish()
    }
}
impl ::core::default::Default for BACKGROUND_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BACKGROUND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BACKGROUND_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.bmType == other.bmType && self.bmWidth == other.bmWidth && self.bmHeight == other.bmHeight && self.bmWidthBytes == other.bmWidthBytes && self.bmPlanes == other.bmPlanes && self.bmBitsPixel == other.bmBitsPixel && self.bmBits == other.bmBits
    }
}
impl ::core::cmp::Eq for BITMAP {}
impl ::core::fmt::Debug for BITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAP").field("bmType", &self.bmType).field("bmWidth", &self.bmWidth).field("bmHeight", &self.bmHeight).field("bmWidthBytes", &self.bmWidthBytes).field("bmPlanes", &self.bmPlanes).field("bmBitsPixel", &self.bmBitsPixel).field("bmBits", &self.bmBits).finish()
    }
}
impl ::core::default::Default for BITMAPCOREHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAPCOREHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bcSize == other.bcSize && self.bcWidth == other.bcWidth && self.bcHeight == other.bcHeight && self.bcPlanes == other.bcPlanes && self.bcBitCount == other.bcBitCount
    }
}
impl ::core::cmp::Eq for BITMAPCOREHEADER {}
impl ::core::fmt::Debug for BITMAPCOREHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPCOREHEADER").field("bcSize", &self.bcSize).field("bcWidth", &self.bcWidth).field("bcHeight", &self.bcHeight).field("bcPlanes", &self.bcPlanes).field("bcBitCount", &self.bcBitCount).finish()
    }
}
impl ::core::default::Default for BITMAPCOREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAPCOREINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bmciHeader == other.bmciHeader && self.bmciColors == other.bmciColors
    }
}
impl ::core::cmp::Eq for BITMAPCOREINFO {}
impl ::core::fmt::Debug for BITMAPCOREINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPCOREINFO").field("bmciHeader", &self.bmciHeader).field("bmciColors", &self.bmciColors).finish()
    }
}
impl ::core::default::Default for BITMAPFILEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for BITMAPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAPINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bmiHeader == other.bmiHeader && self.bmiColors == other.bmiColors
    }
}
impl ::core::cmp::Eq for BITMAPINFO {}
impl ::core::fmt::Debug for BITMAPINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPINFO").field("bmiHeader", &self.bmiHeader).field("bmiColors", &self.bmiColors).finish()
    }
}
impl ::core::default::Default for BITMAPINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAPINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.biSize == other.biSize && self.biWidth == other.biWidth && self.biHeight == other.biHeight && self.biPlanes == other.biPlanes && self.biBitCount == other.biBitCount && self.biCompression == other.biCompression && self.biSizeImage == other.biSizeImage && self.biXPelsPerMeter == other.biXPelsPerMeter && self.biYPelsPerMeter == other.biYPelsPerMeter && self.biClrUsed == other.biClrUsed && self.biClrImportant == other.biClrImportant
    }
}
impl ::core::cmp::Eq for BITMAPINFOHEADER {}
impl ::core::fmt::Debug for BITMAPINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPINFOHEADER").field("biSize", &self.biSize).field("biWidth", &self.biWidth).field("biHeight", &self.biHeight).field("biPlanes", &self.biPlanes).field("biBitCount", &self.biBitCount).field("biCompression", &self.biCompression).field("biSizeImage", &self.biSizeImage).field("biXPelsPerMeter", &self.biXPelsPerMeter).field("biYPelsPerMeter", &self.biYPelsPerMeter).field("biClrUsed", &self.biClrUsed).field("biClrImportant", &self.biClrImportant).finish()
    }
}
impl ::core::default::Default for BITMAPV4HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAPV4HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bV4Size == other.bV4Size
            && self.bV4Width == other.bV4Width
            && self.bV4Height == other.bV4Height
            && self.bV4Planes == other.bV4Planes
            && self.bV4BitCount == other.bV4BitCount
            && self.bV4V4Compression == other.bV4V4Compression
            && self.bV4SizeImage == other.bV4SizeImage
            && self.bV4XPelsPerMeter == other.bV4XPelsPerMeter
            && self.bV4YPelsPerMeter == other.bV4YPelsPerMeter
            && self.bV4ClrUsed == other.bV4ClrUsed
            && self.bV4ClrImportant == other.bV4ClrImportant
            && self.bV4RedMask == other.bV4RedMask
            && self.bV4GreenMask == other.bV4GreenMask
            && self.bV4BlueMask == other.bV4BlueMask
            && self.bV4AlphaMask == other.bV4AlphaMask
            && self.bV4CSType == other.bV4CSType
            && self.bV4Endpoints == other.bV4Endpoints
            && self.bV4GammaRed == other.bV4GammaRed
            && self.bV4GammaGreen == other.bV4GammaGreen
            && self.bV4GammaBlue == other.bV4GammaBlue
    }
}
impl ::core::cmp::Eq for BITMAPV4HEADER {}
impl ::core::fmt::Debug for BITMAPV4HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPV4HEADER")
            .field("bV4Size", &self.bV4Size)
            .field("bV4Width", &self.bV4Width)
            .field("bV4Height", &self.bV4Height)
            .field("bV4Planes", &self.bV4Planes)
            .field("bV4BitCount", &self.bV4BitCount)
            .field("bV4V4Compression", &self.bV4V4Compression)
            .field("bV4SizeImage", &self.bV4SizeImage)
            .field("bV4XPelsPerMeter", &self.bV4XPelsPerMeter)
            .field("bV4YPelsPerMeter", &self.bV4YPelsPerMeter)
            .field("bV4ClrUsed", &self.bV4ClrUsed)
            .field("bV4ClrImportant", &self.bV4ClrImportant)
            .field("bV4RedMask", &self.bV4RedMask)
            .field("bV4GreenMask", &self.bV4GreenMask)
            .field("bV4BlueMask", &self.bV4BlueMask)
            .field("bV4AlphaMask", &self.bV4AlphaMask)
            .field("bV4CSType", &self.bV4CSType)
            .field("bV4Endpoints", &self.bV4Endpoints)
            .field("bV4GammaRed", &self.bV4GammaRed)
            .field("bV4GammaGreen", &self.bV4GammaGreen)
            .field("bV4GammaBlue", &self.bV4GammaBlue)
            .finish()
    }
}
impl ::core::default::Default for BITMAPV5HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BITMAPV5HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.bV5Size == other.bV5Size
            && self.bV5Width == other.bV5Width
            && self.bV5Height == other.bV5Height
            && self.bV5Planes == other.bV5Planes
            && self.bV5BitCount == other.bV5BitCount
            && self.bV5Compression == other.bV5Compression
            && self.bV5SizeImage == other.bV5SizeImage
            && self.bV5XPelsPerMeter == other.bV5XPelsPerMeter
            && self.bV5YPelsPerMeter == other.bV5YPelsPerMeter
            && self.bV5ClrUsed == other.bV5ClrUsed
            && self.bV5ClrImportant == other.bV5ClrImportant
            && self.bV5RedMask == other.bV5RedMask
            && self.bV5GreenMask == other.bV5GreenMask
            && self.bV5BlueMask == other.bV5BlueMask
            && self.bV5AlphaMask == other.bV5AlphaMask
            && self.bV5CSType == other.bV5CSType
            && self.bV5Endpoints == other.bV5Endpoints
            && self.bV5GammaRed == other.bV5GammaRed
            && self.bV5GammaGreen == other.bV5GammaGreen
            && self.bV5GammaBlue == other.bV5GammaBlue
            && self.bV5Intent == other.bV5Intent
            && self.bV5ProfileData == other.bV5ProfileData
            && self.bV5ProfileSize == other.bV5ProfileSize
            && self.bV5Reserved == other.bV5Reserved
    }
}
impl ::core::cmp::Eq for BITMAPV5HEADER {}
impl ::core::fmt::Debug for BITMAPV5HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BITMAPV5HEADER")
            .field("bV5Size", &self.bV5Size)
            .field("bV5Width", &self.bV5Width)
            .field("bV5Height", &self.bV5Height)
            .field("bV5Planes", &self.bV5Planes)
            .field("bV5BitCount", &self.bV5BitCount)
            .field("bV5Compression", &self.bV5Compression)
            .field("bV5SizeImage", &self.bV5SizeImage)
            .field("bV5XPelsPerMeter", &self.bV5XPelsPerMeter)
            .field("bV5YPelsPerMeter", &self.bV5YPelsPerMeter)
            .field("bV5ClrUsed", &self.bV5ClrUsed)
            .field("bV5ClrImportant", &self.bV5ClrImportant)
            .field("bV5RedMask", &self.bV5RedMask)
            .field("bV5GreenMask", &self.bV5GreenMask)
            .field("bV5BlueMask", &self.bV5BlueMask)
            .field("bV5AlphaMask", &self.bV5AlphaMask)
            .field("bV5CSType", &self.bV5CSType)
            .field("bV5Endpoints", &self.bV5Endpoints)
            .field("bV5GammaRed", &self.bV5GammaRed)
            .field("bV5GammaGreen", &self.bV5GammaGreen)
            .field("bV5GammaBlue", &self.bV5GammaBlue)
            .field("bV5Intent", &self.bV5Intent)
            .field("bV5ProfileData", &self.bV5ProfileData)
            .field("bV5ProfileSize", &self.bV5ProfileSize)
            .field("bV5Reserved", &self.bV5Reserved)
            .finish()
    }
}
impl ::core::default::Default for BI_COMPRESSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BI_COMPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BI_COMPRESSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for BLENDFUNCTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BLENDFUNCTION {
    fn eq(&self, other: &Self) -> bool {
        self.BlendOp == other.BlendOp && self.BlendFlags == other.BlendFlags && self.SourceConstantAlpha == other.SourceConstantAlpha && self.AlphaFormat == other.AlphaFormat
    }
}
impl ::core::cmp::Eq for BLENDFUNCTION {}
impl ::core::fmt::Debug for BLENDFUNCTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BLENDFUNCTION").field("BlendOp", &self.BlendOp).field("BlendFlags", &self.BlendFlags).field("SourceConstantAlpha", &self.SourceConstantAlpha).field("AlphaFormat", &self.AlphaFormat).finish()
    }
}
impl ::core::default::Default for BRUSH_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BRUSH_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BRUSH_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CDS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CDS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CDS_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CDS_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CDS_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CDS_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CDS_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CDS_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CIEXYZ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CIEXYZ {
    fn eq(&self, other: &Self) -> bool {
        self.ciexyzX == other.ciexyzX && self.ciexyzY == other.ciexyzY && self.ciexyzZ == other.ciexyzZ
    }
}
impl ::core::cmp::Eq for CIEXYZ {}
impl ::core::fmt::Debug for CIEXYZ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIEXYZ").field("ciexyzX", &self.ciexyzX).field("ciexyzY", &self.ciexyzY).field("ciexyzZ", &self.ciexyzZ).finish()
    }
}
impl ::core::default::Default for CIEXYZTRIPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CIEXYZTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        self.ciexyzRed == other.ciexyzRed && self.ciexyzGreen == other.ciexyzGreen && self.ciexyzBlue == other.ciexyzBlue
    }
}
impl ::core::cmp::Eq for CIEXYZTRIPLE {}
impl ::core::fmt::Debug for CIEXYZTRIPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CIEXYZTRIPLE").field("ciexyzRed", &self.ciexyzRed).field("ciexyzGreen", &self.ciexyzGreen).field("ciexyzBlue", &self.ciexyzBlue).finish()
    }
}
impl ::core::default::Default for COLORADJUSTMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for COLORADJUSTMENT {
    fn eq(&self, other: &Self) -> bool {
        self.caSize == other.caSize && self.caFlags == other.caFlags && self.caIlluminantIndex == other.caIlluminantIndex && self.caRedGamma == other.caRedGamma && self.caGreenGamma == other.caGreenGamma && self.caBlueGamma == other.caBlueGamma && self.caReferenceBlack == other.caReferenceBlack && self.caReferenceWhite == other.caReferenceWhite && self.caContrast == other.caContrast && self.caBrightness == other.caBrightness && self.caColorfulness == other.caColorfulness && self.caRedGreenTint == other.caRedGreenTint
    }
}
impl ::core::cmp::Eq for COLORADJUSTMENT {}
impl ::core::fmt::Debug for COLORADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLORADJUSTMENT")
            .field("caSize", &self.caSize)
            .field("caFlags", &self.caFlags)
            .field("caIlluminantIndex", &self.caIlluminantIndex)
            .field("caRedGamma", &self.caRedGamma)
            .field("caGreenGamma", &self.caGreenGamma)
            .field("caBlueGamma", &self.caBlueGamma)
            .field("caReferenceBlack", &self.caReferenceBlack)
            .field("caReferenceWhite", &self.caReferenceWhite)
            .field("caContrast", &self.caContrast)
            .field("caBrightness", &self.caBrightness)
            .field("caColorfulness", &self.caColorfulness)
            .field("caRedGreenTint", &self.caRedGreenTint)
            .finish()
    }
}
impl ::core::default::Default for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_FONT_PACKAGE_SUBSET_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_FONT_PACKAGE_SUBSET_ENCODING").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_FONT_PACKAGE_SUBSET_PLATFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_FONT_PACKAGE_SUBSET_PLATFORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for CREATE_POLYGON_RGN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_POLYGON_RGN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_POLYGON_RGN_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DC_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DC_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DC_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DC_LAYOUT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DC_LAYOUT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DC_LAYOUT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DC_LAYOUT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DC_LAYOUT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DESIGNVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DESIGNVECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.dvReserved == other.dvReserved && self.dvNumAxes == other.dvNumAxes && self.dvValues == other.dvValues
    }
}
impl ::core::cmp::Eq for DESIGNVECTOR {}
impl ::core::fmt::Debug for DESIGNVECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DESIGNVECTOR").field("dvReserved", &self.dvReserved).field("dvNumAxes", &self.dvNumAxes).field("dvValues", &self.dvValues).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVMODEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVMODE_COLLATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_COLLATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_COLLATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVMODE_COLOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_COLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_COLOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVMODE_DISPLAY_FIXED_OUTPUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_DISPLAY_FIXED_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_DISPLAY_FIXED_OUTPUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVMODE_DISPLAY_ORIENTATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_DISPLAY_ORIENTATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_DISPLAY_ORIENTATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVMODE_DUPLEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_DUPLEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_DUPLEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVMODE_FIELD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_FIELD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_FIELD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DEVMODE_FIELD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DEVMODE_FIELD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DEVMODE_FIELD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DEVMODE_TRUETYPE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVMODE_TRUETYPE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVMODE_TRUETYPE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for DFCS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFCS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFCS_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DFCS_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DFCS_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DFCS_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DFCS_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DFCS_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DFC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DFC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DFC_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIBSECTION {
    fn eq(&self, other: &Self) -> bool {
        self.dsBm == other.dsBm && self.dsBmih == other.dsBmih && self.dsBitfields == other.dsBitfields && self.dshSection == other.dshSection && self.dsOffset == other.dsOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIBSECTION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIBSECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIBSECTION").field("dsBm", &self.dsBm).field("dsBmih", &self.dsBmih).field("dsBitfields", &self.dsBitfields).field("dshSection", &self.dshSection).field("dsOffset", &self.dsOffset).finish()
    }
}
impl ::core::default::Default for DIB_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DIB_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIB_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISPLAYCONFIG_COLOR_ENCODING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISPLAYCONFIG_COLOR_ENCODING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISPLAYCONFIG_COLOR_ENCODING").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISPLAY_DEVICEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISPLAY_DEVICEA {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.DeviceName == other.DeviceName && self.DeviceString == other.DeviceString && self.StateFlags == other.StateFlags && self.DeviceID == other.DeviceID && self.DeviceKey == other.DeviceKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISPLAY_DEVICEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISPLAY_DEVICEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_DEVICEA").field("cb", &self.cb).field("DeviceName", &self.DeviceName).field("DeviceString", &self.DeviceString).field("StateFlags", &self.StateFlags).field("DeviceID", &self.DeviceID).field("DeviceKey", &self.DeviceKey).finish()
    }
}
impl ::core::default::Default for DISPLAY_DEVICEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISPLAY_DEVICEW {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.DeviceName == other.DeviceName && self.DeviceString == other.DeviceString && self.StateFlags == other.StateFlags && self.DeviceID == other.DeviceID && self.DeviceKey == other.DeviceKey
    }
}
impl ::core::cmp::Eq for DISPLAY_DEVICEW {}
impl ::core::fmt::Debug for DISPLAY_DEVICEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISPLAY_DEVICEW").field("cb", &self.cb).field("DeviceName", &self.DeviceName).field("DeviceString", &self.DeviceString).field("StateFlags", &self.StateFlags).field("DeviceID", &self.DeviceID).field("DeviceKey", &self.DeviceKey).finish()
    }
}
impl ::core::default::Default for DISP_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISP_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISP_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DRAWEDGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAWEDGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWEDGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWEDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWEDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWEDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWEDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DRAWSTATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAWSTATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAWSTATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAWSTATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAWSTATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAWSTATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAWSTATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DRAWTEXTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRAWTEXTPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.iTabLength == other.iTabLength && self.iLeftMargin == other.iLeftMargin && self.iRightMargin == other.iRightMargin && self.uiLengthDrawn == other.uiLengthDrawn
    }
}
impl ::core::cmp::Eq for DRAWTEXTPARAMS {}
impl ::core::fmt::Debug for DRAWTEXTPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWTEXTPARAMS").field("cbSize", &self.cbSize).field("iTabLength", &self.iTabLength).field("iLeftMargin", &self.iLeftMargin).field("iRightMargin", &self.iRightMargin).field("uiLengthDrawn", &self.uiLengthDrawn).finish()
    }
}
impl ::core::default::Default for DRAW_CAPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_CAPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_CAPTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_CAPTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_CAPTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_CAPTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DRAW_EDGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_EDGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_EDGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_EDGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_EDGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_EDGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DRAW_TEXT_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DRAW_TEXT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRAW_TEXT_FORMAT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DRAW_TEXT_FORMAT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DRAW_TEXT_FORMAT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DRAW_TEXT_FORMAT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for EMBEDDED_FONT_PRIV_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMBEDDED_FONT_PRIV_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBEDDED_FONT_PRIV_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EMBED_FONT_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMBED_FONT_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMBED_FONT_CHARSET").field(&self.0).finish()
    }
}
impl ::core::default::Default for EMR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMR {
    fn eq(&self, other: &Self) -> bool {
        self.iType == other.iType && self.nSize == other.nSize
    }
}
impl ::core::cmp::Eq for EMR {}
impl ::core::fmt::Debug for EMR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMR").field("iType", &self.iType).field("nSize", &self.nSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRALPHABLEND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRALPHABLEND {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.cxDest == other.cxDest && self.cyDest == other.cyDest && self.dwRop == other.dwRop && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.xformSrc == other.xformSrc && self.crBkColorSrc == other.crBkColorSrc && self.iUsageSrc == other.iUsageSrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.cxSrc == other.cxSrc && self.cySrc == other.cySrc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRALPHABLEND {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRALPHABLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRALPHABLEND")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRANGLEARC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRANGLEARC {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlCenter == other.ptlCenter && self.nRadius == other.nRadius && self.eStartAngle == other.eStartAngle && self.eSweepAngle == other.eSweepAngle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRANGLEARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRANGLEARC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRANGLEARC").field("emr", &self.emr).field("ptlCenter", &self.ptlCenter).field("nRadius", &self.nRadius).field("eStartAngle", &self.eStartAngle).field("eSweepAngle", &self.eSweepAngle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRARC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRARC {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBox == other.rclBox && self.ptlStart == other.ptlStart && self.ptlEnd == other.ptlEnd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRARC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRARC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRARC").field("emr", &self.emr).field("rclBox", &self.rclBox).field("ptlStart", &self.ptlStart).field("ptlEnd", &self.ptlEnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRBITBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRBITBLT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.cxDest == other.cxDest && self.cyDest == other.cyDest && self.dwRop == other.dwRop && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.xformSrc == other.xformSrc && self.crBkColorSrc == other.crBkColorSrc && self.iUsageSrc == other.iUsageSrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRBITBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRBITBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRBITBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .finish()
    }
}
impl ::core::default::Default for EMRCOLORCORRECTPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRCOLORCORRECTPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPalette == other.ihPalette && self.nFirstEntry == other.nFirstEntry && self.nPalEntries == other.nPalEntries && self.nReserved == other.nReserved
    }
}
impl ::core::cmp::Eq for EMRCOLORCORRECTPALETTE {}
impl ::core::fmt::Debug for EMRCOLORCORRECTPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCOLORCORRECTPALETTE").field("emr", &self.emr).field("ihPalette", &self.ihPalette).field("nFirstEntry", &self.nFirstEntry).field("nPalEntries", &self.nPalEntries).field("nReserved", &self.nReserved).finish()
    }
}
impl ::core::default::Default for EMRCOLORMATCHTOTARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRCOLORMATCHTOTARGET {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.dwAction == other.dwAction && self.dwFlags == other.dwFlags && self.cbName == other.cbName && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRCOLORMATCHTOTARGET {}
impl ::core::fmt::Debug for EMRCOLORMATCHTOTARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCOLORMATCHTOTARGET").field("emr", &self.emr).field("dwAction", &self.dwAction).field("dwFlags", &self.dwFlags).field("cbName", &self.cbName).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRCREATEBRUSHINDIRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRCREATEBRUSHINDIRECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihBrush == other.ihBrush && self.lb == other.lb
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRCREATEBRUSHINDIRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRCREATEBRUSHINDIRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEBRUSHINDIRECT").field("emr", &self.emr).field("ihBrush", &self.ihBrush).field("lb", &self.lb).finish()
    }
}
impl ::core::default::Default for EMRCREATEDIBPATTERNBRUSHPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRCREATEDIBPATTERNBRUSHPT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihBrush == other.ihBrush && self.iUsage == other.iUsage && self.offBmi == other.offBmi && self.cbBmi == other.cbBmi && self.offBits == other.offBits && self.cbBits == other.cbBits
    }
}
impl ::core::cmp::Eq for EMRCREATEDIBPATTERNBRUSHPT {}
impl ::core::fmt::Debug for EMRCREATEDIBPATTERNBRUSHPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEDIBPATTERNBRUSHPT").field("emr", &self.emr).field("ihBrush", &self.ihBrush).field("iUsage", &self.iUsage).field("offBmi", &self.offBmi).field("cbBmi", &self.cbBmi).field("offBits", &self.offBits).field("cbBits", &self.cbBits).finish()
    }
}
impl ::core::default::Default for EMRCREATEMONOBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRCREATEMONOBRUSH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihBrush == other.ihBrush && self.iUsage == other.iUsage && self.offBmi == other.offBmi && self.cbBmi == other.cbBmi && self.offBits == other.offBits && self.cbBits == other.cbBits
    }
}
impl ::core::cmp::Eq for EMRCREATEMONOBRUSH {}
impl ::core::fmt::Debug for EMRCREATEMONOBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEMONOBRUSH").field("emr", &self.emr).field("ihBrush", &self.ihBrush).field("iUsage", &self.iUsage).field("offBmi", &self.offBmi).field("cbBmi", &self.cbBmi).field("offBits", &self.offBits).field("cbBits", &self.cbBits).finish()
    }
}
impl ::core::default::Default for EMRCREATEPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRCREATEPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal && self.lgpl == other.lgpl
    }
}
impl ::core::cmp::Eq for EMRCREATEPALETTE {}
impl ::core::fmt::Debug for EMRCREATEPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEPALETTE").field("emr", &self.emr).field("ihPal", &self.ihPal).field("lgpl", &self.lgpl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRCREATEPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRCREATEPEN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPen == other.ihPen && self.lopn == other.lopn
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRCREATEPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATEPEN").field("emr", &self.emr).field("ihPen", &self.ihPen).field("lopn", &self.lopn).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRELLIPSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRELLIPSE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBox == other.rclBox
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRELLIPSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRELLIPSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRELLIPSE").field("emr", &self.emr).field("rclBox", &self.rclBox).finish()
    }
}
impl ::core::default::Default for EMREOF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMREOF {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.nPalEntries == other.nPalEntries && self.offPalEntries == other.offPalEntries && self.nSizeLast == other.nSizeLast
    }
}
impl ::core::cmp::Eq for EMREOF {}
impl ::core::fmt::Debug for EMREOF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREOF").field("emr", &self.emr).field("nPalEntries", &self.nPalEntries).field("offPalEntries", &self.offPalEntries).field("nSizeLast", &self.nSizeLast).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXCLUDECLIPRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXCLUDECLIPRECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclClip == other.rclClip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXCLUDECLIPRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXCLUDECLIPRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXCLUDECLIPRECT").field("emr", &self.emr).field("rclClip", &self.rclClip).finish()
    }
}
impl ::core::default::Default for EMREXTCREATEFONTINDIRECTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMREXTCREATEFONTINDIRECTW {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihFont == other.ihFont && self.elfw == other.elfw
    }
}
impl ::core::cmp::Eq for EMREXTCREATEFONTINDIRECTW {}
impl ::core::fmt::Debug for EMREXTCREATEFONTINDIRECTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTCREATEFONTINDIRECTW").field("emr", &self.emr).field("ihFont", &self.ihFont).field("elfw", &self.elfw).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXTCREATEPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXTCREATEPEN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPen == other.ihPen && self.offBmi == other.offBmi && self.cbBmi == other.cbBmi && self.offBits == other.offBits && self.cbBits == other.cbBits && self.elp == other.elp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXTCREATEPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXTCREATEPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTCREATEPEN").field("emr", &self.emr).field("ihPen", &self.ihPen).field("offBmi", &self.offBmi).field("cbBmi", &self.cbBmi).field("offBits", &self.offBits).field("cbBits", &self.cbBits).field("elp", &self.elp).finish()
    }
}
impl ::core::default::Default for EMREXTESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMREXTESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iEscape == other.iEscape && self.cbEscData == other.cbEscData && self.EscData == other.EscData
    }
}
impl ::core::cmp::Eq for EMREXTESCAPE {}
impl ::core::fmt::Debug for EMREXTESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTESCAPE").field("emr", &self.emr).field("iEscape", &self.iEscape).field("cbEscData", &self.cbEscData).field("EscData", &self.EscData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXTFLOODFILL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXTFLOODFILL {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlStart == other.ptlStart && self.crColor == other.crColor && self.iMode == other.iMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXTFLOODFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXTFLOODFILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTFLOODFILL").field("emr", &self.emr).field("ptlStart", &self.ptlStart).field("crColor", &self.crColor).field("iMode", &self.iMode).finish()
    }
}
impl ::core::default::Default for EMREXTSELECTCLIPRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMREXTSELECTCLIPRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.cbRgnData == other.cbRgnData && self.iMode == other.iMode && self.RgnData == other.RgnData
    }
}
impl ::core::cmp::Eq for EMREXTSELECTCLIPRGN {}
impl ::core::fmt::Debug for EMREXTSELECTCLIPRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTSELECTCLIPRGN").field("emr", &self.emr).field("cbRgnData", &self.cbRgnData).field("iMode", &self.iMode).field("RgnData", &self.RgnData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMREXTTEXTOUTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMREXTTEXTOUTA {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.iGraphicsMode == other.iGraphicsMode && self.exScale == other.exScale && self.eyScale == other.eyScale && self.emrtext == other.emrtext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMREXTTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMREXTTEXTOUTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMREXTTEXTOUTA").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("iGraphicsMode", &self.iGraphicsMode).field("exScale", &self.exScale).field("eyScale", &self.eyScale).field("emrtext", &self.emrtext).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRFILLPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRFILLPATH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRFILLPATH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRFILLPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFILLPATH").field("emr", &self.emr).field("rclBounds", &self.rclBounds).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRFILLRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRFILLRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cbRgnData == other.cbRgnData && self.ihBrush == other.ihBrush && self.RgnData == other.RgnData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRFILLRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRFILLRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFILLRGN").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbRgnData", &self.cbRgnData).field("ihBrush", &self.ihBrush).field("RgnData", &self.RgnData).finish()
    }
}
impl ::core::default::Default for EMRFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.dSignature == other.dSignature && self.nVersion == other.nVersion && self.cbData == other.cbData && self.offData == other.offData
    }
}
impl ::core::cmp::Eq for EMRFORMAT {}
impl ::core::fmt::Debug for EMRFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFORMAT").field("dSignature", &self.dSignature).field("nVersion", &self.nVersion).field("cbData", &self.cbData).field("offData", &self.offData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRFRAMERGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRFRAMERGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cbRgnData == other.cbRgnData && self.ihBrush == other.ihBrush && self.szlStroke == other.szlStroke && self.RgnData == other.RgnData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRFRAMERGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRFRAMERGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRFRAMERGN").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbRgnData", &self.cbRgnData).field("ihBrush", &self.ihBrush).field("szlStroke", &self.szlStroke).field("RgnData", &self.RgnData).finish()
    }
}
impl ::core::default::Default for EMRGDICOMMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRGDICOMMENT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRGDICOMMENT {}
impl ::core::fmt::Debug for EMRGDICOMMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGDICOMMENT").field("emr", &self.emr).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRGLSBOUNDEDRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRGLSBOUNDEDRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cbData == other.cbData && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRGLSBOUNDEDRECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRGLSBOUNDEDRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGLSBOUNDEDRECORD").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for EMRGLSRECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRGLSRECORD {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRGLSRECORD {}
impl ::core::fmt::Debug for EMRGLSRECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGLSRECORD").field("emr", &self.emr).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRGRADIENTFILL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRGRADIENTFILL {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.nVer == other.nVer && self.nTri == other.nTri && self.ulMode == other.ulMode && self.Ver == other.Ver
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRGRADIENTFILL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRGRADIENTFILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRGRADIENTFILL").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("nVer", &self.nVer).field("nTri", &self.nTri).field("ulMode", &self.ulMode).field("Ver", &self.Ver).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRINVERTRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRINVERTRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cbRgnData == other.cbRgnData && self.RgnData == other.RgnData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRINVERTRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRINVERTRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRINVERTRGN").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cbRgnData", &self.cbRgnData).field("RgnData", &self.RgnData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRLINETO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRLINETO {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptl == other.ptl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRLINETO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRLINETO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRLINETO").field("emr", &self.emr).field("ptl", &self.ptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRMASKBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRMASKBLT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.cxDest == other.cxDest && self.cyDest == other.cyDest && self.dwRop == other.dwRop && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.xformSrc == other.xformSrc && self.crBkColorSrc == other.crBkColorSrc && self.iUsageSrc == other.iUsageSrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.xMask == other.xMask && self.yMask == other.yMask && self.iUsageMask == other.iUsageMask && self.offBmiMask == other.offBmiMask && self.cbBmiMask == other.cbBmiMask && self.offBitsMask == other.offBitsMask && self.cbBitsMask == other.cbBitsMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRMASKBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRMASKBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRMASKBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("xMask", &self.xMask)
            .field("yMask", &self.yMask)
            .field("iUsageMask", &self.iUsageMask)
            .field("offBmiMask", &self.offBmiMask)
            .field("cbBmiMask", &self.cbBmiMask)
            .field("offBitsMask", &self.offBitsMask)
            .field("cbBitsMask", &self.cbBitsMask)
            .finish()
    }
}
impl ::core::default::Default for EMRMODIFYWORLDTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRMODIFYWORLDTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.xform == other.xform && self.iMode == other.iMode
    }
}
impl ::core::cmp::Eq for EMRMODIFYWORLDTRANSFORM {}
impl ::core::fmt::Debug for EMRMODIFYWORLDTRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRMODIFYWORLDTRANSFORM").field("emr", &self.emr).field("xform", &self.xform).field("iMode", &self.iMode).finish()
    }
}
impl ::core::default::Default for EMRNAMEDESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRNAMEDESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iEscape == other.iEscape && self.cbDriver == other.cbDriver && self.cbEscData == other.cbEscData && self.EscData == other.EscData
    }
}
impl ::core::cmp::Eq for EMRNAMEDESCAPE {}
impl ::core::fmt::Debug for EMRNAMEDESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRNAMEDESCAPE").field("emr", &self.emr).field("iEscape", &self.iEscape).field("cbDriver", &self.cbDriver).field("cbEscData", &self.cbEscData).field("EscData", &self.EscData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMROFFSETCLIPRGN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMROFFSETCLIPRGN {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlOffset == other.ptlOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMROFFSETCLIPRGN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMROFFSETCLIPRGN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMROFFSETCLIPRGN").field("emr", &self.emr).field("ptlOffset", &self.ptlOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPLGBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPLGBLT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.aptlDest == other.aptlDest && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.cxSrc == other.cxSrc && self.cySrc == other.cySrc && self.xformSrc == other.xformSrc && self.crBkColorSrc == other.crBkColorSrc && self.iUsageSrc == other.iUsageSrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.xMask == other.xMask && self.yMask == other.yMask && self.iUsageMask == other.iUsageMask && self.offBmiMask == other.offBmiMask && self.cbBmiMask == other.cbBmiMask && self.offBitsMask == other.offBitsMask && self.cbBitsMask == other.cbBitsMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPLGBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPLGBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPLGBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("aptlDest", &self.aptlDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("xMask", &self.xMask)
            .field("yMask", &self.yMask)
            .field("iUsageMask", &self.iUsageMask)
            .field("offBmiMask", &self.offBmiMask)
            .field("cbBmiMask", &self.cbBmiMask)
            .field("offBitsMask", &self.offBitsMask)
            .field("cbBitsMask", &self.cbBitsMask)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYDRAW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYDRAW {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cptl == other.cptl && self.aptl == other.aptl && self.abTypes == other.abTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYDRAW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYDRAW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYDRAW").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cptl", &self.cptl).field("aptl", &self.aptl).field("abTypes", &self.abTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYDRAW16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYDRAW16 {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cpts == other.cpts && self.apts == other.apts && self.abTypes == other.abTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYDRAW16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYDRAW16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYDRAW16").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cpts", &self.cpts).field("apts", &self.apts).field("abTypes", &self.abTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYLINE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cptl == other.cptl && self.aptl == other.aptl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYLINE").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cptl", &self.cptl).field("aptl", &self.aptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYLINE16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYLINE16 {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.cpts == other.cpts && self.apts == other.apts
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYLINE16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYLINE16").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("cpts", &self.cpts).field("apts", &self.apts).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYPOLYLINE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYPOLYLINE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.nPolys == other.nPolys && self.cptl == other.cptl && self.aPolyCounts == other.aPolyCounts && self.aptl == other.aptl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYPOLYLINE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYPOLYLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYPOLYLINE").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("nPolys", &self.nPolys).field("cptl", &self.cptl).field("aPolyCounts", &self.aPolyCounts).field("aptl", &self.aptl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYPOLYLINE16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYPOLYLINE16 {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.nPolys == other.nPolys && self.cpts == other.cpts && self.aPolyCounts == other.aPolyCounts && self.apts == other.apts
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYPOLYLINE16 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYPOLYLINE16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYPOLYLINE16").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("nPolys", &self.nPolys).field("cpts", &self.cpts).field("aPolyCounts", &self.aPolyCounts).field("apts", &self.apts).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRPOLYTEXTOUTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRPOLYTEXTOUTA {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.iGraphicsMode == other.iGraphicsMode && self.exScale == other.exScale && self.eyScale == other.eyScale && self.cStrings == other.cStrings && self.aemrtext == other.aemrtext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRPOLYTEXTOUTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRPOLYTEXTOUTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRPOLYTEXTOUTA").field("emr", &self.emr).field("rclBounds", &self.rclBounds).field("iGraphicsMode", &self.iGraphicsMode).field("exScale", &self.exScale).field("eyScale", &self.eyScale).field("cStrings", &self.cStrings).field("aemrtext", &self.aemrtext).finish()
    }
}
impl ::core::default::Default for EMRRESIZEPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRRESIZEPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal && self.cEntries == other.cEntries
    }
}
impl ::core::cmp::Eq for EMRRESIZEPALETTE {}
impl ::core::fmt::Debug for EMRRESIZEPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRRESIZEPALETTE").field("emr", &self.emr).field("ihPal", &self.ihPal).field("cEntries", &self.cEntries).finish()
    }
}
impl ::core::default::Default for EMRRESTOREDC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRRESTOREDC {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iRelative == other.iRelative
    }
}
impl ::core::cmp::Eq for EMRRESTOREDC {}
impl ::core::fmt::Debug for EMRRESTOREDC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRRESTOREDC").field("emr", &self.emr).field("iRelative", &self.iRelative).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRROUNDRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRROUNDRECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBox == other.rclBox && self.szlCorner == other.szlCorner
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRROUNDRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRROUNDRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRROUNDRECT").field("emr", &self.emr).field("rclBox", &self.rclBox).field("szlCorner", &self.szlCorner).finish()
    }
}
impl ::core::default::Default for EMRSCALEVIEWPORTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSCALEVIEWPORTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.xNum == other.xNum && self.xDenom == other.xDenom && self.yNum == other.yNum && self.yDenom == other.yDenom
    }
}
impl ::core::cmp::Eq for EMRSCALEVIEWPORTEXTEX {}
impl ::core::fmt::Debug for EMRSCALEVIEWPORTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSCALEVIEWPORTEXTEX").field("emr", &self.emr).field("xNum", &self.xNum).field("xDenom", &self.xDenom).field("yNum", &self.yNum).field("yDenom", &self.yDenom).finish()
    }
}
impl ::core::default::Default for EMRSELECTCLIPPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSELECTCLIPPATH {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iMode == other.iMode
    }
}
impl ::core::cmp::Eq for EMRSELECTCLIPPATH {}
impl ::core::fmt::Debug for EMRSELECTCLIPPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTCLIPPATH").field("emr", &self.emr).field("iMode", &self.iMode).finish()
    }
}
impl ::core::default::Default for EMRSELECTOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSELECTOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihObject == other.ihObject
    }
}
impl ::core::cmp::Eq for EMRSELECTOBJECT {}
impl ::core::fmt::Debug for EMRSELECTOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTOBJECT").field("emr", &self.emr).field("ihObject", &self.ihObject).finish()
    }
}
impl ::core::default::Default for EMRSELECTPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSELECTPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal
    }
}
impl ::core::cmp::Eq for EMRSELECTPALETTE {}
impl ::core::fmt::Debug for EMRSELECTPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSELECTPALETTE").field("emr", &self.emr).field("ihPal", &self.ihPal).finish()
    }
}
impl ::core::default::Default for EMRSETARCDIRECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETARCDIRECTION {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.iArcDirection == other.iArcDirection
    }
}
impl ::core::cmp::Eq for EMRSETARCDIRECTION {}
impl ::core::fmt::Debug for EMRSETARCDIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETARCDIRECTION").field("emr", &self.emr).field("iArcDirection", &self.iArcDirection).finish()
    }
}
impl ::core::default::Default for EMRSETCOLORADJUSTMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETCOLORADJUSTMENT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ColorAdjustment == other.ColorAdjustment
    }
}
impl ::core::cmp::Eq for EMRSETCOLORADJUSTMENT {}
impl ::core::fmt::Debug for EMRSETCOLORADJUSTMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETCOLORADJUSTMENT").field("emr", &self.emr).field("ColorAdjustment", &self.ColorAdjustment).finish()
    }
}
impl ::core::default::Default for EMRSETCOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETCOLORSPACE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihCS == other.ihCS
    }
}
impl ::core::cmp::Eq for EMRSETCOLORSPACE {}
impl ::core::fmt::Debug for EMRSETCOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETCOLORSPACE").field("emr", &self.emr).field("ihCS", &self.ihCS).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETDIBITSTODEVICE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETDIBITSTODEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.cxSrc == other.cxSrc && self.cySrc == other.cySrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.iUsageSrc == other.iUsageSrc && self.iStartScan == other.iStartScan && self.cScans == other.cScans
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETDIBITSTODEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETDIBITSTODEVICE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETDIBITSTODEVICE")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("iStartScan", &self.iStartScan)
            .field("cScans", &self.cScans)
            .finish()
    }
}
impl ::core::default::Default for EMRSETICMPROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETICMPROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.dwFlags == other.dwFlags && self.cbName == other.cbName && self.cbData == other.cbData && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for EMRSETICMPROFILE {}
impl ::core::fmt::Debug for EMRSETICMPROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETICMPROFILE").field("emr", &self.emr).field("dwFlags", &self.dwFlags).field("cbName", &self.cbName).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for EMRSETMAPPERFLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETMAPPERFLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for EMRSETMAPPERFLAGS {}
impl ::core::fmt::Debug for EMRSETMAPPERFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETMAPPERFLAGS").field("emr", &self.emr).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for EMRSETMITERLIMIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETMITERLIMIT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.eMiterLimit == other.eMiterLimit
    }
}
impl ::core::cmp::Eq for EMRSETMITERLIMIT {}
impl ::core::fmt::Debug for EMRSETMITERLIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETMITERLIMIT").field("emr", &self.emr).field("eMiterLimit", &self.eMiterLimit).finish()
    }
}
impl ::core::default::Default for EMRSETPALETTEENTRIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETPALETTEENTRIES {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihPal == other.ihPal && self.iStart == other.iStart && self.cEntries == other.cEntries && self.aPalEntries == other.aPalEntries
    }
}
impl ::core::cmp::Eq for EMRSETPALETTEENTRIES {}
impl ::core::fmt::Debug for EMRSETPALETTEENTRIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETPALETTEENTRIES").field("emr", &self.emr).field("ihPal", &self.ihPal).field("iStart", &self.iStart).field("cEntries", &self.cEntries).field("aPalEntries", &self.aPalEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETPIXELV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETPIXELV {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlPixel == other.ptlPixel && self.crColor == other.crColor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETPIXELV {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETPIXELV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETPIXELV").field("emr", &self.emr).field("ptlPixel", &self.ptlPixel).field("crColor", &self.crColor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETTEXTCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETTEXTCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.crColor == other.crColor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETTEXTCOLOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETTEXTCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETTEXTCOLOR").field("emr", &self.emr).field("crColor", &self.crColor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETVIEWPORTEXTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETVIEWPORTEXTEX {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.szlExtent == other.szlExtent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETVIEWPORTEXTEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETVIEWPORTEXTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETVIEWPORTEXTEX").field("emr", &self.emr).field("szlExtent", &self.szlExtent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSETVIEWPORTORGEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSETVIEWPORTORGEX {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ptlOrigin == other.ptlOrigin
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSETVIEWPORTORGEX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSETVIEWPORTORGEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETVIEWPORTORGEX").field("emr", &self.emr).field("ptlOrigin", &self.ptlOrigin).finish()
    }
}
impl ::core::default::Default for EMRSETWORLDTRANSFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EMRSETWORLDTRANSFORM {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.xform == other.xform
    }
}
impl ::core::cmp::Eq for EMRSETWORLDTRANSFORM {}
impl ::core::fmt::Debug for EMRSETWORLDTRANSFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSETWORLDTRANSFORM").field("emr", &self.emr).field("xform", &self.xform).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSTRETCHBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSTRETCHBLT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.cxDest == other.cxDest && self.cyDest == other.cyDest && self.dwRop == other.dwRop && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.xformSrc == other.xformSrc && self.crBkColorSrc == other.crBkColorSrc && self.iUsageSrc == other.iUsageSrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.cxSrc == other.cxSrc && self.cySrc == other.cySrc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSTRETCHBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSTRETCHBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSTRETCHBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRSTRETCHDIBITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRSTRETCHDIBITS {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.cxSrc == other.cxSrc && self.cySrc == other.cySrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.iUsageSrc == other.iUsageSrc && self.dwRop == other.dwRop && self.cxDest == other.cxDest && self.cyDest == other.cyDest
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRSTRETCHDIBITS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRSTRETCHDIBITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRSTRETCHDIBITS")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("dwRop", &self.dwRop)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ptlReference == other.ptlReference && self.nChars == other.nChars && self.offString == other.offString && self.fOptions == other.fOptions && self.rcl == other.rcl && self.offDx == other.offDx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRTEXT").field("ptlReference", &self.ptlReference).field("nChars", &self.nChars).field("offString", &self.offString).field("fOptions", &self.fOptions).field("rcl", &self.rcl).field("offDx", &self.offDx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EMRTRANSPARENTBLT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EMRTRANSPARENTBLT {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.rclBounds == other.rclBounds && self.xDest == other.xDest && self.yDest == other.yDest && self.cxDest == other.cxDest && self.cyDest == other.cyDest && self.dwRop == other.dwRop && self.xSrc == other.xSrc && self.ySrc == other.ySrc && self.xformSrc == other.xformSrc && self.crBkColorSrc == other.crBkColorSrc && self.iUsageSrc == other.iUsageSrc && self.offBmiSrc == other.offBmiSrc && self.cbBmiSrc == other.cbBmiSrc && self.offBitsSrc == other.offBitsSrc && self.cbBitsSrc == other.cbBitsSrc && self.cxSrc == other.cxSrc && self.cySrc == other.cySrc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EMRTRANSPARENTBLT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EMRTRANSPARENTBLT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRTRANSPARENTBLT")
            .field("emr", &self.emr)
            .field("rclBounds", &self.rclBounds)
            .field("xDest", &self.xDest)
            .field("yDest", &self.yDest)
            .field("cxDest", &self.cxDest)
            .field("cyDest", &self.cyDest)
            .field("dwRop", &self.dwRop)
            .field("xSrc", &self.xSrc)
            .field("ySrc", &self.ySrc)
            .field("xformSrc", &self.xformSrc)
            .field("crBkColorSrc", &self.crBkColorSrc)
            .field("iUsageSrc", &self.iUsageSrc)
            .field("offBmiSrc", &self.offBmiSrc)
            .field("cbBmiSrc", &self.cbBmiSrc)
            .field("offBitsSrc", &self.offBitsSrc)
            .field("cbBitsSrc", &self.cbBitsSrc)
            .field("cxSrc", &self.cxSrc)
            .field("cySrc", &self.cySrc)
            .finish()
    }
}
impl ::core::default::Default for ENHANCED_METAFILE_RECORD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENHANCED_METAFILE_RECORD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENHANCED_METAFILE_RECORD_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENHMETAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENHMETAHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.iType == other.iType && self.nSize == other.nSize && self.rclBounds == other.rclBounds && self.rclFrame == other.rclFrame && self.dSignature == other.dSignature && self.nVersion == other.nVersion && self.nBytes == other.nBytes && self.nRecords == other.nRecords && self.nHandles == other.nHandles && self.sReserved == other.sReserved && self.nDescription == other.nDescription && self.offDescription == other.offDescription && self.nPalEntries == other.nPalEntries && self.szlDevice == other.szlDevice && self.szlMillimeters == other.szlMillimeters && self.cbPixelFormat == other.cbPixelFormat && self.offPixelFormat == other.offPixelFormat && self.bOpenGL == other.bOpenGL && self.szlMicrometers == other.szlMicrometers
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENHMETAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENHMETAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHMETAHEADER")
            .field("iType", &self.iType)
            .field("nSize", &self.nSize)
            .field("rclBounds", &self.rclBounds)
            .field("rclFrame", &self.rclFrame)
            .field("dSignature", &self.dSignature)
            .field("nVersion", &self.nVersion)
            .field("nBytes", &self.nBytes)
            .field("nRecords", &self.nRecords)
            .field("nHandles", &self.nHandles)
            .field("sReserved", &self.sReserved)
            .field("nDescription", &self.nDescription)
            .field("offDescription", &self.offDescription)
            .field("nPalEntries", &self.nPalEntries)
            .field("szlDevice", &self.szlDevice)
            .field("szlMillimeters", &self.szlMillimeters)
            .field("cbPixelFormat", &self.cbPixelFormat)
            .field("offPixelFormat", &self.offPixelFormat)
            .field("bOpenGL", &self.bOpenGL)
            .field("szlMicrometers", &self.szlMicrometers)
            .finish()
    }
}
impl ::core::default::Default for ENHMETARECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENHMETARECORD {
    fn eq(&self, other: &Self) -> bool {
        self.iType == other.iType && self.nSize == other.nSize && self.dParm == other.dParm
    }
}
impl ::core::cmp::Eq for ENHMETARECORD {}
impl ::core::fmt::Debug for ENHMETARECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENHMETARECORD").field("iType", &self.iType).field("nSize", &self.nSize).field("dParm", &self.dParm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMLOGFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMLOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont && self.elfFullName == other.elfFullName && self.elfStyle == other.elfStyle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMLOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTA").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMLOGFONTEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMLOGFONTEXA {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont && self.elfFullName == other.elfFullName && self.elfStyle == other.elfStyle && self.elfScript == other.elfScript
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMLOGFONTEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMLOGFONTEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXA").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfScript", &self.elfScript).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUMLOGFONTEXDVA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUMLOGFONTEXDVA {
    fn eq(&self, other: &Self) -> bool {
        self.elfEnumLogfontEx == other.elfEnumLogfontEx && self.elfDesignVector == other.elfDesignVector
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUMLOGFONTEXDVA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUMLOGFONTEXDVA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXDVA").field("elfEnumLogfontEx", &self.elfEnumLogfontEx).field("elfDesignVector", &self.elfDesignVector).finish()
    }
}
impl ::core::default::Default for ENUMLOGFONTEXDVW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXDVW {
    fn eq(&self, other: &Self) -> bool {
        self.elfEnumLogfontEx == other.elfEnumLogfontEx && self.elfDesignVector == other.elfDesignVector
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXDVW {}
impl ::core::fmt::Debug for ENUMLOGFONTEXDVW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXDVW").field("elfEnumLogfontEx", &self.elfEnumLogfontEx).field("elfDesignVector", &self.elfDesignVector).finish()
    }
}
impl ::core::default::Default for ENUMLOGFONTEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTEXW {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont && self.elfFullName == other.elfFullName && self.elfStyle == other.elfStyle && self.elfScript == other.elfScript
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTEXW {}
impl ::core::fmt::Debug for ENUMLOGFONTEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTEXW").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfScript", &self.elfScript).finish()
    }
}
impl ::core::default::Default for ENUMLOGFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENUMLOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont && self.elfFullName == other.elfFullName && self.elfStyle == other.elfStyle
    }
}
impl ::core::cmp::Eq for ENUMLOGFONTW {}
impl ::core::fmt::Debug for ENUMLOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMLOGFONTW").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).finish()
    }
}
impl ::core::default::Default for ENUM_DISPLAY_SETTINGS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_DISPLAY_SETTINGS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_DISPLAY_SETTINGS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ETO_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ETO_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETO_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ETO_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ETO_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ETO_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ETO_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ETO_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTLOGFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTLOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont && self.elfFullName == other.elfFullName && self.elfStyle == other.elfStyle && self.elfVersion == other.elfVersion && self.elfStyleSize == other.elfStyleSize && self.elfMatch == other.elfMatch && self.elfReserved == other.elfReserved && self.elfVendorId == other.elfVendorId && self.elfCulture == other.elfCulture && self.elfPanose == other.elfPanose
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTLOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTLOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGFONTA").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfVersion", &self.elfVersion).field("elfStyleSize", &self.elfStyleSize).field("elfMatch", &self.elfMatch).field("elfReserved", &self.elfReserved).field("elfVendorId", &self.elfVendorId).field("elfCulture", &self.elfCulture).field("elfPanose", &self.elfPanose).finish()
    }
}
impl ::core::default::Default for EXTLOGFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTLOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.elfLogFont == other.elfLogFont && self.elfFullName == other.elfFullName && self.elfStyle == other.elfStyle && self.elfVersion == other.elfVersion && self.elfStyleSize == other.elfStyleSize && self.elfMatch == other.elfMatch && self.elfReserved == other.elfReserved && self.elfVendorId == other.elfVendorId && self.elfCulture == other.elfCulture && self.elfPanose == other.elfPanose
    }
}
impl ::core::cmp::Eq for EXTLOGFONTW {}
impl ::core::fmt::Debug for EXTLOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGFONTW").field("elfLogFont", &self.elfLogFont).field("elfFullName", &self.elfFullName).field("elfStyle", &self.elfStyle).field("elfVersion", &self.elfVersion).field("elfStyleSize", &self.elfStyleSize).field("elfMatch", &self.elfMatch).field("elfReserved", &self.elfReserved).field("elfVendorId", &self.elfVendorId).field("elfCulture", &self.elfCulture).field("elfPanose", &self.elfPanose).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTLOGPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTLOGPEN {
    fn eq(&self, other: &Self) -> bool {
        self.elpPenStyle == other.elpPenStyle && self.elpWidth == other.elpWidth && self.elpBrushStyle == other.elpBrushStyle && self.elpColor == other.elpColor && self.elpHatch == other.elpHatch && self.elpNumEntries == other.elpNumEntries && self.elpStyleEntry == other.elpStyleEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTLOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTLOGPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGPEN").field("elpPenStyle", &self.elpPenStyle).field("elpWidth", &self.elpWidth).field("elpBrushStyle", &self.elpBrushStyle).field("elpColor", &self.elpColor).field("elpHatch", &self.elpHatch).field("elpNumEntries", &self.elpNumEntries).field("elpStyleEntry", &self.elpStyleEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EXTLOGPEN32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EXTLOGPEN32 {
    fn eq(&self, other: &Self) -> bool {
        self.elpPenStyle == other.elpPenStyle && self.elpWidth == other.elpWidth && self.elpBrushStyle == other.elpBrushStyle && self.elpColor == other.elpColor && self.elpHatch == other.elpHatch && self.elpNumEntries == other.elpNumEntries && self.elpStyleEntry == other.elpStyleEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EXTLOGPEN32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EXTLOGPEN32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTLOGPEN32").field("elpPenStyle", &self.elpPenStyle).field("elpWidth", &self.elpWidth).field("elpBrushStyle", &self.elpBrushStyle).field("elpColor", &self.elpColor).field("elpHatch", &self.elpHatch).field("elpNumEntries", &self.elpNumEntries).field("elpStyleEntry", &self.elpStyleEntry).finish()
    }
}
impl ::core::default::Default for EXT_FLOOD_FILL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EXT_FLOOD_FILL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXT_FLOOD_FILL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FIXED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FIXED {
    fn eq(&self, other: &Self) -> bool {
        self.fract == other.fract && self.value == other.value
    }
}
impl ::core::cmp::Eq for FIXED {}
impl ::core::fmt::Debug for FIXED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIXED").field("fract", &self.fract).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for FONT_CHARSET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_CHARSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_CHARSET").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_CLIP_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_CLIP_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_CLIP_PRECISION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FONT_CLIP_PRECISION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FONT_CLIP_PRECISION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FONT_CLIP_PRECISION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FONT_CLIP_PRECISION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FONT_FAMILY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_FAMILY").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_LICENSE_PRIVS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_LICENSE_PRIVS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_LICENSE_PRIVS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_OUTPUT_PRECISION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_OUTPUT_PRECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_OUTPUT_PRECISION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_PITCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_PITCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_PITCH").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_QUALITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_QUALITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_RESOURCE_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_RESOURCE_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_RESOURCE_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FONT_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FONT_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FONT_WEIGHT").field(&self.0).finish()
    }
}
impl ::core::default::Default for GCP_RESULTSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GCP_RESULTSA {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize && self.lpOutString == other.lpOutString && self.lpOrder == other.lpOrder && self.lpDx == other.lpDx && self.lpCaretPos == other.lpCaretPos && self.lpClass == other.lpClass && self.lpGlyphs == other.lpGlyphs && self.nGlyphs == other.nGlyphs && self.nMaxFit == other.nMaxFit
    }
}
impl ::core::cmp::Eq for GCP_RESULTSA {}
impl ::core::fmt::Debug for GCP_RESULTSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GCP_RESULTSA").field("lStructSize", &self.lStructSize).field("lpOutString", &self.lpOutString).field("lpOrder", &self.lpOrder).field("lpDx", &self.lpDx).field("lpCaretPos", &self.lpCaretPos).field("lpClass", &self.lpClass).field("lpGlyphs", &self.lpGlyphs).field("nGlyphs", &self.nGlyphs).field("nMaxFit", &self.nMaxFit).finish()
    }
}
impl ::core::default::Default for GCP_RESULTSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GCP_RESULTSW {
    fn eq(&self, other: &Self) -> bool {
        self.lStructSize == other.lStructSize && self.lpOutString == other.lpOutString && self.lpOrder == other.lpOrder && self.lpDx == other.lpDx && self.lpCaretPos == other.lpCaretPos && self.lpClass == other.lpClass && self.lpGlyphs == other.lpGlyphs && self.nGlyphs == other.nGlyphs && self.nMaxFit == other.nMaxFit
    }
}
impl ::core::cmp::Eq for GCP_RESULTSW {}
impl ::core::fmt::Debug for GCP_RESULTSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GCP_RESULTSW").field("lStructSize", &self.lStructSize).field("lpOutString", &self.lpOutString).field("lpOrder", &self.lpOrder).field("lpDx", &self.lpDx).field("lpCaretPos", &self.lpCaretPos).field("lpClass", &self.lpClass).field("lpGlyphs", &self.lpGlyphs).field("nGlyphs", &self.nGlyphs).field("nMaxFit", &self.nMaxFit).finish()
    }
}
impl ::core::default::Default for GDI_REGION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GDI_REGION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GDI_REGION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_CHARACTER_PLACEMENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_CHARACTER_PLACEMENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CHARACTER_PLACEMENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_CHARACTER_PLACEMENT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_CHARACTER_PLACEMENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GET_DCX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_DCX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DCX_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_DCX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_DCX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_DCX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_DCX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_DCX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GET_DEVICE_CAPS_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_DEVICE_CAPS_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_DEVICE_CAPS_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_GLYPH_OUTLINE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_GLYPH_OUTLINE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GLYPH_OUTLINE_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for GET_STOCK_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_STOCK_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_STOCK_OBJECT_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLYPHMETRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for GLYPHMETRICS {
    fn eq(&self, other: &Self) -> bool {
        self.gmBlackBoxX == other.gmBlackBoxX && self.gmBlackBoxY == other.gmBlackBoxY && self.gmptGlyphOrigin == other.gmptGlyphOrigin && self.gmCellIncX == other.gmCellIncX && self.gmCellIncY == other.gmCellIncY
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for GLYPHMETRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for GLYPHMETRICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHMETRICS").field("gmBlackBoxX", &self.gmBlackBoxX).field("gmBlackBoxY", &self.gmBlackBoxY).field("gmptGlyphOrigin", &self.gmptGlyphOrigin).field("gmCellIncX", &self.gmCellIncX).field("gmCellIncY", &self.gmCellIncY).finish()
    }
}
impl ::core::default::Default for GLYPHSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GLYPHSET {
    fn eq(&self, other: &Self) -> bool {
        self.cbThis == other.cbThis && self.flAccel == other.flAccel && self.cGlyphsSupported == other.cGlyphsSupported && self.cRanges == other.cRanges && self.ranges == other.ranges
    }
}
impl ::core::cmp::Eq for GLYPHSET {}
impl ::core::fmt::Debug for GLYPHSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GLYPHSET").field("cbThis", &self.cbThis).field("flAccel", &self.flAccel).field("cGlyphsSupported", &self.cGlyphsSupported).field("cRanges", &self.cRanges).field("ranges", &self.ranges).finish()
    }
}
impl ::core::default::Default for GRADIENT_FILL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRADIENT_FILL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRADIENT_FILL").field(&self.0).finish()
    }
}
impl ::core::default::Default for GRADIENT_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GRADIENT_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.UpperLeft == other.UpperLeft && self.LowerRight == other.LowerRight
    }
}
impl ::core::cmp::Eq for GRADIENT_RECT {}
impl ::core::fmt::Debug for GRADIENT_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRADIENT_RECT").field("UpperLeft", &self.UpperLeft).field("LowerRight", &self.LowerRight).finish()
    }
}
impl ::core::default::Default for GRADIENT_TRIANGLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GRADIENT_TRIANGLE {
    fn eq(&self, other: &Self) -> bool {
        self.Vertex1 == other.Vertex1 && self.Vertex2 == other.Vertex2 && self.Vertex3 == other.Vertex3
    }
}
impl ::core::cmp::Eq for GRADIENT_TRIANGLE {}
impl ::core::fmt::Debug for GRADIENT_TRIANGLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRADIENT_TRIANGLE").field("Vertex1", &self.Vertex1).field("Vertex2", &self.Vertex2).field("Vertex3", &self.Vertex3).finish()
    }
}
impl ::core::default::Default for GRAPHICS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GRAPHICS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRAPHICS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HANDLETABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HANDLETABLE {
    fn eq(&self, other: &Self) -> bool {
        self.objectHandle == other.objectHandle
    }
}
impl ::core::cmp::Eq for HANDLETABLE {}
impl ::core::fmt::Debug for HANDLETABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HANDLETABLE").field("objectHandle", &self.objectHandle).finish()
    }
}
impl ::core::default::Default for HATCH_BRUSH_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HATCH_BRUSH_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HATCH_BRUSH_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDC_MAP_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDC_MAP_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDC_MAP_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for KERNINGPAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KERNINGPAIR {
    fn eq(&self, other: &Self) -> bool {
        self.wFirst == other.wFirst && self.wSecond == other.wSecond && self.iKernAmount == other.iKernAmount
    }
}
impl ::core::cmp::Eq for KERNINGPAIR {}
impl ::core::fmt::Debug for KERNINGPAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KERNINGPAIR").field("wFirst", &self.wFirst).field("wSecond", &self.wSecond).field("iKernAmount", &self.iKernAmount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGBRUSH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGBRUSH {
    fn eq(&self, other: &Self) -> bool {
        self.lbStyle == other.lbStyle && self.lbColor == other.lbColor && self.lbHatch == other.lbHatch
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGBRUSH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGBRUSH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGBRUSH").field("lbStyle", &self.lbStyle).field("lbColor", &self.lbColor).field("lbHatch", &self.lbHatch).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGBRUSH32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGBRUSH32 {
    fn eq(&self, other: &Self) -> bool {
        self.lbStyle == other.lbStyle && self.lbColor == other.lbColor && self.lbHatch == other.lbHatch
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGBRUSH32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGBRUSH32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGBRUSH32").field("lbStyle", &self.lbStyle).field("lbColor", &self.lbColor).field("lbHatch", &self.lbHatch).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGFONTA {
    fn eq(&self, other: &Self) -> bool {
        self.lfHeight == other.lfHeight && self.lfWidth == other.lfWidth && self.lfEscapement == other.lfEscapement && self.lfOrientation == other.lfOrientation && self.lfWeight == other.lfWeight && self.lfItalic == other.lfItalic && self.lfUnderline == other.lfUnderline && self.lfStrikeOut == other.lfStrikeOut && self.lfCharSet == other.lfCharSet && self.lfOutPrecision == other.lfOutPrecision && self.lfClipPrecision == other.lfClipPrecision && self.lfQuality == other.lfQuality && self.lfPitchAndFamily == other.lfPitchAndFamily && self.lfFaceName == other.lfFaceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGFONTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGFONTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGFONTA")
            .field("lfHeight", &self.lfHeight)
            .field("lfWidth", &self.lfWidth)
            .field("lfEscapement", &self.lfEscapement)
            .field("lfOrientation", &self.lfOrientation)
            .field("lfWeight", &self.lfWeight)
            .field("lfItalic", &self.lfItalic)
            .field("lfUnderline", &self.lfUnderline)
            .field("lfStrikeOut", &self.lfStrikeOut)
            .field("lfCharSet", &self.lfCharSet)
            .field("lfOutPrecision", &self.lfOutPrecision)
            .field("lfClipPrecision", &self.lfClipPrecision)
            .field("lfQuality", &self.lfQuality)
            .field("lfPitchAndFamily", &self.lfPitchAndFamily)
            .field("lfFaceName", &self.lfFaceName)
            .finish()
    }
}
impl ::core::default::Default for LOGFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOGFONTW {
    fn eq(&self, other: &Self) -> bool {
        self.lfHeight == other.lfHeight && self.lfWidth == other.lfWidth && self.lfEscapement == other.lfEscapement && self.lfOrientation == other.lfOrientation && self.lfWeight == other.lfWeight && self.lfItalic == other.lfItalic && self.lfUnderline == other.lfUnderline && self.lfStrikeOut == other.lfStrikeOut && self.lfCharSet == other.lfCharSet && self.lfOutPrecision == other.lfOutPrecision && self.lfClipPrecision == other.lfClipPrecision && self.lfQuality == other.lfQuality && self.lfPitchAndFamily == other.lfPitchAndFamily && self.lfFaceName == other.lfFaceName
    }
}
impl ::core::cmp::Eq for LOGFONTW {}
impl ::core::fmt::Debug for LOGFONTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGFONTW")
            .field("lfHeight", &self.lfHeight)
            .field("lfWidth", &self.lfWidth)
            .field("lfEscapement", &self.lfEscapement)
            .field("lfOrientation", &self.lfOrientation)
            .field("lfWeight", &self.lfWeight)
            .field("lfItalic", &self.lfItalic)
            .field("lfUnderline", &self.lfUnderline)
            .field("lfStrikeOut", &self.lfStrikeOut)
            .field("lfCharSet", &self.lfCharSet)
            .field("lfOutPrecision", &self.lfOutPrecision)
            .field("lfClipPrecision", &self.lfClipPrecision)
            .field("lfQuality", &self.lfQuality)
            .field("lfPitchAndFamily", &self.lfPitchAndFamily)
            .field("lfFaceName", &self.lfFaceName)
            .finish()
    }
}
impl ::core::default::Default for LOGPALETTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOGPALETTE {
    fn eq(&self, other: &Self) -> bool {
        self.palVersion == other.palVersion && self.palNumEntries == other.palNumEntries && self.palPalEntry == other.palPalEntry
    }
}
impl ::core::cmp::Eq for LOGPALETTE {}
impl ::core::fmt::Debug for LOGPALETTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGPALETTE").field("palVersion", &self.palVersion).field("palNumEntries", &self.palNumEntries).field("palPalEntry", &self.palPalEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LOGPEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LOGPEN {
    fn eq(&self, other: &Self) -> bool {
        self.lopnStyle == other.lopnStyle && self.lopnWidth == other.lopnWidth && self.lopnColor == other.lopnColor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LOGPEN {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LOGPEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGPEN").field("lopnStyle", &self.lopnStyle).field("lopnWidth", &self.lopnWidth).field("lopnColor", &self.lopnColor).finish()
    }
}
impl ::core::default::Default for MAT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MAT2 {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22
    }
}
impl ::core::cmp::Eq for MAT2 {}
impl ::core::fmt::Debug for MAT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MAT2").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).finish()
    }
}
impl ::core::default::Default for METAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for METARECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for METARECORD {
    fn eq(&self, other: &Self) -> bool {
        self.rdSize == other.rdSize && self.rdFunction == other.rdFunction && self.rdParm == other.rdParm
    }
}
impl ::core::cmp::Eq for METARECORD {}
impl ::core::fmt::Debug for METARECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("METARECORD").field("rdSize", &self.rdSize).field("rdFunction", &self.rdFunction).field("rdParm", &self.rdParm).finish()
    }
}
impl ::core::default::Default for MODIFY_WORLD_TRANSFORM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MODIFY_WORLD_TRANSFORM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MODIFY_WORLD_TRANSFORM_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.rcMonitor == other.rcMonitor && self.rcWork == other.rcWork && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITORINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFO").field("cbSize", &self.cbSize).field("rcMonitor", &self.rcMonitor).field("rcWork", &self.rcWork).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITORINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITORINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        self.monitorInfo == other.monitorInfo && self.szDevice == other.szDevice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITORINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITORINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFOEXA").field("monitorInfo", &self.monitorInfo).field("szDevice", &self.szDevice).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MONITORINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MONITORINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        self.monitorInfo == other.monitorInfo && self.szDevice == other.szDevice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MONITORINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MONITORINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MONITORINFOEXW").field("monitorInfo", &self.monitorInfo).field("szDevice", &self.szDevice).finish()
    }
}
impl ::core::default::Default for MONITOR_FROM_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MONITOR_FROM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MONITOR_FROM_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for NEWTEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
            && self.ntmFlags == other.ntmFlags
            && self.ntmSizeEM == other.ntmSizeEM
            && self.ntmCellHeight == other.ntmCellHeight
            && self.ntmAvgWidth == other.ntmAvgWidth
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICA {}
impl ::core::fmt::Debug for NEWTEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICA")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .field("ntmFlags", &self.ntmFlags)
            .field("ntmSizeEM", &self.ntmSizeEM)
            .field("ntmCellHeight", &self.ntmCellHeight)
            .field("ntmAvgWidth", &self.ntmAvgWidth)
            .finish()
    }
}
impl ::core::default::Default for NEWTEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NEWTEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
            && self.ntmFlags == other.ntmFlags
            && self.ntmSizeEM == other.ntmSizeEM
            && self.ntmCellHeight == other.ntmCellHeight
            && self.ntmAvgWidth == other.ntmAvgWidth
    }
}
impl ::core::cmp::Eq for NEWTEXTMETRICW {}
impl ::core::fmt::Debug for NEWTEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NEWTEXTMETRICW")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .field("ntmFlags", &self.ntmFlags)
            .field("ntmSizeEM", &self.ntmSizeEM)
            .field("ntmCellHeight", &self.ntmCellHeight)
            .field("ntmAvgWidth", &self.ntmAvgWidth)
            .finish()
    }
}
impl ::core::default::Default for OBJ_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OBJ_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OBJ_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OUTLINETEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OUTLINETEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.otmSize == other.otmSize
            && self.otmTextMetrics == other.otmTextMetrics
            && self.otmFiller == other.otmFiller
            && self.otmPanoseNumber == other.otmPanoseNumber
            && self.otmfsSelection == other.otmfsSelection
            && self.otmfsType == other.otmfsType
            && self.otmsCharSlopeRise == other.otmsCharSlopeRise
            && self.otmsCharSlopeRun == other.otmsCharSlopeRun
            && self.otmItalicAngle == other.otmItalicAngle
            && self.otmEMSquare == other.otmEMSquare
            && self.otmAscent == other.otmAscent
            && self.otmDescent == other.otmDescent
            && self.otmLineGap == other.otmLineGap
            && self.otmsCapEmHeight == other.otmsCapEmHeight
            && self.otmsXHeight == other.otmsXHeight
            && self.otmrcFontBox == other.otmrcFontBox
            && self.otmMacAscent == other.otmMacAscent
            && self.otmMacDescent == other.otmMacDescent
            && self.otmMacLineGap == other.otmMacLineGap
            && self.otmusMinimumPPEM == other.otmusMinimumPPEM
            && self.otmptSubscriptSize == other.otmptSubscriptSize
            && self.otmptSubscriptOffset == other.otmptSubscriptOffset
            && self.otmptSuperscriptSize == other.otmptSuperscriptSize
            && self.otmptSuperscriptOffset == other.otmptSuperscriptOffset
            && self.otmsStrikeoutSize == other.otmsStrikeoutSize
            && self.otmsStrikeoutPosition == other.otmsStrikeoutPosition
            && self.otmsUnderscoreSize == other.otmsUnderscoreSize
            && self.otmsUnderscorePosition == other.otmsUnderscorePosition
            && self.otmpFamilyName == other.otmpFamilyName
            && self.otmpFaceName == other.otmpFaceName
            && self.otmpStyleName == other.otmpStyleName
            && self.otmpFullName == other.otmpFullName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OUTLINETEXTMETRICA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OUTLINETEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTLINETEXTMETRICA")
            .field("otmSize", &self.otmSize)
            .field("otmTextMetrics", &self.otmTextMetrics)
            .field("otmFiller", &self.otmFiller)
            .field("otmPanoseNumber", &self.otmPanoseNumber)
            .field("otmfsSelection", &self.otmfsSelection)
            .field("otmfsType", &self.otmfsType)
            .field("otmsCharSlopeRise", &self.otmsCharSlopeRise)
            .field("otmsCharSlopeRun", &self.otmsCharSlopeRun)
            .field("otmItalicAngle", &self.otmItalicAngle)
            .field("otmEMSquare", &self.otmEMSquare)
            .field("otmAscent", &self.otmAscent)
            .field("otmDescent", &self.otmDescent)
            .field("otmLineGap", &self.otmLineGap)
            .field("otmsCapEmHeight", &self.otmsCapEmHeight)
            .field("otmsXHeight", &self.otmsXHeight)
            .field("otmrcFontBox", &self.otmrcFontBox)
            .field("otmMacAscent", &self.otmMacAscent)
            .field("otmMacDescent", &self.otmMacDescent)
            .field("otmMacLineGap", &self.otmMacLineGap)
            .field("otmusMinimumPPEM", &self.otmusMinimumPPEM)
            .field("otmptSubscriptSize", &self.otmptSubscriptSize)
            .field("otmptSubscriptOffset", &self.otmptSubscriptOffset)
            .field("otmptSuperscriptSize", &self.otmptSuperscriptSize)
            .field("otmptSuperscriptOffset", &self.otmptSuperscriptOffset)
            .field("otmsStrikeoutSize", &self.otmsStrikeoutSize)
            .field("otmsStrikeoutPosition", &self.otmsStrikeoutPosition)
            .field("otmsUnderscoreSize", &self.otmsUnderscoreSize)
            .field("otmsUnderscorePosition", &self.otmsUnderscorePosition)
            .field("otmpFamilyName", &self.otmpFamilyName)
            .field("otmpFaceName", &self.otmpFaceName)
            .field("otmpStyleName", &self.otmpStyleName)
            .field("otmpFullName", &self.otmpFullName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OUTLINETEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for OUTLINETEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.otmSize == other.otmSize
            && self.otmTextMetrics == other.otmTextMetrics
            && self.otmFiller == other.otmFiller
            && self.otmPanoseNumber == other.otmPanoseNumber
            && self.otmfsSelection == other.otmfsSelection
            && self.otmfsType == other.otmfsType
            && self.otmsCharSlopeRise == other.otmsCharSlopeRise
            && self.otmsCharSlopeRun == other.otmsCharSlopeRun
            && self.otmItalicAngle == other.otmItalicAngle
            && self.otmEMSquare == other.otmEMSquare
            && self.otmAscent == other.otmAscent
            && self.otmDescent == other.otmDescent
            && self.otmLineGap == other.otmLineGap
            && self.otmsCapEmHeight == other.otmsCapEmHeight
            && self.otmsXHeight == other.otmsXHeight
            && self.otmrcFontBox == other.otmrcFontBox
            && self.otmMacAscent == other.otmMacAscent
            && self.otmMacDescent == other.otmMacDescent
            && self.otmMacLineGap == other.otmMacLineGap
            && self.otmusMinimumPPEM == other.otmusMinimumPPEM
            && self.otmptSubscriptSize == other.otmptSubscriptSize
            && self.otmptSubscriptOffset == other.otmptSubscriptOffset
            && self.otmptSuperscriptSize == other.otmptSuperscriptSize
            && self.otmptSuperscriptOffset == other.otmptSuperscriptOffset
            && self.otmsStrikeoutSize == other.otmsStrikeoutSize
            && self.otmsStrikeoutPosition == other.otmsStrikeoutPosition
            && self.otmsUnderscoreSize == other.otmsUnderscoreSize
            && self.otmsUnderscorePosition == other.otmsUnderscorePosition
            && self.otmpFamilyName == other.otmpFamilyName
            && self.otmpFaceName == other.otmpFaceName
            && self.otmpStyleName == other.otmpStyleName
            && self.otmpFullName == other.otmpFullName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for OUTLINETEXTMETRICW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for OUTLINETEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OUTLINETEXTMETRICW")
            .field("otmSize", &self.otmSize)
            .field("otmTextMetrics", &self.otmTextMetrics)
            .field("otmFiller", &self.otmFiller)
            .field("otmPanoseNumber", &self.otmPanoseNumber)
            .field("otmfsSelection", &self.otmfsSelection)
            .field("otmfsType", &self.otmfsType)
            .field("otmsCharSlopeRise", &self.otmsCharSlopeRise)
            .field("otmsCharSlopeRun", &self.otmsCharSlopeRun)
            .field("otmItalicAngle", &self.otmItalicAngle)
            .field("otmEMSquare", &self.otmEMSquare)
            .field("otmAscent", &self.otmAscent)
            .field("otmDescent", &self.otmDescent)
            .field("otmLineGap", &self.otmLineGap)
            .field("otmsCapEmHeight", &self.otmsCapEmHeight)
            .field("otmsXHeight", &self.otmsXHeight)
            .field("otmrcFontBox", &self.otmrcFontBox)
            .field("otmMacAscent", &self.otmMacAscent)
            .field("otmMacDescent", &self.otmMacDescent)
            .field("otmMacLineGap", &self.otmMacLineGap)
            .field("otmusMinimumPPEM", &self.otmusMinimumPPEM)
            .field("otmptSubscriptSize", &self.otmptSubscriptSize)
            .field("otmptSubscriptOffset", &self.otmptSubscriptOffset)
            .field("otmptSuperscriptSize", &self.otmptSuperscriptSize)
            .field("otmptSuperscriptOffset", &self.otmptSuperscriptOffset)
            .field("otmsStrikeoutSize", &self.otmsStrikeoutSize)
            .field("otmsStrikeoutPosition", &self.otmsStrikeoutPosition)
            .field("otmsUnderscoreSize", &self.otmsUnderscoreSize)
            .field("otmsUnderscorePosition", &self.otmsUnderscorePosition)
            .field("otmpFamilyName", &self.otmpFamilyName)
            .field("otmpFaceName", &self.otmpFaceName)
            .field("otmpStyleName", &self.otmpStyleName)
            .field("otmpFullName", &self.otmpFullName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAINTSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PAINTSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hdc == other.hdc && self.fErase == other.fErase && self.rcPaint == other.rcPaint && self.fRestore == other.fRestore && self.fIncUpdate == other.fIncUpdate && self.rgbReserved == other.rgbReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PAINTSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PAINTSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PAINTSTRUCT").field("hdc", &self.hdc).field("fErase", &self.fErase).field("rcPaint", &self.rcPaint).field("fRestore", &self.fRestore).field("fIncUpdate", &self.fIncUpdate).field("rgbReserved", &self.rgbReserved).finish()
    }
}
impl ::core::default::Default for PALETTEENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PALETTEENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.peRed == other.peRed && self.peGreen == other.peGreen && self.peBlue == other.peBlue && self.peFlags == other.peFlags
    }
}
impl ::core::cmp::Eq for PALETTEENTRY {}
impl ::core::fmt::Debug for PALETTEENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PALETTEENTRY").field("peRed", &self.peRed).field("peGreen", &self.peGreen).field("peBlue", &self.peBlue).field("peFlags", &self.peFlags).finish()
    }
}
impl ::core::default::Default for PANOSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PANOSE {
    fn eq(&self, other: &Self) -> bool {
        self.bFamilyType == other.bFamilyType && self.bSerifStyle == other.bSerifStyle && self.bWeight == other.bWeight && self.bProportion == other.bProportion && self.bContrast == other.bContrast && self.bStrokeVariation == other.bStrokeVariation && self.bArmStyle == other.bArmStyle && self.bLetterform == other.bLetterform && self.bMidline == other.bMidline && self.bXHeight == other.bXHeight
    }
}
impl ::core::cmp::Eq for PANOSE {}
impl ::core::fmt::Debug for PANOSE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PANOSE").field("bFamilyType", &self.bFamilyType).field("bSerifStyle", &self.bSerifStyle).field("bWeight", &self.bWeight).field("bProportion", &self.bProportion).field("bContrast", &self.bContrast).field("bStrokeVariation", &self.bStrokeVariation).field("bArmStyle", &self.bArmStyle).field("bLetterform", &self.bLetterform).field("bMidline", &self.bMidline).field("bXHeight", &self.bXHeight).finish()
    }
}
impl ::core::default::Default for PAN_ARM_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_ARM_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_ARM_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_CONTRAST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_CONTRAST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_CONTRAST").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_FAMILY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_FAMILY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_FAMILY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_LETT_FORM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_LETT_FORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_LETT_FORM").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_MIDLINE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_MIDLINE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_MIDLINE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_PROPORTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_PROPORTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_PROPORTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_SERIF_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_SERIF_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_SERIF_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_STROKE_VARIATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_STROKE_VARIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_STROKE_VARIATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_WEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_WEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_WEIGHT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PAN_XHEIGHT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAN_XHEIGHT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAN_XHEIGHT").field(&self.0).finish()
    }
}
impl ::core::default::Default for PELARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PELARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.paXCount == other.paXCount && self.paYCount == other.paYCount && self.paXExt == other.paXExt && self.paYExt == other.paYExt && self.paRGBs == other.paRGBs
    }
}
impl ::core::cmp::Eq for PELARRAY {}
impl ::core::fmt::Debug for PELARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PELARRAY").field("paXCount", &self.paXCount).field("paYCount", &self.paYCount).field("paXExt", &self.paXExt).field("paYExt", &self.paYExt).field("paRGBs", &self.paRGBs).finish()
    }
}
impl ::core::default::Default for PEN_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PEN_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PEN_STYLE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PEN_STYLE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PEN_STYLE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PEN_STYLE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PEN_STYLE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PEN_STYLE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for POINTFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for POINTFX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for POINTFX {}
impl ::core::fmt::Debug for POINTFX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POINTFX").field("x", &self.x).field("y", &self.y).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLYTEXTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLYTEXTA {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.n == other.n && self.lpstr == other.lpstr && self.uiFlags == other.uiFlags && self.rcl == other.rcl && self.pdx == other.pdx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLYTEXTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLYTEXTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLYTEXTA").field("x", &self.x).field("y", &self.y).field("n", &self.n).field("lpstr", &self.lpstr).field("uiFlags", &self.uiFlags).field("rcl", &self.rcl).field("pdx", &self.pdx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for POLYTEXTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for POLYTEXTW {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.n == other.n && self.lpstr == other.lpstr && self.uiFlags == other.uiFlags && self.rcl == other.rcl && self.pdx == other.pdx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for POLYTEXTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for POLYTEXTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("POLYTEXTW").field("x", &self.x).field("y", &self.y).field("n", &self.n).field("lpstr", &self.lpstr).field("uiFlags", &self.uiFlags).field("rcl", &self.rcl).field("pdx", &self.pdx).finish()
    }
}
impl ::core::default::Default for R2_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for R2_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("R2_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for RASTERIZER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RASTERIZER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.nSize == other.nSize && self.wFlags == other.wFlags && self.nLanguageID == other.nLanguageID
    }
}
impl ::core::cmp::Eq for RASTERIZER_STATUS {}
impl ::core::fmt::Debug for RASTERIZER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASTERIZER_STATUS").field("nSize", &self.nSize).field("wFlags", &self.wFlags).field("nLanguageID", &self.nLanguageID).finish()
    }
}
impl ::core::default::Default for REDRAW_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REDRAW_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REDRAW_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for REDRAW_WINDOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for REDRAW_WINDOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for REDRAW_WINDOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for RGBQUAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RGBQUAD {
    fn eq(&self, other: &Self) -> bool {
        self.rgbBlue == other.rgbBlue && self.rgbGreen == other.rgbGreen && self.rgbRed == other.rgbRed && self.rgbReserved == other.rgbReserved
    }
}
impl ::core::cmp::Eq for RGBQUAD {}
impl ::core::fmt::Debug for RGBQUAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBQUAD").field("rgbBlue", &self.rgbBlue).field("rgbGreen", &self.rgbGreen).field("rgbRed", &self.rgbRed).field("rgbReserved", &self.rgbReserved).finish()
    }
}
impl ::core::default::Default for RGBTRIPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RGBTRIPLE {
    fn eq(&self, other: &Self) -> bool {
        self.rgbtBlue == other.rgbtBlue && self.rgbtGreen == other.rgbtGreen && self.rgbtRed == other.rgbtRed
    }
}
impl ::core::cmp::Eq for RGBTRIPLE {}
impl ::core::fmt::Debug for RGBTRIPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBTRIPLE").field("rgbtBlue", &self.rgbtBlue).field("rgbtGreen", &self.rgbtGreen).field("rgbtRed", &self.rgbtRed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RGNDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RGNDATA {
    fn eq(&self, other: &Self) -> bool {
        self.rdh == other.rdh && self.Buffer == other.Buffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RGNDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RGNDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNDATA").field("rdh", &self.rdh).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RGNDATAHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RGNDATAHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.iType == other.iType && self.nCount == other.nCount && self.nRgnSize == other.nRgnSize && self.rcBound == other.rcBound
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RGNDATAHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RGNDATAHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGNDATAHEADER").field("dwSize", &self.dwSize).field("iType", &self.iType).field("nCount", &self.nCount).field("nRgnSize", &self.nRgnSize).field("rcBound", &self.rcBound).finish()
    }
}
impl ::core::default::Default for RGN_COMBINE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RGN_COMBINE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RGN_COMBINE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ROP_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ROP_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROP_CODE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ROP_CODE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ROP_CODE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ROP_CODE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ROP_CODE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ROP_CODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for SET_BOUNDS_RECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SET_BOUNDS_RECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SET_BOUNDS_RECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STRETCH_BLT_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRETCH_BLT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRETCH_BLT_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_PALETTE_USE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_PALETTE_USE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_PALETTE_USE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYS_COLOR_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYS_COLOR_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYS_COLOR_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXTMETRICA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TEXTMETRICA {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
    }
}
impl ::core::cmp::Eq for TEXTMETRICA {}
impl ::core::fmt::Debug for TEXTMETRICA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTMETRICA")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .finish()
    }
}
impl ::core::default::Default for TEXTMETRICW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TEXTMETRICW {
    fn eq(&self, other: &Self) -> bool {
        self.tmHeight == other.tmHeight
            && self.tmAscent == other.tmAscent
            && self.tmDescent == other.tmDescent
            && self.tmInternalLeading == other.tmInternalLeading
            && self.tmExternalLeading == other.tmExternalLeading
            && self.tmAveCharWidth == other.tmAveCharWidth
            && self.tmMaxCharWidth == other.tmMaxCharWidth
            && self.tmWeight == other.tmWeight
            && self.tmOverhang == other.tmOverhang
            && self.tmDigitizedAspectX == other.tmDigitizedAspectX
            && self.tmDigitizedAspectY == other.tmDigitizedAspectY
            && self.tmFirstChar == other.tmFirstChar
            && self.tmLastChar == other.tmLastChar
            && self.tmDefaultChar == other.tmDefaultChar
            && self.tmBreakChar == other.tmBreakChar
            && self.tmItalic == other.tmItalic
            && self.tmUnderlined == other.tmUnderlined
            && self.tmStruckOut == other.tmStruckOut
            && self.tmPitchAndFamily == other.tmPitchAndFamily
            && self.tmCharSet == other.tmCharSet
    }
}
impl ::core::cmp::Eq for TEXTMETRICW {}
impl ::core::fmt::Debug for TEXTMETRICW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEXTMETRICW")
            .field("tmHeight", &self.tmHeight)
            .field("tmAscent", &self.tmAscent)
            .field("tmDescent", &self.tmDescent)
            .field("tmInternalLeading", &self.tmInternalLeading)
            .field("tmExternalLeading", &self.tmExternalLeading)
            .field("tmAveCharWidth", &self.tmAveCharWidth)
            .field("tmMaxCharWidth", &self.tmMaxCharWidth)
            .field("tmWeight", &self.tmWeight)
            .field("tmOverhang", &self.tmOverhang)
            .field("tmDigitizedAspectX", &self.tmDigitizedAspectX)
            .field("tmDigitizedAspectY", &self.tmDigitizedAspectY)
            .field("tmFirstChar", &self.tmFirstChar)
            .field("tmLastChar", &self.tmLastChar)
            .field("tmDefaultChar", &self.tmDefaultChar)
            .field("tmBreakChar", &self.tmBreakChar)
            .field("tmItalic", &self.tmItalic)
            .field("tmUnderlined", &self.tmUnderlined)
            .field("tmStruckOut", &self.tmStruckOut)
            .field("tmPitchAndFamily", &self.tmPitchAndFamily)
            .field("tmCharSet", &self.tmCharSet)
            .finish()
    }
}
impl ::core::default::Default for TEXT_ALIGN_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TEXT_ALIGN_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEXT_ALIGN_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TEXT_ALIGN_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TEXT_ALIGN_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TEXT_ALIGN_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TMPF_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TMPF_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TMPF_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TMPF_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TMPF_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TMPF_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TMPF_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TMPF_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TRIVERTEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TRIVERTEX {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.Red == other.Red && self.Green == other.Green && self.Blue == other.Blue && self.Alpha == other.Alpha
    }
}
impl ::core::cmp::Eq for TRIVERTEX {}
impl ::core::fmt::Debug for TRIVERTEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRIVERTEX").field("x", &self.x).field("y", &self.y).field("Red", &self.Red).field("Green", &self.Green).field("Blue", &self.Blue).field("Alpha", &self.Alpha).finish()
    }
}
impl ::core::default::Default for TTEMBEDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTEMBEDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.usStructSize == other.usStructSize && self.usRootStrSize == other.usRootStrSize && self.pusRootStr == other.pusRootStr
    }
}
impl ::core::cmp::Eq for TTEMBEDINFO {}
impl ::core::fmt::Debug for TTEMBEDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTEMBEDINFO").field("usStructSize", &self.usStructSize).field("usRootStrSize", &self.usRootStrSize).field("pusRootStr", &self.pusRootStr).finish()
    }
}
impl ::core::default::Default for TTEMBED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TTEMBED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTEMBED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TTEMBED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTEMBED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTEMBED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTEMBED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTEMBED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TTLOADINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTLOADINFO {
    fn eq(&self, other: &Self) -> bool {
        self.usStructSize == other.usStructSize && self.usRefStrSize == other.usRefStrSize && self.pusRefStr == other.pusRefStr
    }
}
impl ::core::cmp::Eq for TTLOADINFO {}
impl ::core::fmt::Debug for TTLOADINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTLOADINFO").field("usStructSize", &self.usStructSize).field("usRefStrSize", &self.usRefStrSize).field("pusRefStr", &self.pusRefStr).finish()
    }
}
impl ::core::default::Default for TTLOAD_EMBEDDED_FONT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TTLOAD_EMBEDDED_FONT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TTLOAD_EMBEDDED_FONT_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TTLOAD_EMBEDDED_FONT_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TTLOAD_EMBEDDED_FONT_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TTPOLYCURVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTPOLYCURVE {
    fn eq(&self, other: &Self) -> bool {
        self.wType == other.wType && self.cpfx == other.cpfx && self.apfx == other.apfx
    }
}
impl ::core::cmp::Eq for TTPOLYCURVE {}
impl ::core::fmt::Debug for TTPOLYCURVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTPOLYCURVE").field("wType", &self.wType).field("cpfx", &self.cpfx).field("apfx", &self.apfx).finish()
    }
}
impl ::core::default::Default for TTPOLYGONHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTPOLYGONHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.dwType == other.dwType && self.pfxStart == other.pfxStart
    }
}
impl ::core::cmp::Eq for TTPOLYGONHEADER {}
impl ::core::fmt::Debug for TTPOLYGONHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTPOLYGONHEADER").field("cb", &self.cb).field("dwType", &self.dwType).field("pfxStart", &self.pfxStart).finish()
    }
}
impl ::core::default::Default for TTVALIDATIONTESTSPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTVALIDATIONTESTSPARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.ulStructSize == other.ulStructSize && self.lTestFromSize == other.lTestFromSize && self.lTestToSize == other.lTestToSize && self.ulCharSet == other.ulCharSet && self.usReserved1 == other.usReserved1 && self.usCharCodeCount == other.usCharCodeCount && self.pusCharCodeSet == other.pusCharCodeSet
    }
}
impl ::core::cmp::Eq for TTVALIDATIONTESTSPARAMS {}
impl ::core::fmt::Debug for TTVALIDATIONTESTSPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTVALIDATIONTESTSPARAMS").field("ulStructSize", &self.ulStructSize).field("lTestFromSize", &self.lTestFromSize).field("lTestToSize", &self.lTestToSize).field("ulCharSet", &self.ulCharSet).field("usReserved1", &self.usReserved1).field("usCharCodeCount", &self.usCharCodeCount).field("pusCharCodeSet", &self.pusCharCodeSet).finish()
    }
}
impl ::core::default::Default for TTVALIDATIONTESTSPARAMSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TTVALIDATIONTESTSPARAMSEX {
    fn eq(&self, other: &Self) -> bool {
        self.ulStructSize == other.ulStructSize && self.lTestFromSize == other.lTestFromSize && self.lTestToSize == other.lTestToSize && self.ulCharSet == other.ulCharSet && self.usReserved1 == other.usReserved1 && self.usCharCodeCount == other.usCharCodeCount && self.pulCharCodeSet == other.pulCharCodeSet
    }
}
impl ::core::cmp::Eq for TTVALIDATIONTESTSPARAMSEX {}
impl ::core::fmt::Debug for TTVALIDATIONTESTSPARAMSEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TTVALIDATIONTESTSPARAMSEX").field("ulStructSize", &self.ulStructSize).field("lTestFromSize", &self.lTestFromSize).field("lTestToSize", &self.lTestToSize).field("ulCharSet", &self.ulCharSet).field("usReserved1", &self.usReserved1).field("usCharCodeCount", &self.usCharCodeCount).field("pulCharCodeSet", &self.pulCharCodeSet).finish()
    }
}
impl ::core::default::Default for WCRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cGlyphs == other.cGlyphs
    }
}
impl ::core::cmp::Eq for WCRANGE {}
impl ::core::fmt::Debug for WCRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCRANGE").field("wcLow", &self.wcLow).field("cGlyphs", &self.cGlyphs).finish()
    }
}
impl ::core::default::Default for WGLSWAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WGLSWAP {
    fn eq(&self, other: &Self) -> bool {
        self.hdc == other.hdc && self.uiFlags == other.uiFlags
    }
}
impl ::core::cmp::Eq for WGLSWAP {}
impl ::core::fmt::Debug for WGLSWAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WGLSWAP").field("hdc", &self.hdc).field("uiFlags", &self.uiFlags).finish()
    }
}
impl ::core::default::Default for XFORM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XFORM {
    fn eq(&self, other: &Self) -> bool {
        self.eM11 == other.eM11 && self.eM12 == other.eM12 && self.eM21 == other.eM21 && self.eM22 == other.eM22 && self.eDx == other.eDx && self.eDy == other.eDy
    }
}
impl ::core::cmp::Eq for XFORM {}
impl ::core::fmt::Debug for XFORM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XFORM").field("eM11", &self.eM11).field("eM12", &self.eM12).field("eM21", &self.eM21).field("eM22", &self.eM22).field("eDx", &self.eDx).field("eDy", &self.eDy).finish()
    }
}
