impl ::core::default::Default for BMFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BMFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BMFORMAT").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for BlackInformation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for BlackInformation {
    fn eq(&self, other: &Self) -> bool {
        self.fBlackOnly == other.fBlackOnly && self.blackWeight == other.blackWeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for BlackInformation {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for BlackInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BlackInformation").field("fBlackOnly", &self.fBlackOnly).field("blackWeight", &self.blackWeight).finish()
    }
}
impl ::core::default::Default for CMYKCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CMYKCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.cyan == other.cyan && self.magenta == other.magenta && self.yellow == other.yellow && self.black == other.black
    }
}
impl ::core::cmp::Eq for CMYKCOLOR {}
impl ::core::fmt::Debug for CMYKCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CMYKCOLOR").field("cyan", &self.cyan).field("magenta", &self.magenta).field("yellow", &self.yellow).field("black", &self.black).finish()
    }
}
impl ::core::default::Default for COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORDATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORDATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORDATATYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for COLORMATCHSETUPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::core::default::Default for COLORMATCHSETUPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for COLORPROFILESUBTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORPROFILESUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORPROFILESUBTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLORPROFILETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORPROFILETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORPROFILETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLORTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLORTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLORTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for COLOR_MATCH_TO_TARGET_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for COLOR_MATCH_TO_TARGET_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COLOR_MATCH_TO_TARGET_ACTION").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for EMRCREATECOLORSPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for EMRCREATECOLORSPACE {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihCS == other.ihCS && self.lcs == other.lcs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for EMRCREATECOLORSPACE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for EMRCREATECOLORSPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATECOLORSPACE").field("emr", &self.emr).field("ihCS", &self.ihCS).field("lcs", &self.lcs).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for EMRCREATECOLORSPACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for EMRCREATECOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        self.emr == other.emr && self.ihCS == other.ihCS && self.lcs == other.lcs && self.dwFlags == other.dwFlags && self.cbData == other.cbData && self.Data == other.Data
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for EMRCREATECOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for EMRCREATECOLORSPACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EMRCREATECOLORSPACEW").field("emr", &self.emr).field("ihCS", &self.ihCS).field("lcs", &self.lcs).field("dwFlags", &self.dwFlags).field("cbData", &self.cbData).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for ENUMTYPEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENUMTYPEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFields == other.dwFields
            && self.pDeviceName == other.pDeviceName
            && self.dwMediaType == other.dwMediaType
            && self.dwDitheringMode == other.dwDitheringMode
            && self.dwResolution == other.dwResolution
            && self.dwCMMType == other.dwCMMType
            && self.dwClass == other.dwClass
            && self.dwDataColorSpace == other.dwDataColorSpace
            && self.dwConnectionSpace == other.dwConnectionSpace
            && self.dwSignature == other.dwSignature
            && self.dwPlatform == other.dwPlatform
            && self.dwProfileFlags == other.dwProfileFlags
            && self.dwManufacturer == other.dwManufacturer
            && self.dwModel == other.dwModel
            && self.dwAttributes == other.dwAttributes
            && self.dwRenderingIntent == other.dwRenderingIntent
            && self.dwCreator == other.dwCreator
            && self.dwDeviceClass == other.dwDeviceClass
    }
}
impl ::core::cmp::Eq for ENUMTYPEA {}
impl ::core::fmt::Debug for ENUMTYPEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTYPEA")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFields", &self.dwFields)
            .field("pDeviceName", &self.pDeviceName)
            .field("dwMediaType", &self.dwMediaType)
            .field("dwDitheringMode", &self.dwDitheringMode)
            .field("dwResolution", &self.dwResolution)
            .field("dwCMMType", &self.dwCMMType)
            .field("dwClass", &self.dwClass)
            .field("dwDataColorSpace", &self.dwDataColorSpace)
            .field("dwConnectionSpace", &self.dwConnectionSpace)
            .field("dwSignature", &self.dwSignature)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwProfileFlags", &self.dwProfileFlags)
            .field("dwManufacturer", &self.dwManufacturer)
            .field("dwModel", &self.dwModel)
            .field("dwAttributes", &self.dwAttributes)
            .field("dwRenderingIntent", &self.dwRenderingIntent)
            .field("dwCreator", &self.dwCreator)
            .field("dwDeviceClass", &self.dwDeviceClass)
            .finish()
    }
}
impl ::core::default::Default for ENUMTYPEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENUMTYPEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwVersion == other.dwVersion
            && self.dwFields == other.dwFields
            && self.pDeviceName == other.pDeviceName
            && self.dwMediaType == other.dwMediaType
            && self.dwDitheringMode == other.dwDitheringMode
            && self.dwResolution == other.dwResolution
            && self.dwCMMType == other.dwCMMType
            && self.dwClass == other.dwClass
            && self.dwDataColorSpace == other.dwDataColorSpace
            && self.dwConnectionSpace == other.dwConnectionSpace
            && self.dwSignature == other.dwSignature
            && self.dwPlatform == other.dwPlatform
            && self.dwProfileFlags == other.dwProfileFlags
            && self.dwManufacturer == other.dwManufacturer
            && self.dwModel == other.dwModel
            && self.dwAttributes == other.dwAttributes
            && self.dwRenderingIntent == other.dwRenderingIntent
            && self.dwCreator == other.dwCreator
            && self.dwDeviceClass == other.dwDeviceClass
    }
}
impl ::core::cmp::Eq for ENUMTYPEW {}
impl ::core::fmt::Debug for ENUMTYPEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUMTYPEW")
            .field("dwSize", &self.dwSize)
            .field("dwVersion", &self.dwVersion)
            .field("dwFields", &self.dwFields)
            .field("pDeviceName", &self.pDeviceName)
            .field("dwMediaType", &self.dwMediaType)
            .field("dwDitheringMode", &self.dwDitheringMode)
            .field("dwResolution", &self.dwResolution)
            .field("dwCMMType", &self.dwCMMType)
            .field("dwClass", &self.dwClass)
            .field("dwDataColorSpace", &self.dwDataColorSpace)
            .field("dwConnectionSpace", &self.dwConnectionSpace)
            .field("dwSignature", &self.dwSignature)
            .field("dwPlatform", &self.dwPlatform)
            .field("dwProfileFlags", &self.dwProfileFlags)
            .field("dwManufacturer", &self.dwManufacturer)
            .field("dwModel", &self.dwModel)
            .field("dwAttributes", &self.dwAttributes)
            .field("dwRenderingIntent", &self.dwRenderingIntent)
            .field("dwCreator", &self.dwCreator)
            .field("dwDeviceClass", &self.dwDeviceClass)
            .finish()
    }
}
impl ::core::default::Default for GENERIC3CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GENERIC3CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.ch1 == other.ch1 && self.ch2 == other.ch2 && self.ch3 == other.ch3
    }
}
impl ::core::cmp::Eq for GENERIC3CHANNEL {}
impl ::core::fmt::Debug for GENERIC3CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC3CHANNEL").field("ch1", &self.ch1).field("ch2", &self.ch2).field("ch3", &self.ch3).finish()
    }
}
impl ::core::default::Default for GRAYCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GRAYCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.gray == other.gray
    }
}
impl ::core::cmp::Eq for GRAYCOLOR {}
impl ::core::fmt::Debug for GRAYCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRAYCOLOR").field("gray", &self.gray).finish()
    }
}
impl ::core::default::Default for GamutBoundaryDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GamutBoundaryDescription {
    fn eq(&self, other: &Self) -> bool {
        self.pPrimaries == other.pPrimaries && self.cNeutralSamples == other.cNeutralSamples && self.pNeutralSamples == other.pNeutralSamples && self.pReferenceShell == other.pReferenceShell && self.pPlausibleShell == other.pPlausibleShell && self.pPossibleShell == other.pPossibleShell
    }
}
impl ::core::cmp::Eq for GamutBoundaryDescription {}
impl ::core::fmt::Debug for GamutBoundaryDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutBoundaryDescription").field("pPrimaries", &self.pPrimaries).field("cNeutralSamples", &self.cNeutralSamples).field("pNeutralSamples", &self.pNeutralSamples).field("pReferenceShell", &self.pReferenceShell).field("pPlausibleShell", &self.pPlausibleShell).field("pPossibleShell", &self.pPossibleShell).finish()
    }
}
impl ::core::default::Default for GamutShell {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GamutShell {
    fn eq(&self, other: &Self) -> bool {
        self.JMin == other.JMin && self.JMax == other.JMax && self.cVertices == other.cVertices && self.cTriangles == other.cTriangles && self.pVertices == other.pVertices && self.pTriangles == other.pTriangles
    }
}
impl ::core::cmp::Eq for GamutShell {}
impl ::core::fmt::Debug for GamutShell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutShell").field("JMin", &self.JMin).field("JMax", &self.JMax).field("cVertices", &self.cVertices).field("cTriangles", &self.cTriangles).field("pVertices", &self.pVertices).field("pTriangles", &self.pTriangles).finish()
    }
}
impl ::core::default::Default for GamutShellTriangle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GamutShellTriangle {
    fn eq(&self, other: &Self) -> bool {
        self.aVertexIndex == other.aVertexIndex
    }
}
impl ::core::cmp::Eq for GamutShellTriangle {}
impl ::core::fmt::Debug for GamutShellTriangle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GamutShellTriangle").field("aVertexIndex", &self.aVertexIndex).finish()
    }
}
impl ::core::default::Default for HiFiCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HiFiCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.channel == other.channel
    }
}
impl ::core::cmp::Eq for HiFiCOLOR {}
impl ::core::fmt::Debug for HiFiCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HiFiCOLOR").field("channel", &self.channel).finish()
    }
}
impl ::core::default::Default for ICM_COMMAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICM_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICM_COMMAND").field(&self.0).finish()
    }
}
impl ::core::default::Default for ICM_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ICM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICM_MODE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDeviceModelPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDeviceModelPlugIn {}
impl ::core::fmt::Debug for IDeviceModelPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDeviceModelPlugIn").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGamutMapModelPlugIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGamutMapModelPlugIn {}
impl ::core::fmt::Debug for IGamutMapModelPlugIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGamutMapModelPlugIn").field(&self.0).finish()
    }
}
impl ::core::default::Default for JChColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JChColorF {
    fn eq(&self, other: &Self) -> bool {
        self.J == other.J && self.C == other.C && self.h == other.h
    }
}
impl ::core::cmp::Eq for JChColorF {}
impl ::core::fmt::Debug for JChColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JChColorF").field("J", &self.J).field("C", &self.C).field("h", &self.h).finish()
    }
}
impl ::core::default::Default for JabColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JabColorF {
    fn eq(&self, other: &Self) -> bool {
        self.J == other.J && self.a == other.a && self.b == other.b
    }
}
impl ::core::cmp::Eq for JabColorF {}
impl ::core::fmt::Debug for JabColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JabColorF").field("J", &self.J).field("a", &self.a).field("b", &self.b).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for LOGCOLORSPACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for LOGCOLORSPACEA {
    fn eq(&self, other: &Self) -> bool {
        self.lcsSignature == other.lcsSignature && self.lcsVersion == other.lcsVersion && self.lcsSize == other.lcsSize && self.lcsCSType == other.lcsCSType && self.lcsIntent == other.lcsIntent && self.lcsEndpoints == other.lcsEndpoints && self.lcsGammaRed == other.lcsGammaRed && self.lcsGammaGreen == other.lcsGammaGreen && self.lcsGammaBlue == other.lcsGammaBlue && self.lcsFilename == other.lcsFilename
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for LOGCOLORSPACEA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for LOGCOLORSPACEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGCOLORSPACEA").field("lcsSignature", &self.lcsSignature).field("lcsVersion", &self.lcsVersion).field("lcsSize", &self.lcsSize).field("lcsCSType", &self.lcsCSType).field("lcsIntent", &self.lcsIntent).field("lcsEndpoints", &self.lcsEndpoints).field("lcsGammaRed", &self.lcsGammaRed).field("lcsGammaGreen", &self.lcsGammaGreen).field("lcsGammaBlue", &self.lcsGammaBlue).field("lcsFilename", &self.lcsFilename).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for LOGCOLORSPACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for LOGCOLORSPACEW {
    fn eq(&self, other: &Self) -> bool {
        self.lcsSignature == other.lcsSignature && self.lcsVersion == other.lcsVersion && self.lcsSize == other.lcsSize && self.lcsCSType == other.lcsCSType && self.lcsIntent == other.lcsIntent && self.lcsEndpoints == other.lcsEndpoints && self.lcsGammaRed == other.lcsGammaRed && self.lcsGammaGreen == other.lcsGammaGreen && self.lcsGammaBlue == other.lcsGammaBlue && self.lcsFilename == other.lcsFilename
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for LOGCOLORSPACEW {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for LOGCOLORSPACEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOGCOLORSPACEW").field("lcsSignature", &self.lcsSignature).field("lcsVersion", &self.lcsVersion).field("lcsSize", &self.lcsSize).field("lcsCSType", &self.lcsCSType).field("lcsIntent", &self.lcsIntent).field("lcsEndpoints", &self.lcsEndpoints).field("lcsGammaRed", &self.lcsGammaRed).field("lcsGammaGreen", &self.lcsGammaGreen).field("lcsGammaBlue", &self.lcsGammaBlue).field("lcsFilename", &self.lcsFilename).finish()
    }
}
impl ::core::default::Default for LabCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LabCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.L == other.L && self.a == other.a && self.b == other.b
    }
}
impl ::core::cmp::Eq for LabCOLOR {}
impl ::core::fmt::Debug for LabCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LabCOLOR").field("L", &self.L).field("a", &self.a).field("b", &self.b).finish()
    }
}
impl ::core::default::Default for NAMEDCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NAMEDCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.dwIndex == other.dwIndex
    }
}
impl ::core::cmp::Eq for NAMEDCOLOR {}
impl ::core::fmt::Debug for NAMEDCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAMEDCOLOR").field("dwIndex", &self.dwIndex).finish()
    }
}
impl ::core::default::Default for NAMED_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NAMED_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwCount == other.dwCount && self.dwCountDevCoordinates == other.dwCountDevCoordinates && self.szPrefix == other.szPrefix && self.szSuffix == other.szSuffix
    }
}
impl ::core::cmp::Eq for NAMED_PROFILE_INFO {}
impl ::core::fmt::Debug for NAMED_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NAMED_PROFILE_INFO").field("dwFlags", &self.dwFlags).field("dwCount", &self.dwCount).field("dwCountDevCoordinates", &self.dwCountDevCoordinates).field("szPrefix", &self.szPrefix).field("szSuffix", &self.szSuffix).finish()
    }
}
impl ::core::default::Default for PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType && self.pProfileData == other.pProfileData && self.cbDataSize == other.cbDataSize
    }
}
impl ::core::cmp::Eq for PROFILE {}
impl ::core::fmt::Debug for PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILE").field("dwType", &self.dwType).field("pProfileData", &self.pProfileData).field("cbDataSize", &self.cbDataSize).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for PROFILEHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for PROFILEHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.phSize == other.phSize && self.phCMMType == other.phCMMType && self.phVersion == other.phVersion && self.phClass == other.phClass && self.phDataColorSpace == other.phDataColorSpace && self.phConnectionSpace == other.phConnectionSpace && self.phDateTime == other.phDateTime && self.phSignature == other.phSignature && self.phPlatform == other.phPlatform && self.phProfileFlags == other.phProfileFlags && self.phManufacturer == other.phManufacturer && self.phModel == other.phModel && self.phAttributes == other.phAttributes && self.phRenderingIntent == other.phRenderingIntent && self.phIlluminant == other.phIlluminant && self.phCreator == other.phCreator && self.phReserved == other.phReserved
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for PROFILEHEADER {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for PROFILEHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROFILEHEADER")
            .field("phSize", &self.phSize)
            .field("phCMMType", &self.phCMMType)
            .field("phVersion", &self.phVersion)
            .field("phClass", &self.phClass)
            .field("phDataColorSpace", &self.phDataColorSpace)
            .field("phConnectionSpace", &self.phConnectionSpace)
            .field("phDateTime", &self.phDateTime)
            .field("phSignature", &self.phSignature)
            .field("phPlatform", &self.phPlatform)
            .field("phProfileFlags", &self.phProfileFlags)
            .field("phManufacturer", &self.phManufacturer)
            .field("phModel", &self.phModel)
            .field("phAttributes", &self.phAttributes)
            .field("phRenderingIntent", &self.phRenderingIntent)
            .field("phIlluminant", &self.phIlluminant)
            .field("phCreator", &self.phCreator)
            .field("phReserved", &self.phReserved)
            .finish()
    }
}
impl ::core::default::Default for PrimaryJabColors {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PrimaryJabColors {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.yellow == other.yellow && self.green == other.green && self.cyan == other.cyan && self.blue == other.blue && self.magenta == other.magenta && self.black == other.black && self.white == other.white
    }
}
impl ::core::cmp::Eq for PrimaryJabColors {}
impl ::core::fmt::Debug for PrimaryJabColors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrimaryJabColors").field("red", &self.red).field("yellow", &self.yellow).field("green", &self.green).field("cyan", &self.cyan).field("blue", &self.blue).field("magenta", &self.magenta).field("black", &self.black).field("white", &self.white).finish()
    }
}
impl ::core::default::Default for PrimaryXYZColors {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PrimaryXYZColors {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.yellow == other.yellow && self.green == other.green && self.cyan == other.cyan && self.blue == other.blue && self.magenta == other.magenta && self.black == other.black && self.white == other.white
    }
}
impl ::core::cmp::Eq for PrimaryXYZColors {}
impl ::core::fmt::Debug for PrimaryXYZColors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PrimaryXYZColors").field("red", &self.red).field("yellow", &self.yellow).field("green", &self.green).field("cyan", &self.cyan).field("blue", &self.blue).field("magenta", &self.magenta).field("black", &self.black).field("white", &self.white).finish()
    }
}
impl ::core::default::Default for RGBCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RGBCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for RGBCOLOR {}
impl ::core::fmt::Debug for RGBCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RGBCOLOR").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::core::default::Default for WCS_DEVICE_CAPABILITIES_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCS_DEVICE_CAPABILITIES_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCS_DEVICE_CAPABILITIES_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCS_DEVICE_MHC2_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCS_DEVICE_MHC2_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SupportsMhc2 == other.SupportsMhc2 && self.RegammaLutEntryCount == other.RegammaLutEntryCount && self.CscXyzMatrixRows == other.CscXyzMatrixRows && self.CscXyzMatrixColumns == other.CscXyzMatrixColumns
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCS_DEVICE_MHC2_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCS_DEVICE_MHC2_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCS_DEVICE_MHC2_CAPABILITIES").field("Size", &self.Size).field("SupportsMhc2", &self.SupportsMhc2).field("RegammaLutEntryCount", &self.RegammaLutEntryCount).field("CscXyzMatrixRows", &self.CscXyzMatrixRows).field("CscXyzMatrixColumns", &self.CscXyzMatrixColumns).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCS_DEVICE_VCGT_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCS_DEVICE_VCGT_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.SupportsVcgt == other.SupportsVcgt
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCS_DEVICE_VCGT_CAPABILITIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCS_DEVICE_VCGT_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCS_DEVICE_VCGT_CAPABILITIES").field("Size", &self.Size).field("SupportsVcgt", &self.SupportsVcgt).finish()
    }
}
impl ::core::default::Default for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCS_PROFILE_MANAGEMENT_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCS_PROFILE_MANAGEMENT_SCOPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XYZCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XYZCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::core::cmp::Eq for XYZCOLOR {}
impl ::core::fmt::Debug for XYZCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XYZCOLOR").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::core::default::Default for XYZColorF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XYZColorF {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::core::cmp::Eq for XYZColorF {}
impl ::core::fmt::Debug for XYZColorF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XYZColorF").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::core::default::Default for YxyCOLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for YxyCOLOR {
    fn eq(&self, other: &Self) -> bool {
        self.Y == other.Y && self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for YxyCOLOR {}
impl ::core::fmt::Debug for YxyCOLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("YxyCOLOR").field("Y", &self.Y).field("x", &self.x).field("y", &self.y).finish()
    }
}
