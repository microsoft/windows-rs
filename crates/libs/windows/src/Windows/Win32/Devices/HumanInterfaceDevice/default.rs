impl ::core::default::Default for CPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.lP == other.lP && self.dwLog == other.dwLog
    }
}
impl ::core::cmp::Eq for CPOINT {}
impl ::core::fmt::Debug for CPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CPOINT").field("lP", &self.lP).field("dwLog", &self.dwLog).finish()
    }
}
impl ::core::default::Default for DIACTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIACTIONFORMATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIACTIONFORMATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwActionSize == other.dwActionSize && self.dwDataSize == other.dwDataSize && self.dwNumActions == other.dwNumActions && self.rgoAction == other.rgoAction && self.guidActionMap == other.guidActionMap && self.dwGenre == other.dwGenre && self.dwBufferSize == other.dwBufferSize && self.lAxisMin == other.lAxisMin && self.lAxisMax == other.lAxisMax && self.hInstString == other.hInstString && self.ftTimeStamp == other.ftTimeStamp && self.dwCRC == other.dwCRC && self.tszActionMap == other.tszActionMap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIACTIONFORMATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIACTIONFORMATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIACTIONFORMATA")
            .field("dwSize", &self.dwSize)
            .field("dwActionSize", &self.dwActionSize)
            .field("dwDataSize", &self.dwDataSize)
            .field("dwNumActions", &self.dwNumActions)
            .field("rgoAction", &self.rgoAction)
            .field("guidActionMap", &self.guidActionMap)
            .field("dwGenre", &self.dwGenre)
            .field("dwBufferSize", &self.dwBufferSize)
            .field("lAxisMin", &self.lAxisMin)
            .field("lAxisMax", &self.lAxisMax)
            .field("hInstString", &self.hInstString)
            .field("ftTimeStamp", &self.ftTimeStamp)
            .field("dwCRC", &self.dwCRC)
            .field("tszActionMap", &self.tszActionMap)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIACTIONFORMATW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIACTIONFORMATW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwActionSize == other.dwActionSize && self.dwDataSize == other.dwDataSize && self.dwNumActions == other.dwNumActions && self.rgoAction == other.rgoAction && self.guidActionMap == other.guidActionMap && self.dwGenre == other.dwGenre && self.dwBufferSize == other.dwBufferSize && self.lAxisMin == other.lAxisMin && self.lAxisMax == other.lAxisMax && self.hInstString == other.hInstString && self.ftTimeStamp == other.ftTimeStamp && self.dwCRC == other.dwCRC && self.tszActionMap == other.tszActionMap
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIACTIONFORMATW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIACTIONFORMATW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIACTIONFORMATW")
            .field("dwSize", &self.dwSize)
            .field("dwActionSize", &self.dwActionSize)
            .field("dwDataSize", &self.dwDataSize)
            .field("dwNumActions", &self.dwNumActions)
            .field("rgoAction", &self.rgoAction)
            .field("guidActionMap", &self.guidActionMap)
            .field("dwGenre", &self.dwGenre)
            .field("dwBufferSize", &self.dwBufferSize)
            .field("lAxisMin", &self.lAxisMin)
            .field("lAxisMax", &self.lAxisMax)
            .field("hInstString", &self.hInstString)
            .field("ftTimeStamp", &self.ftTimeStamp)
            .field("dwCRC", &self.dwCRC)
            .field("tszActionMap", &self.tszActionMap)
            .finish()
    }
}
impl ::core::default::Default for DIACTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DICOLORSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DICOLORSET {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cTextFore == other.cTextFore && self.cTextHighlight == other.cTextHighlight && self.cCalloutLine == other.cCalloutLine && self.cCalloutHighlight == other.cCalloutHighlight && self.cBorder == other.cBorder && self.cControlFill == other.cControlFill && self.cHighlightFill == other.cHighlightFill && self.cAreaFill == other.cAreaFill
    }
}
impl ::core::cmp::Eq for DICOLORSET {}
impl ::core::fmt::Debug for DICOLORSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICOLORSET").field("dwSize", &self.dwSize).field("cTextFore", &self.cTextFore).field("cTextHighlight", &self.cTextHighlight).field("cCalloutLine", &self.cCalloutLine).field("cCalloutHighlight", &self.cCalloutHighlight).field("cBorder", &self.cBorder).field("cControlFill", &self.cControlFill).field("cHighlightFill", &self.cHighlightFill).field("cAreaFill", &self.cAreaFill).finish()
    }
}
impl ::core::default::Default for DICONDITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DICONDITION {
    fn eq(&self, other: &Self) -> bool {
        self.lOffset == other.lOffset && self.lPositiveCoefficient == other.lPositiveCoefficient && self.lNegativeCoefficient == other.lNegativeCoefficient && self.dwPositiveSaturation == other.dwPositiveSaturation && self.dwNegativeSaturation == other.dwNegativeSaturation && self.lDeadBand == other.lDeadBand
    }
}
impl ::core::cmp::Eq for DICONDITION {}
impl ::core::fmt::Debug for DICONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONDITION").field("lOffset", &self.lOffset).field("lPositiveCoefficient", &self.lPositiveCoefficient).field("lNegativeCoefficient", &self.lNegativeCoefficient).field("dwPositiveSaturation", &self.dwPositiveSaturation).field("dwNegativeSaturation", &self.dwNegativeSaturation).field("lDeadBand", &self.lDeadBand).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DICONFIGUREDEVICESPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DICONFIGUREDEVICESPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwcUsers == other.dwcUsers && self.lptszUserNames == other.lptszUserNames && self.dwcFormats == other.dwcFormats && self.lprgFormats == other.lprgFormats && self.hwnd == other.hwnd && self.dics == other.dics && self.lpUnkDDSTarget == other.lpUnkDDSTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DICONFIGUREDEVICESPARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DICONFIGUREDEVICESPARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONFIGUREDEVICESPARAMSA").field("dwSize", &self.dwSize).field("dwcUsers", &self.dwcUsers).field("lptszUserNames", &self.lptszUserNames).field("dwcFormats", &self.dwcFormats).field("lprgFormats", &self.lprgFormats).field("hwnd", &self.hwnd).field("dics", &self.dics).field("lpUnkDDSTarget", &self.lpUnkDDSTarget).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DICONFIGUREDEVICESPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DICONFIGUREDEVICESPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwcUsers == other.dwcUsers && self.lptszUserNames == other.lptszUserNames && self.dwcFormats == other.dwcFormats && self.lprgFormats == other.lprgFormats && self.hwnd == other.hwnd && self.dics == other.dics && self.lpUnkDDSTarget == other.lpUnkDDSTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DICONFIGUREDEVICESPARAMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DICONFIGUREDEVICESPARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONFIGUREDEVICESPARAMSW").field("dwSize", &self.dwSize).field("dwcUsers", &self.dwcUsers).field("lptszUserNames", &self.lptszUserNames).field("dwcFormats", &self.dwcFormats).field("lprgFormats", &self.lprgFormats).field("hwnd", &self.hwnd).field("dics", &self.dics).field("lpUnkDDSTarget", &self.lpUnkDDSTarget).finish()
    }
}
impl ::core::default::Default for DICONSTANTFORCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DICONSTANTFORCE {
    fn eq(&self, other: &Self) -> bool {
        self.lMagnitude == other.lMagnitude
    }
}
impl ::core::cmp::Eq for DICONSTANTFORCE {}
impl ::core::fmt::Debug for DICONSTANTFORCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICONSTANTFORCE").field("lMagnitude", &self.lMagnitude).finish()
    }
}
impl ::core::default::Default for DICUSTOMFORCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DICUSTOMFORCE {
    fn eq(&self, other: &Self) -> bool {
        self.cChannels == other.cChannels && self.dwSamplePeriod == other.dwSamplePeriod && self.cSamples == other.cSamples && self.rglForceData == other.rglForceData
    }
}
impl ::core::cmp::Eq for DICUSTOMFORCE {}
impl ::core::fmt::Debug for DICUSTOMFORCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DICUSTOMFORCE").field("cChannels", &self.cChannels).field("dwSamplePeriod", &self.dwSamplePeriod).field("cSamples", &self.cSamples).field("rglForceData", &self.rglForceData).finish()
    }
}
impl ::core::default::Default for DIDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDATAFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwObjSize == other.dwObjSize && self.dwFlags == other.dwFlags && self.dwDataSize == other.dwDataSize && self.dwNumObjs == other.dwNumObjs && self.rgodf == other.rgodf
    }
}
impl ::core::cmp::Eq for DIDATAFORMAT {}
impl ::core::fmt::Debug for DIDATAFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDATAFORMAT").field("dwSize", &self.dwSize).field("dwObjSize", &self.dwObjSize).field("dwFlags", &self.dwFlags).field("dwDataSize", &self.dwDataSize).field("dwNumObjs", &self.dwNumObjs).field("rgodf", &self.rgodf).finish()
    }
}
impl ::core::default::Default for DIDEVCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDevType == other.dwDevType && self.dwAxes == other.dwAxes && self.dwButtons == other.dwButtons && self.dwPOVs == other.dwPOVs && self.dwFFSamplePeriod == other.dwFFSamplePeriod && self.dwFFMinTimeResolution == other.dwFFMinTimeResolution && self.dwFirmwareRevision == other.dwFirmwareRevision && self.dwHardwareRevision == other.dwHardwareRevision && self.dwFFDriverVersion == other.dwFFDriverVersion
    }
}
impl ::core::cmp::Eq for DIDEVCAPS {}
impl ::core::fmt::Debug for DIDEVCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwDevType", &self.dwDevType)
            .field("dwAxes", &self.dwAxes)
            .field("dwButtons", &self.dwButtons)
            .field("dwPOVs", &self.dwPOVs)
            .field("dwFFSamplePeriod", &self.dwFFSamplePeriod)
            .field("dwFFMinTimeResolution", &self.dwFFMinTimeResolution)
            .field("dwFirmwareRevision", &self.dwFirmwareRevision)
            .field("dwHardwareRevision", &self.dwHardwareRevision)
            .field("dwFFDriverVersion", &self.dwFFDriverVersion)
            .finish()
    }
}
impl ::core::default::Default for DIDEVCAPS_DX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVCAPS_DX3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDevType == other.dwDevType && self.dwAxes == other.dwAxes && self.dwButtons == other.dwButtons && self.dwPOVs == other.dwPOVs
    }
}
impl ::core::cmp::Eq for DIDEVCAPS_DX3 {}
impl ::core::fmt::Debug for DIDEVCAPS_DX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVCAPS_DX3").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDevType", &self.dwDevType).field("dwAxes", &self.dwAxes).field("dwButtons", &self.dwButtons).field("dwPOVs", &self.dwPOVs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.tszImagePath == other.tszImagePath && self.dwFlags == other.dwFlags && self.dwViewID == other.dwViewID && self.rcOverlay == other.rcOverlay && self.dwObjID == other.dwObjID && self.dwcValidPts == other.dwcValidPts && self.rgptCalloutLine == other.rgptCalloutLine && self.rcCalloutRect == other.rcCalloutRect && self.dwTextAlign == other.dwTextAlign
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOA").field("tszImagePath", &self.tszImagePath).field("dwFlags", &self.dwFlags).field("dwViewID", &self.dwViewID).field("rcOverlay", &self.rcOverlay).field("dwObjID", &self.dwObjID).field("dwcValidPts", &self.dwcValidPts).field("rgptCalloutLine", &self.rgptCalloutLine).field("rcCalloutRect", &self.rcCalloutRect).field("dwTextAlign", &self.dwTextAlign).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOHEADERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOHEADERA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwSizeImageInfo == other.dwSizeImageInfo && self.dwcViews == other.dwcViews && self.dwcButtons == other.dwcButtons && self.dwcAxes == other.dwcAxes && self.dwcPOVs == other.dwcPOVs && self.dwBufferSize == other.dwBufferSize && self.dwBufferUsed == other.dwBufferUsed && self.lprgImageInfoArray == other.lprgImageInfoArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOHEADERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOHEADERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOHEADERA").field("dwSize", &self.dwSize).field("dwSizeImageInfo", &self.dwSizeImageInfo).field("dwcViews", &self.dwcViews).field("dwcButtons", &self.dwcButtons).field("dwcAxes", &self.dwcAxes).field("dwcPOVs", &self.dwcPOVs).field("dwBufferSize", &self.dwBufferSize).field("dwBufferUsed", &self.dwBufferUsed).field("lprgImageInfoArray", &self.lprgImageInfoArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOHEADERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOHEADERW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwSizeImageInfo == other.dwSizeImageInfo && self.dwcViews == other.dwcViews && self.dwcButtons == other.dwcButtons && self.dwcAxes == other.dwcAxes && self.dwcPOVs == other.dwcPOVs && self.dwBufferSize == other.dwBufferSize && self.dwBufferUsed == other.dwBufferUsed && self.lprgImageInfoArray == other.lprgImageInfoArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOHEADERW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOHEADERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOHEADERW").field("dwSize", &self.dwSize).field("dwSizeImageInfo", &self.dwSizeImageInfo).field("dwcViews", &self.dwcViews).field("dwcButtons", &self.dwcButtons).field("dwcAxes", &self.dwcAxes).field("dwcPOVs", &self.dwcPOVs).field("dwBufferSize", &self.dwBufferSize).field("dwBufferUsed", &self.dwBufferUsed).field("lprgImageInfoArray", &self.lprgImageInfoArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEIMAGEINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEIMAGEINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.tszImagePath == other.tszImagePath && self.dwFlags == other.dwFlags && self.dwViewID == other.dwViewID && self.rcOverlay == other.rcOverlay && self.dwObjID == other.dwObjID && self.dwcValidPts == other.dwcValidPts && self.rgptCalloutLine == other.rgptCalloutLine && self.rcCalloutRect == other.rcCalloutRect && self.dwTextAlign == other.dwTextAlign
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEIMAGEINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEIMAGEINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEIMAGEINFOW").field("tszImagePath", &self.tszImagePath).field("dwFlags", &self.dwFlags).field("dwViewID", &self.dwViewID).field("rcOverlay", &self.rcOverlay).field("dwObjID", &self.dwObjID).field("dwcValidPts", &self.dwcValidPts).field("rgptCalloutLine", &self.rgptCalloutLine).field("rcCalloutRect", &self.rcCalloutRect).field("dwTextAlign", &self.dwTextAlign).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEINSTANCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEINSTANCEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName && self.guidFFDriver == other.guidFFDriver && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEINSTANCEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEINSTANCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCEA").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).field("guidFFDriver", &self.guidFFDriver).field("wUsagePage", &self.wUsagePage).field("wUsage", &self.wUsage).finish()
    }
}
impl ::core::default::Default for DIDEVICEINSTANCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICEINSTANCEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName && self.guidFFDriver == other.guidFFDriver && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage
    }
}
impl ::core::cmp::Eq for DIDEVICEINSTANCEW {}
impl ::core::fmt::Debug for DIDEVICEINSTANCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCEW").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).field("guidFFDriver", &self.guidFFDriver).field("wUsagePage", &self.wUsagePage).field("wUsage", &self.wUsage).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEINSTANCE_DX3A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEINSTANCE_DX3A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEINSTANCE_DX3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCE_DX3A").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).finish()
    }
}
impl ::core::default::Default for DIDEVICEINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICEINSTANCE_DX3W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.guidProduct == other.guidProduct && self.dwDevType == other.dwDevType && self.tszInstanceName == other.tszInstanceName && self.tszProductName == other.tszProductName
    }
}
impl ::core::cmp::Eq for DIDEVICEINSTANCE_DX3W {}
impl ::core::fmt::Debug for DIDEVICEINSTANCE_DX3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEINSTANCE_DX3W").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("guidProduct", &self.guidProduct).field("dwDevType", &self.dwDevType).field("tszInstanceName", &self.tszInstanceName).field("tszProductName", &self.tszProductName).finish()
    }
}
impl ::core::default::Default for DIDEVICEOBJECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwOfs == other.dwOfs && self.dwData == other.dwData && self.dwTimeStamp == other.dwTimeStamp && self.dwSequence == other.dwSequence && self.uAppData == other.uAppData
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTDATA {}
impl ::core::fmt::Debug for DIDEVICEOBJECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTDATA").field("dwOfs", &self.dwOfs).field("dwData", &self.dwData).field("dwTimeStamp", &self.dwTimeStamp).field("dwSequence", &self.dwSequence).field("uAppData", &self.uAppData).finish()
    }
}
impl ::core::default::Default for DIDEVICEOBJECTDATA_DX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTDATA_DX3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwOfs == other.dwOfs && self.dwData == other.dwData && self.dwTimeStamp == other.dwTimeStamp && self.dwSequence == other.dwSequence
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTDATA_DX3 {}
impl ::core::fmt::Debug for DIDEVICEOBJECTDATA_DX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTDATA_DX3").field("dwOfs", &self.dwOfs).field("dwData", &self.dwData).field("dwTimeStamp", &self.dwTimeStamp).field("dwSequence", &self.dwSequence).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEOBJECTINSTANCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName && self.dwFFMaxForce == other.dwFFMaxForce && self.dwFFForceResolution == other.dwFFForceResolution && self.wCollectionNumber == other.wCollectionNumber && self.wDesignatorIndex == other.wDesignatorIndex && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage && self.dwDimension == other.dwDimension && self.wExponent == other.wExponent && self.wReportId == other.wReportId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCEA")
            .field("dwSize", &self.dwSize)
            .field("guidType", &self.guidType)
            .field("dwOfs", &self.dwOfs)
            .field("dwType", &self.dwType)
            .field("dwFlags", &self.dwFlags)
            .field("tszName", &self.tszName)
            .field("dwFFMaxForce", &self.dwFFMaxForce)
            .field("dwFFForceResolution", &self.dwFFForceResolution)
            .field("wCollectionNumber", &self.wCollectionNumber)
            .field("wDesignatorIndex", &self.wDesignatorIndex)
            .field("wUsagePage", &self.wUsagePage)
            .field("wUsage", &self.wUsage)
            .field("dwDimension", &self.dwDimension)
            .field("wExponent", &self.wExponent)
            .field("wReportId", &self.wReportId)
            .finish()
    }
}
impl ::core::default::Default for DIDEVICEOBJECTINSTANCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName && self.dwFFMaxForce == other.dwFFMaxForce && self.dwFFForceResolution == other.dwFFForceResolution && self.wCollectionNumber == other.wCollectionNumber && self.wDesignatorIndex == other.wDesignatorIndex && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage && self.dwDimension == other.dwDimension && self.wExponent == other.wExponent && self.wReportId == other.wReportId
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCEW {}
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCEW")
            .field("dwSize", &self.dwSize)
            .field("guidType", &self.guidType)
            .field("dwOfs", &self.dwOfs)
            .field("dwType", &self.dwType)
            .field("dwFlags", &self.dwFlags)
            .field("tszName", &self.tszName)
            .field("dwFFMaxForce", &self.dwFFMaxForce)
            .field("dwFFForceResolution", &self.dwFFForceResolution)
            .field("wCollectionNumber", &self.wCollectionNumber)
            .field("wDesignatorIndex", &self.wDesignatorIndex)
            .field("wUsagePage", &self.wUsagePage)
            .field("wUsage", &self.wUsage)
            .field("dwDimension", &self.dwDimension)
            .field("wExponent", &self.wExponent)
            .field("wReportId", &self.wReportId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIDEVICEOBJECTINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCE_DX3A {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCE_DX3A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCE_DX3A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCE_DX3A").field("dwSize", &self.dwSize).field("guidType", &self.guidType).field("dwOfs", &self.dwOfs).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).field("tszName", &self.tszName).finish()
    }
}
impl ::core::default::Default for DIDEVICEOBJECTINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICEOBJECTINSTANCE_DX3W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidType == other.guidType && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags && self.tszName == other.tszName
    }
}
impl ::core::cmp::Eq for DIDEVICEOBJECTINSTANCE_DX3W {}
impl ::core::fmt::Debug for DIDEVICEOBJECTINSTANCE_DX3W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICEOBJECTINSTANCE_DX3W").field("dwSize", &self.dwSize).field("guidType", &self.guidType).field("dwOfs", &self.dwOfs).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).field("tszName", &self.tszName).finish()
    }
}
impl ::core::default::Default for DIDEVICESTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDEVICESTATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwState == other.dwState && self.dwLoad == other.dwLoad
    }
}
impl ::core::cmp::Eq for DIDEVICESTATE {}
impl ::core::fmt::Debug for DIDEVICESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDEVICESTATE").field("dwSize", &self.dwSize).field("dwState", &self.dwState).field("dwLoad", &self.dwLoad).finish()
    }
}
impl ::core::default::Default for DIDRIVERVERSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIDRIVERVERSIONS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFirmwareRevision == other.dwFirmwareRevision && self.dwHardwareRevision == other.dwHardwareRevision && self.dwFFDriverVersion == other.dwFFDriverVersion
    }
}
impl ::core::cmp::Eq for DIDRIVERVERSIONS {}
impl ::core::fmt::Debug for DIDRIVERVERSIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIDRIVERVERSIONS").field("dwSize", &self.dwSize).field("dwFirmwareRevision", &self.dwFirmwareRevision).field("dwHardwareRevision", &self.dwHardwareRevision).field("dwFFDriverVersion", &self.dwFFDriverVersion).finish()
    }
}
impl ::core::default::Default for DIEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIEFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDuration == other.dwDuration && self.dwSamplePeriod == other.dwSamplePeriod && self.dwGain == other.dwGain && self.dwTriggerButton == other.dwTriggerButton && self.dwTriggerRepeatInterval == other.dwTriggerRepeatInterval && self.cAxes == other.cAxes && self.rgdwAxes == other.rgdwAxes && self.rglDirection == other.rglDirection && self.lpEnvelope == other.lpEnvelope && self.cbTypeSpecificParams == other.cbTypeSpecificParams && self.lpvTypeSpecificParams == other.lpvTypeSpecificParams && self.dwStartDelay == other.dwStartDelay
    }
}
impl ::core::cmp::Eq for DIEFFECT {}
impl ::core::fmt::Debug for DIEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECT")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwDuration", &self.dwDuration)
            .field("dwSamplePeriod", &self.dwSamplePeriod)
            .field("dwGain", &self.dwGain)
            .field("dwTriggerButton", &self.dwTriggerButton)
            .field("dwTriggerRepeatInterval", &self.dwTriggerRepeatInterval)
            .field("cAxes", &self.cAxes)
            .field("rgdwAxes", &self.rgdwAxes)
            .field("rglDirection", &self.rglDirection)
            .field("lpEnvelope", &self.lpEnvelope)
            .field("cbTypeSpecificParams", &self.cbTypeSpecificParams)
            .field("lpvTypeSpecificParams", &self.lpvTypeSpecificParams)
            .field("dwStartDelay", &self.dwStartDelay)
            .finish()
    }
}
impl ::core::default::Default for DIEFFECTATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIEFFECTATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwEffectId == other.dwEffectId && self.dwEffType == other.dwEffType && self.dwStaticParams == other.dwStaticParams && self.dwDynamicParams == other.dwDynamicParams && self.dwCoords == other.dwCoords
    }
}
impl ::core::cmp::Eq for DIEFFECTATTRIBUTES {}
impl ::core::fmt::Debug for DIEFFECTATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECTATTRIBUTES").field("dwEffectId", &self.dwEffectId).field("dwEffType", &self.dwEffType).field("dwStaticParams", &self.dwStaticParams).field("dwDynamicParams", &self.dwDynamicParams).field("dwCoords", &self.dwCoords).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIEFFECTINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIEFFECTINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guid == other.guid && self.dwEffType == other.dwEffType && self.dwStaticParams == other.dwStaticParams && self.dwDynamicParams == other.dwDynamicParams && self.tszName == other.tszName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIEFFECTINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIEFFECTINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECTINFOA").field("dwSize", &self.dwSize).field("guid", &self.guid).field("dwEffType", &self.dwEffType).field("dwStaticParams", &self.dwStaticParams).field("dwDynamicParams", &self.dwDynamicParams).field("tszName", &self.tszName).finish()
    }
}
impl ::core::default::Default for DIEFFECTINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIEFFECTINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guid == other.guid && self.dwEffType == other.dwEffType && self.dwStaticParams == other.dwStaticParams && self.dwDynamicParams == other.dwDynamicParams && self.tszName == other.tszName
    }
}
impl ::core::cmp::Eq for DIEFFECTINFOW {}
impl ::core::fmt::Debug for DIEFFECTINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECTINFOW").field("dwSize", &self.dwSize).field("guid", &self.guid).field("dwEffType", &self.dwEffType).field("dwStaticParams", &self.dwStaticParams).field("dwDynamicParams", &self.dwDynamicParams).field("tszName", &self.tszName).finish()
    }
}
impl ::core::default::Default for DIEFFECT_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIEFFECT_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDuration == other.dwDuration && self.dwSamplePeriod == other.dwSamplePeriod && self.dwGain == other.dwGain && self.dwTriggerButton == other.dwTriggerButton && self.dwTriggerRepeatInterval == other.dwTriggerRepeatInterval && self.cAxes == other.cAxes && self.rgdwAxes == other.rgdwAxes && self.rglDirection == other.rglDirection && self.lpEnvelope == other.lpEnvelope && self.cbTypeSpecificParams == other.cbTypeSpecificParams && self.lpvTypeSpecificParams == other.lpvTypeSpecificParams
    }
}
impl ::core::cmp::Eq for DIEFFECT_DX5 {}
impl ::core::fmt::Debug for DIEFFECT_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFECT_DX5")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwDuration", &self.dwDuration)
            .field("dwSamplePeriod", &self.dwSamplePeriod)
            .field("dwGain", &self.dwGain)
            .field("dwTriggerButton", &self.dwTriggerButton)
            .field("dwTriggerRepeatInterval", &self.dwTriggerRepeatInterval)
            .field("cAxes", &self.cAxes)
            .field("rgdwAxes", &self.rgdwAxes)
            .field("rglDirection", &self.rglDirection)
            .field("lpEnvelope", &self.lpEnvelope)
            .field("cbTypeSpecificParams", &self.cbTypeSpecificParams)
            .field("lpvTypeSpecificParams", &self.lpvTypeSpecificParams)
            .finish()
    }
}
impl ::core::default::Default for DIEFFESCAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIEFFESCAPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCommand == other.dwCommand && self.lpvInBuffer == other.lpvInBuffer && self.cbInBuffer == other.cbInBuffer && self.lpvOutBuffer == other.lpvOutBuffer && self.cbOutBuffer == other.cbOutBuffer
    }
}
impl ::core::cmp::Eq for DIEFFESCAPE {}
impl ::core::fmt::Debug for DIEFFESCAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIEFFESCAPE").field("dwSize", &self.dwSize).field("dwCommand", &self.dwCommand).field("lpvInBuffer", &self.lpvInBuffer).field("cbInBuffer", &self.cbInBuffer).field("lpvOutBuffer", &self.lpvOutBuffer).field("cbOutBuffer", &self.cbOutBuffer).finish()
    }
}
impl ::core::default::Default for DIENVELOPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIENVELOPE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwAttackLevel == other.dwAttackLevel && self.dwAttackTime == other.dwAttackTime && self.dwFadeLevel == other.dwFadeLevel && self.dwFadeTime == other.dwFadeTime
    }
}
impl ::core::cmp::Eq for DIENVELOPE {}
impl ::core::fmt::Debug for DIENVELOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIENVELOPE").field("dwSize", &self.dwSize).field("dwAttackLevel", &self.dwAttackLevel).field("dwAttackTime", &self.dwAttackTime).field("dwFadeLevel", &self.dwFadeLevel).field("dwFadeTime", &self.dwFadeTime).finish()
    }
}
impl ::core::default::Default for DIFFDEVICEATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIFFDEVICEATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwFFSamplePeriod == other.dwFFSamplePeriod && self.dwFFMinTimeResolution == other.dwFFMinTimeResolution
    }
}
impl ::core::cmp::Eq for DIFFDEVICEATTRIBUTES {}
impl ::core::fmt::Debug for DIFFDEVICEATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIFFDEVICEATTRIBUTES").field("dwFlags", &self.dwFlags).field("dwFFSamplePeriod", &self.dwFFSamplePeriod).field("dwFFMinTimeResolution", &self.dwFFMinTimeResolution).finish()
    }
}
impl ::core::default::Default for DIFFOBJECTATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIFFOBJECTATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFFMaxForce == other.dwFFMaxForce && self.dwFFForceResolution == other.dwFFForceResolution
    }
}
impl ::core::cmp::Eq for DIFFOBJECTATTRIBUTES {}
impl ::core::fmt::Debug for DIFFOBJECTATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIFFOBJECTATTRIBUTES").field("dwFFMaxForce", &self.dwFFMaxForce).field("dwFFForceResolution", &self.dwFFForceResolution).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIFILEEFFECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIFILEEFFECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.GuidEffect == other.GuidEffect && self.lpDiEffect == other.lpDiEffect && self.szFriendlyName == other.szFriendlyName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIFILEEFFECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIFILEEFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIFILEEFFECT").field("dwSize", &self.dwSize).field("GuidEffect", &self.GuidEffect).field("lpDiEffect", &self.lpDiEffect).field("szFriendlyName", &self.szFriendlyName).finish()
    }
}
impl ::core::default::Default for DIHIDFFINITINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIHIDFFINITINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pwszDeviceInterface == other.pwszDeviceInterface && self.GuidInstance == other.GuidInstance
    }
}
impl ::core::cmp::Eq for DIHIDFFINITINFO {}
impl ::core::fmt::Debug for DIHIDFFINITINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIHIDFFINITINFO").field("dwSize", &self.dwSize).field("pwszDeviceInterface", &self.pwszDeviceInterface).field("GuidInstance", &self.GuidInstance).finish()
    }
}
impl ::core::default::Default for DIJOYCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.hwc == other.hwc && self.dwGain == other.dwGain && self.wszType == other.wszType && self.wszCallout == other.wszCallout && self.guidGameport == other.guidGameport
    }
}
impl ::core::cmp::Eq for DIJOYCONFIG {}
impl ::core::fmt::Debug for DIJOYCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYCONFIG").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("hwc", &self.hwc).field("dwGain", &self.dwGain).field("wszType", &self.wszType).field("wszCallout", &self.wszCallout).field("guidGameport", &self.guidGameport).finish()
    }
}
impl ::core::default::Default for DIJOYCONFIG_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYCONFIG_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.guidInstance == other.guidInstance && self.hwc == other.hwc && self.dwGain == other.dwGain && self.wszType == other.wszType && self.wszCallout == other.wszCallout
    }
}
impl ::core::cmp::Eq for DIJOYCONFIG_DX5 {}
impl ::core::fmt::Debug for DIJOYCONFIG_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYCONFIG_DX5").field("dwSize", &self.dwSize).field("guidInstance", &self.guidInstance).field("hwc", &self.hwc).field("dwGain", &self.dwGain).field("wszType", &self.wszType).field("wszCallout", &self.wszCallout).finish()
    }
}
impl ::core::default::Default for DIJOYSTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYSTATE {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX && self.lY == other.lY && self.lZ == other.lZ && self.lRx == other.lRx && self.lRy == other.lRy && self.lRz == other.lRz && self.rglSlider == other.rglSlider && self.rgdwPOV == other.rgdwPOV && self.rgbButtons == other.rgbButtons
    }
}
impl ::core::cmp::Eq for DIJOYSTATE {}
impl ::core::fmt::Debug for DIJOYSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYSTATE").field("lX", &self.lX).field("lY", &self.lY).field("lZ", &self.lZ).field("lRx", &self.lRx).field("lRy", &self.lRy).field("lRz", &self.lRz).field("rglSlider", &self.rglSlider).field("rgdwPOV", &self.rgdwPOV).field("rgbButtons", &self.rgbButtons).finish()
    }
}
impl ::core::default::Default for DIJOYSTATE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYSTATE2 {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX
            && self.lY == other.lY
            && self.lZ == other.lZ
            && self.lRx == other.lRx
            && self.lRy == other.lRy
            && self.lRz == other.lRz
            && self.rglSlider == other.rglSlider
            && self.rgdwPOV == other.rgdwPOV
            && self.rgbButtons == other.rgbButtons
            && self.lVX == other.lVX
            && self.lVY == other.lVY
            && self.lVZ == other.lVZ
            && self.lVRx == other.lVRx
            && self.lVRy == other.lVRy
            && self.lVRz == other.lVRz
            && self.rglVSlider == other.rglVSlider
            && self.lAX == other.lAX
            && self.lAY == other.lAY
            && self.lAZ == other.lAZ
            && self.lARx == other.lARx
            && self.lARy == other.lARy
            && self.lARz == other.lARz
            && self.rglASlider == other.rglASlider
            && self.lFX == other.lFX
            && self.lFY == other.lFY
            && self.lFZ == other.lFZ
            && self.lFRx == other.lFRx
            && self.lFRy == other.lFRy
            && self.lFRz == other.lFRz
            && self.rglFSlider == other.rglFSlider
    }
}
impl ::core::cmp::Eq for DIJOYSTATE2 {}
impl ::core::fmt::Debug for DIJOYSTATE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYSTATE2")
            .field("lX", &self.lX)
            .field("lY", &self.lY)
            .field("lZ", &self.lZ)
            .field("lRx", &self.lRx)
            .field("lRy", &self.lRy)
            .field("lRz", &self.lRz)
            .field("rglSlider", &self.rglSlider)
            .field("rgdwPOV", &self.rgdwPOV)
            .field("rgbButtons", &self.rgbButtons)
            .field("lVX", &self.lVX)
            .field("lVY", &self.lVY)
            .field("lVZ", &self.lVZ)
            .field("lVRx", &self.lVRx)
            .field("lVRy", &self.lVRy)
            .field("lVRz", &self.lVRz)
            .field("rglVSlider", &self.rglVSlider)
            .field("lAX", &self.lAX)
            .field("lAY", &self.lAY)
            .field("lAZ", &self.lAZ)
            .field("lARx", &self.lARx)
            .field("lARy", &self.lARy)
            .field("lARz", &self.lARz)
            .field("rglASlider", &self.rglASlider)
            .field("lFX", &self.lFX)
            .field("lFY", &self.lFY)
            .field("lFZ", &self.lFZ)
            .field("lFRx", &self.lFRx)
            .field("lFRy", &self.lFRy)
            .field("lFRz", &self.lFRz)
            .field("rglFSlider", &self.rglFSlider)
            .finish()
    }
}
impl ::core::default::Default for DIJOYTYPEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYTYPEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hws == other.hws && self.clsidConfig == other.clsidConfig && self.wszDisplayName == other.wszDisplayName && self.wszCallout == other.wszCallout && self.wszHardwareId == other.wszHardwareId && self.dwFlags1 == other.dwFlags1 && self.dwFlags2 == other.dwFlags2 && self.wszMapFile == other.wszMapFile
    }
}
impl ::core::cmp::Eq for DIJOYTYPEINFO {}
impl ::core::fmt::Debug for DIJOYTYPEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYTYPEINFO").field("dwSize", &self.dwSize).field("hws", &self.hws).field("clsidConfig", &self.clsidConfig).field("wszDisplayName", &self.wszDisplayName).field("wszCallout", &self.wszCallout).field("wszHardwareId", &self.wszHardwareId).field("dwFlags1", &self.dwFlags1).field("dwFlags2", &self.dwFlags2).field("wszMapFile", &self.wszMapFile).finish()
    }
}
impl ::core::default::Default for DIJOYTYPEINFO_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYTYPEINFO_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hws == other.hws && self.clsidConfig == other.clsidConfig && self.wszDisplayName == other.wszDisplayName && self.wszCallout == other.wszCallout
    }
}
impl ::core::cmp::Eq for DIJOYTYPEINFO_DX5 {}
impl ::core::fmt::Debug for DIJOYTYPEINFO_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYTYPEINFO_DX5").field("dwSize", &self.dwSize).field("hws", &self.hws).field("clsidConfig", &self.clsidConfig).field("wszDisplayName", &self.wszDisplayName).field("wszCallout", &self.wszCallout).finish()
    }
}
impl ::core::default::Default for DIJOYTYPEINFO_DX6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYTYPEINFO_DX6 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hws == other.hws && self.clsidConfig == other.clsidConfig && self.wszDisplayName == other.wszDisplayName && self.wszCallout == other.wszCallout && self.wszHardwareId == other.wszHardwareId && self.dwFlags1 == other.dwFlags1
    }
}
impl ::core::cmp::Eq for DIJOYTYPEINFO_DX6 {}
impl ::core::fmt::Debug for DIJOYTYPEINFO_DX6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYTYPEINFO_DX6").field("dwSize", &self.dwSize).field("hws", &self.hws).field("clsidConfig", &self.clsidConfig).field("wszDisplayName", &self.wszDisplayName).field("wszCallout", &self.wszCallout).field("wszHardwareId", &self.wszHardwareId).field("dwFlags1", &self.dwFlags1).finish()
    }
}
impl ::core::default::Default for DIJOYUSERVALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIJOYUSERVALUES {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ruv == other.ruv && self.wszGlobalDriver == other.wszGlobalDriver && self.wszGameportEmulator == other.wszGameportEmulator
    }
}
impl ::core::cmp::Eq for DIJOYUSERVALUES {}
impl ::core::fmt::Debug for DIJOYUSERVALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIJOYUSERVALUES").field("dwSize", &self.dwSize).field("ruv", &self.ruv).field("wszGlobalDriver", &self.wszGlobalDriver).field("wszGameportEmulator", &self.wszGameportEmulator).finish()
    }
}
impl ::core::default::Default for DIMOUSESTATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIMOUSESTATE {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX && self.lY == other.lY && self.lZ == other.lZ && self.rgbButtons == other.rgbButtons
    }
}
impl ::core::cmp::Eq for DIMOUSESTATE {}
impl ::core::fmt::Debug for DIMOUSESTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIMOUSESTATE").field("lX", &self.lX).field("lY", &self.lY).field("lZ", &self.lZ).field("rgbButtons", &self.rgbButtons).finish()
    }
}
impl ::core::default::Default for DIMOUSESTATE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIMOUSESTATE2 {
    fn eq(&self, other: &Self) -> bool {
        self.lX == other.lX && self.lY == other.lY && self.lZ == other.lZ && self.rgbButtons == other.rgbButtons
    }
}
impl ::core::cmp::Eq for DIMOUSESTATE2 {}
impl ::core::fmt::Debug for DIMOUSESTATE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIMOUSESTATE2").field("lX", &self.lX).field("lY", &self.lY).field("lZ", &self.lZ).field("rgbButtons", &self.rgbButtons).finish()
    }
}
impl ::core::default::Default for DIOBJECTATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIOBJECTATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.wUsagePage == other.wUsagePage && self.wUsage == other.wUsage
    }
}
impl ::core::cmp::Eq for DIOBJECTATTRIBUTES {}
impl ::core::fmt::Debug for DIOBJECTATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIOBJECTATTRIBUTES").field("dwFlags", &self.dwFlags).field("wUsagePage", &self.wUsagePage).field("wUsage", &self.wUsage).finish()
    }
}
impl ::core::default::Default for DIOBJECTCALIBRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIOBJECTCALIBRATION {
    fn eq(&self, other: &Self) -> bool {
        self.lMin == other.lMin && self.lCenter == other.lCenter && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIOBJECTCALIBRATION {}
impl ::core::fmt::Debug for DIOBJECTCALIBRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIOBJECTCALIBRATION").field("lMin", &self.lMin).field("lCenter", &self.lCenter).field("lMax", &self.lMax).finish()
    }
}
impl ::core::default::Default for DIOBJECTDATAFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIOBJECTDATAFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.dwOfs == other.dwOfs && self.dwType == other.dwType && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DIOBJECTDATAFORMAT {}
impl ::core::fmt::Debug for DIOBJECTDATAFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIOBJECTDATAFORMAT").field("pguid", &self.pguid).field("dwOfs", &self.dwOfs).field("dwType", &self.dwType).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::core::default::Default for DIPERIODIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPERIODIC {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagnitude == other.dwMagnitude && self.lOffset == other.lOffset && self.dwPhase == other.dwPhase && self.dwPeriod == other.dwPeriod
    }
}
impl ::core::cmp::Eq for DIPERIODIC {}
impl ::core::fmt::Debug for DIPERIODIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPERIODIC").field("dwMagnitude", &self.dwMagnitude).field("lOffset", &self.lOffset).field("dwPhase", &self.dwPhase).field("dwPeriod", &self.dwPeriod).finish()
    }
}
impl ::core::default::Default for DIPOVCALIBRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPOVCALIBRATION {
    fn eq(&self, other: &Self) -> bool {
        self.lMin == other.lMin && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPOVCALIBRATION {}
impl ::core::fmt::Debug for DIPOVCALIBRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPOVCALIBRATION").field("lMin", &self.lMin).field("lMax", &self.lMax).finish()
    }
}
impl ::core::default::Default for DIPROPCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPCAL {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.lMin == other.lMin && self.lCenter == other.lCenter && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPROPCAL {}
impl ::core::fmt::Debug for DIPROPCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPCAL").field("diph", &self.diph).field("lMin", &self.lMin).field("lCenter", &self.lCenter).field("lMax", &self.lMax).finish()
    }
}
impl ::core::default::Default for DIPROPCALPOV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPCALPOV {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.lMin == other.lMin && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPROPCALPOV {}
impl ::core::fmt::Debug for DIPROPCALPOV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPCALPOV").field("diph", &self.diph).field("lMin", &self.lMin).field("lMax", &self.lMax).finish()
    }
}
impl ::core::default::Default for DIPROPCPOINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPCPOINTS {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.dwCPointsNum == other.dwCPointsNum && self.cp == other.cp
    }
}
impl ::core::cmp::Eq for DIPROPCPOINTS {}
impl ::core::fmt::Debug for DIPROPCPOINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPCPOINTS").field("diph", &self.diph).field("dwCPointsNum", &self.dwCPointsNum).field("cp", &self.cp).finish()
    }
}
impl ::core::default::Default for DIPROPDWORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPDWORD {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.dwData == other.dwData
    }
}
impl ::core::cmp::Eq for DIPROPDWORD {}
impl ::core::fmt::Debug for DIPROPDWORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPDWORD").field("diph", &self.diph).field("dwData", &self.dwData).finish()
    }
}
impl ::core::default::Default for DIPROPGUIDANDPATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPGUIDANDPATH {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.guidClass == other.guidClass && self.wszPath == other.wszPath
    }
}
impl ::core::cmp::Eq for DIPROPGUIDANDPATH {}
impl ::core::fmt::Debug for DIPROPGUIDANDPATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPGUIDANDPATH").field("diph", &self.diph).field("guidClass", &self.guidClass).field("wszPath", &self.wszPath).finish()
    }
}
impl ::core::default::Default for DIPROPHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeaderSize == other.dwHeaderSize && self.dwObj == other.dwObj && self.dwHow == other.dwHow
    }
}
impl ::core::cmp::Eq for DIPROPHEADER {}
impl ::core::fmt::Debug for DIPROPHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPHEADER").field("dwSize", &self.dwSize).field("dwHeaderSize", &self.dwHeaderSize).field("dwObj", &self.dwObj).field("dwHow", &self.dwHow).finish()
    }
}
impl ::core::default::Default for DIPROPPOINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPPOINTER {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.uData == other.uData
    }
}
impl ::core::cmp::Eq for DIPROPPOINTER {}
impl ::core::fmt::Debug for DIPROPPOINTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPPOINTER").field("diph", &self.diph).field("uData", &self.uData).finish()
    }
}
impl ::core::default::Default for DIPROPRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.lMin == other.lMin && self.lMax == other.lMax
    }
}
impl ::core::cmp::Eq for DIPROPRANGE {}
impl ::core::fmt::Debug for DIPROPRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPRANGE").field("diph", &self.diph).field("lMin", &self.lMin).field("lMax", &self.lMax).finish()
    }
}
impl ::core::default::Default for DIPROPSTRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIPROPSTRING {
    fn eq(&self, other: &Self) -> bool {
        self.diph == other.diph && self.wsz == other.wsz
    }
}
impl ::core::cmp::Eq for DIPROPSTRING {}
impl ::core::fmt::Debug for DIPROPSTRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIPROPSTRING").field("diph", &self.diph).field("wsz", &self.wsz).finish()
    }
}
impl ::core::default::Default for DIRAMPFORCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DIRAMPFORCE {
    fn eq(&self, other: &Self) -> bool {
        self.lStart == other.lStart && self.lEnd == other.lEnd
    }
}
impl ::core::cmp::Eq for DIRAMPFORCE {}
impl ::core::fmt::Debug for DIRAMPFORCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIRAMPFORCE").field("lStart", &self.lStart).field("lEnd", &self.lEnd).finish()
    }
}
impl ::core::default::Default for GPIOBUTTONS_BUTTON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GPIOBUTTONS_BUTTON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPIOBUTTONS_BUTTON_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HIDD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIDD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.VendorID == other.VendorID && self.ProductID == other.ProductID && self.VersionNumber == other.VersionNumber
    }
}
impl ::core::cmp::Eq for HIDD_ATTRIBUTES {}
impl ::core::fmt::Debug for HIDD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDD_ATTRIBUTES").field("Size", &self.Size).field("VendorID", &self.VendorID).field("ProductID", &self.ProductID).field("VersionNumber", &self.VersionNumber).finish()
    }
}
impl ::core::default::Default for HIDD_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_ARRAY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HIDP_BUTTON_ARRAY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ArrayIndex == other.ArrayIndex && self.On == other.On
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HIDP_BUTTON_ARRAY_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HIDP_BUTTON_ARRAY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_BUTTON_ARRAY_DATA").field("ArrayIndex", &self.ArrayIndex).field("On", &self.On).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIDP_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIDP_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Usage == other.Usage
            && self.UsagePage == other.UsagePage
            && self.InputReportByteLength == other.InputReportByteLength
            && self.OutputReportByteLength == other.OutputReportByteLength
            && self.FeatureReportByteLength == other.FeatureReportByteLength
            && self.Reserved == other.Reserved
            && self.NumberLinkCollectionNodes == other.NumberLinkCollectionNodes
            && self.NumberInputButtonCaps == other.NumberInputButtonCaps
            && self.NumberInputValueCaps == other.NumberInputValueCaps
            && self.NumberInputDataIndices == other.NumberInputDataIndices
            && self.NumberOutputButtonCaps == other.NumberOutputButtonCaps
            && self.NumberOutputValueCaps == other.NumberOutputValueCaps
            && self.NumberOutputDataIndices == other.NumberOutputDataIndices
            && self.NumberFeatureButtonCaps == other.NumberFeatureButtonCaps
            && self.NumberFeatureValueCaps == other.NumberFeatureValueCaps
            && self.NumberFeatureDataIndices == other.NumberFeatureDataIndices
    }
}
impl ::core::cmp::Eq for HIDP_CAPS {}
impl ::core::fmt::Debug for HIDP_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_CAPS")
            .field("Usage", &self.Usage)
            .field("UsagePage", &self.UsagePage)
            .field("InputReportByteLength", &self.InputReportByteLength)
            .field("OutputReportByteLength", &self.OutputReportByteLength)
            .field("FeatureReportByteLength", &self.FeatureReportByteLength)
            .field("Reserved", &self.Reserved)
            .field("NumberLinkCollectionNodes", &self.NumberLinkCollectionNodes)
            .field("NumberInputButtonCaps", &self.NumberInputButtonCaps)
            .field("NumberInputValueCaps", &self.NumberInputValueCaps)
            .field("NumberInputDataIndices", &self.NumberInputDataIndices)
            .field("NumberOutputButtonCaps", &self.NumberOutputButtonCaps)
            .field("NumberOutputValueCaps", &self.NumberOutputValueCaps)
            .field("NumberOutputDataIndices", &self.NumberOutputDataIndices)
            .field("NumberFeatureButtonCaps", &self.NumberFeatureButtonCaps)
            .field("NumberFeatureValueCaps", &self.NumberFeatureValueCaps)
            .field("NumberFeatureDataIndices", &self.NumberFeatureDataIndices)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIDP_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIDP_KEYBOARD_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HIDP_KEYBOARD_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIDP_KEYBOARD_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for HIDP_KEYBOARD_MODIFIER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIDP_LINK_COLLECTION_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HIDP_REPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HIDP_REPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HIDP_REPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HIDP_UNKNOWN_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HIDP_UNKNOWN_TOKEN {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token && self.Reserved == other.Reserved && self.BitField == other.BitField
    }
}
impl ::core::cmp::Eq for HIDP_UNKNOWN_TOKEN {}
impl ::core::fmt::Debug for HIDP_UNKNOWN_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HIDP_UNKNOWN_TOKEN").field("Token", &self.Token).field("Reserved", &self.Reserved).field("BitField", &self.BitField).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HID_COLLECTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HID_COLLECTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.DescriptorSize == other.DescriptorSize && self.Polled == other.Polled && self.Reserved1 == other.Reserved1 && self.VendorID == other.VendorID && self.ProductID == other.ProductID && self.VersionNumber == other.VersionNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HID_COLLECTION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HID_COLLECTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HID_COLLECTION_INFORMATION").field("DescriptorSize", &self.DescriptorSize).field("Polled", &self.Polled).field("Reserved1", &self.Reserved1).field("VendorID", &self.VendorID).field("ProductID", &self.ProductID).field("VersionNumber", &self.VersionNumber).finish()
    }
}
impl ::core::default::Default for HID_DRIVER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HID_DRIVER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.RingBufferSize == other.RingBufferSize
    }
}
impl ::core::cmp::Eq for HID_DRIVER_CONFIG {}
impl ::core::fmt::Debug for HID_DRIVER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HID_DRIVER_CONFIG").field("Size", &self.Size).field("RingBufferSize", &self.RingBufferSize).finish()
    }
}
impl ::core::default::Default for HID_XFER_PACKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HID_XFER_PACKET {
    fn eq(&self, other: &Self) -> bool {
        self.reportBuffer == other.reportBuffer && self.reportBufferLen == other.reportBufferLen && self.reportId == other.reportId
    }
}
impl ::core::cmp::Eq for HID_XFER_PACKET {}
impl ::core::fmt::Debug for HID_XFER_PACKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HID_XFER_PACKET").field("reportBuffer", &self.reportBuffer).field("reportBufferLen", &self.reportBufferLen).field("reportId", &self.reportId).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInput2A {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInput2A {}
impl ::core::fmt::Debug for IDirectInput2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInput2A").field(&self.0).finish()
    }
}
impl IDirectInput2A {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDevice)(::windows::core::Vtable::as_raw(self), param0, ::core::mem::transmute(param1), param2.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumDevices)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceStatus)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInput2W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInput2W {}
impl ::core::fmt::Debug for IDirectInput2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInput2W").field(&self.0).finish()
    }
}
impl IDirectInput2W {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDevice)(::windows::core::Vtable::as_raw(self), param0, ::core::mem::transmute(param1), param2.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumDevices)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceStatus)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInput7A {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInput7A {}
impl ::core::fmt::Debug for IDirectInput7A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInput7A").field(&self.0).finish()
    }
}
impl IDirectInput7A {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceA>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), param0, ::core::mem::transmute(param1), param2.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumDevices)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceStatus)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows::core::GUID, param1: P0, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindDevice)(::windows::core::Vtable::as_raw(self), param0, param1.into().abi(), param2).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInput7W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInput7W {}
impl ::core::fmt::Debug for IDirectInput7W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInput7W").field(&self.0).finish()
    }
}
impl IDirectInput7W {
    pub unsafe fn CreateDevice<P0>(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::option::Option<IDirectInputDeviceW>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDevice)(::windows::core::Vtable::as_raw(self), param0, ::core::mem::transmute(param1), param2.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut ::core::ffi::c_void, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumDevices)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceStatus)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    pub unsafe fn FindDevice<P0>(&self, param0: *const ::windows::core::GUID, param1: P0, param2: *mut ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindDevice)(::windows::core::Vtable::as_raw(self), param0, param1.into().abi(), param2).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInput8A {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInput8A {}
impl ::core::fmt::Debug for IDirectInput8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInput8A").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInput8W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInput8W {}
impl ::core::fmt::Debug for IDirectInput8W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInput8W").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputA {}
impl ::core::fmt::Debug for IDirectInputA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDevice2A {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDevice2A {}
impl ::core::fmt::Debug for IDirectInputDevice2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDevice2A").field(&self.0).finish()
    }
}
impl IDirectInputDevice2A {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumObjects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Acquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unacquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceState)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceData)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDataFormat)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEventNotification)(::windows::core::Vtable::as_raw(self), param0.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCooperativeLevel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectInfo)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceInfo)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1, param2).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDevice2W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDevice2W {}
impl ::core::fmt::Debug for IDirectInputDevice2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDevice2W").field(&self.0).finish()
    }
}
impl IDirectInputDevice2W {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumObjects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Acquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unacquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceState)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceData)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDataFormat)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEventNotification)(::windows::core::Vtable::as_raw(self), param0.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCooperativeLevel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetObjectInfo)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceInfo)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1, param2).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDevice7A {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDevice7A {}
impl ::core::fmt::Debug for IDirectInputDevice7A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDevice7A").field(&self.0).finish()
    }
}
impl IDirectInputDevice7A {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumObjects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Acquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Unacquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceState)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceData)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDataFormat)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEventNotification)(::windows::core::Vtable::as_raw(self), param0.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCooperativeLevel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetObjectInfo)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceInfo)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateEffect)(::windows::core::Vtable::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumEffects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEffectInfo)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetForceFeedbackState)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendForceFeedbackCommand)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumCreatedEffectObjects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Escape)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Poll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendDeviceData)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDevice7W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDevice7W {}
impl ::core::fmt::Debug for IDirectInputDevice7W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDevice7W").field(&self.0).finish()
    }
}
impl IDirectInputDevice7W {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumObjects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetProperty(&self, param0: *const ::windows::core::GUID, param1: *mut DIPROPHEADER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProperty)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn Acquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Acquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Unacquire(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Unacquire)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceState)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceData)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDataFormat)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEventNotification)(::windows::core::Vtable::as_raw(self), param0.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCooperativeLevel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetObjectInfo)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceInfo)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RunControlPanel)(::windows::core::Vtable::as_raw(self), param0.into(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), param0.into(), param1, param2).ok()
    }
    pub unsafe fn CreateEffect<P0>(&self, param0: *const ::windows::core::GUID, param1: *mut DIEFFECT, param2: *mut ::core::option::Option<IDirectInputEffect>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateEffect)(::windows::core::Vtable::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumEffects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetEffectInfo)(::windows::core::Vtable::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetForceFeedbackState)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendForceFeedbackCommand)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut ::core::ffi::c_void, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumCreatedEffectObjects)(::windows::core::Vtable::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Escape)(::windows::core::Vtable::as_raw(self), param0).ok()
    }
    pub unsafe fn Poll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Poll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendDeviceData)(::windows::core::Vtable::as_raw(self), param0, param1, param2, param3).ok()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDevice8A {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDevice8A {}
impl ::core::fmt::Debug for IDirectInputDevice8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDevice8A").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDevice8W {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDevice8W {}
impl ::core::fmt::Debug for IDirectInputDevice8W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDevice8W").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDeviceA {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDeviceA {}
impl ::core::fmt::Debug for IDirectInputDeviceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDeviceA").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputDeviceW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputDeviceW {}
impl ::core::fmt::Debug for IDirectInputDeviceW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputDeviceW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputEffect {}
impl ::core::fmt::Debug for IDirectInputEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputEffect").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputEffectDriver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputEffectDriver {}
impl ::core::fmt::Debug for IDirectInputEffectDriver {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputEffectDriver").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputJoyConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputJoyConfig {}
impl ::core::fmt::Debug for IDirectInputJoyConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputJoyConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputJoyConfig8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputJoyConfig8 {}
impl ::core::fmt::Debug for IDirectInputJoyConfig8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputJoyConfig8").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDirectInputW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectInputW {}
impl ::core::fmt::Debug for IDirectInputW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectInputW").field(&self.0).finish()
    }
}
impl ::core::default::Default for INDICATOR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for INDICATOR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.MakeCode == other.MakeCode && self.IndicatorFlags == other.IndicatorFlags
    }
}
impl ::core::cmp::Eq for INDICATOR_LIST {}
impl ::core::fmt::Debug for INDICATOR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INDICATOR_LIST").field("MakeCode", &self.MakeCode).field("IndicatorFlags", &self.IndicatorFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INPUT_BUTTON_ENABLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INPUT_BUTTON_ENABLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ButtonType == other.ButtonType && self.Enabled == other.Enabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INPUT_BUTTON_ENABLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INPUT_BUTTON_ENABLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INPUT_BUTTON_ENABLE_INFO").field("ButtonType", &self.ButtonType).field("Enabled", &self.Enabled).finish()
    }
}
impl ::core::default::Default for JOYCALIBRATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYCALIBRATE {
    fn eq(&self, other: &Self) -> bool {
        self.wXbase == other.wXbase && self.wXdelta == other.wXdelta && self.wYbase == other.wYbase && self.wYdelta == other.wYdelta && self.wZbase == other.wZbase && self.wZdelta == other.wZdelta
    }
}
impl ::core::cmp::Eq for JOYCALIBRATE {}
impl ::core::fmt::Debug for JOYCALIBRATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYCALIBRATE").field("wXbase", &self.wXbase).field("wXdelta", &self.wXdelta).field("wYbase", &self.wYbase).field("wYdelta", &self.wYdelta).field("wZbase", &self.wZbase).field("wZdelta", &self.wZdelta).finish()
    }
}
impl ::core::default::Default for JOYPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYPOS {
    fn eq(&self, other: &Self) -> bool {
        self.dwX == other.dwX && self.dwY == other.dwY && self.dwZ == other.dwZ && self.dwR == other.dwR && self.dwU == other.dwU && self.dwV == other.dwV
    }
}
impl ::core::cmp::Eq for JOYPOS {}
impl ::core::fmt::Debug for JOYPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYPOS").field("dwX", &self.dwX).field("dwY", &self.dwY).field("dwZ", &self.dwZ).field("dwR", &self.dwR).field("dwU", &self.dwU).field("dwV", &self.dwV).finish()
    }
}
impl ::core::default::Default for JOYRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.jpMin == other.jpMin && self.jpMax == other.jpMax && self.jpCenter == other.jpCenter
    }
}
impl ::core::cmp::Eq for JOYRANGE {}
impl ::core::fmt::Debug for JOYRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYRANGE").field("jpMin", &self.jpMin).field("jpMax", &self.jpMax).field("jpCenter", &self.jpCenter).finish()
    }
}
impl ::core::default::Default for JOYREGHWCONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYREGHWCONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.hws == other.hws && self.dwUsageSettings == other.dwUsageSettings && self.hwv == other.hwv && self.dwType == other.dwType && self.dwReserved == other.dwReserved
    }
}
impl ::core::cmp::Eq for JOYREGHWCONFIG {}
impl ::core::fmt::Debug for JOYREGHWCONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGHWCONFIG").field("hws", &self.hws).field("dwUsageSettings", &self.dwUsageSettings).field("hwv", &self.hwv).field("dwType", &self.dwType).field("dwReserved", &self.dwReserved).finish()
    }
}
impl ::core::default::Default for JOYREGHWSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYREGHWSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.dwNumButtons == other.dwNumButtons
    }
}
impl ::core::cmp::Eq for JOYREGHWSETTINGS {}
impl ::core::fmt::Debug for JOYREGHWSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGHWSETTINGS").field("dwFlags", &self.dwFlags).field("dwNumButtons", &self.dwNumButtons).finish()
    }
}
impl ::core::default::Default for JOYREGHWVALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYREGHWVALUES {
    fn eq(&self, other: &Self) -> bool {
        self.jrvHardware == other.jrvHardware && self.dwPOVValues == other.dwPOVValues && self.dwCalFlags == other.dwCalFlags
    }
}
impl ::core::cmp::Eq for JOYREGHWVALUES {}
impl ::core::fmt::Debug for JOYREGHWVALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGHWVALUES").field("jrvHardware", &self.jrvHardware).field("dwPOVValues", &self.dwPOVValues).field("dwCalFlags", &self.dwCalFlags).finish()
    }
}
impl ::core::default::Default for JOYREGUSERVALUES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JOYREGUSERVALUES {
    fn eq(&self, other: &Self) -> bool {
        self.dwTimeOut == other.dwTimeOut && self.jrvRanges == other.jrvRanges && self.jpDeadZone == other.jpDeadZone
    }
}
impl ::core::cmp::Eq for JOYREGUSERVALUES {}
impl ::core::fmt::Debug for JOYREGUSERVALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOYREGUSERVALUES").field("dwTimeOut", &self.dwTimeOut).field("jrvRanges", &self.jrvRanges).field("jpDeadZone", &self.jpDeadZone).finish()
    }
}
impl ::core::default::Default for KEYBOARD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.KeyboardIdentifier == other.KeyboardIdentifier && self.KeyboardMode == other.KeyboardMode && self.NumberOfFunctionKeys == other.NumberOfFunctionKeys && self.NumberOfIndicators == other.NumberOfIndicators && self.NumberOfKeysTotal == other.NumberOfKeysTotal && self.InputDataQueueLength == other.InputDataQueueLength && self.KeyRepeatMinimum == other.KeyRepeatMinimum && self.KeyRepeatMaximum == other.KeyRepeatMaximum
    }
}
impl ::core::cmp::Eq for KEYBOARD_ATTRIBUTES {}
impl ::core::fmt::Debug for KEYBOARD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_ATTRIBUTES").field("KeyboardIdentifier", &self.KeyboardIdentifier).field("KeyboardMode", &self.KeyboardMode).field("NumberOfFunctionKeys", &self.NumberOfFunctionKeys).field("NumberOfIndicators", &self.NumberOfIndicators).field("NumberOfKeysTotal", &self.NumberOfKeysTotal).field("InputDataQueueLength", &self.InputDataQueueLength).field("KeyRepeatMinimum", &self.KeyRepeatMinimum).field("KeyRepeatMaximum", &self.KeyRepeatMaximum).finish()
    }
}
impl ::core::default::Default for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.FormFactor == other.FormFactor && self.KeyType == other.KeyType && self.PhysicalLayout == other.PhysicalLayout && self.VendorSpecificPhysicalLayout == other.VendorSpecificPhysicalLayout && self.IETFLanguageTagIndex == other.IETFLanguageTagIndex && self.ImplementedInputAssistControls == other.ImplementedInputAssistControls
    }
}
impl ::core::cmp::Eq for KEYBOARD_EXTENDED_ATTRIBUTES {}
impl ::core::fmt::Debug for KEYBOARD_EXTENDED_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_EXTENDED_ATTRIBUTES").field("Version", &self.Version).field("FormFactor", &self.FormFactor).field("KeyType", &self.KeyType).field("PhysicalLayout", &self.PhysicalLayout).field("VendorSpecificPhysicalLayout", &self.VendorSpecificPhysicalLayout).field("IETFLanguageTagIndex", &self.IETFLanguageTagIndex).field("ImplementedInputAssistControls", &self.ImplementedInputAssistControls).finish()
    }
}
impl ::core::default::Default for KEYBOARD_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Subtype == other.Subtype
    }
}
impl ::core::cmp::Eq for KEYBOARD_ID {}
impl ::core::fmt::Debug for KEYBOARD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_ID").field("Type", &self.Type).field("Subtype", &self.Subtype).finish()
    }
}
impl ::core::default::Default for KEYBOARD_IME_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_IME_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.ImeOpen == other.ImeOpen && self.ImeConvMode == other.ImeConvMode
    }
}
impl ::core::cmp::Eq for KEYBOARD_IME_STATUS {}
impl ::core::fmt::Debug for KEYBOARD_IME_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_IME_STATUS").field("UnitId", &self.UnitId).field("ImeOpen", &self.ImeOpen).field("ImeConvMode", &self.ImeConvMode).finish()
    }
}
impl ::core::default::Default for KEYBOARD_INDICATOR_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_INDICATOR_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.LedFlags == other.LedFlags
    }
}
impl ::core::cmp::Eq for KEYBOARD_INDICATOR_PARAMETERS {}
impl ::core::fmt::Debug for KEYBOARD_INDICATOR_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_INDICATOR_PARAMETERS").field("UnitId", &self.UnitId).field("LedFlags", &self.LedFlags).finish()
    }
}
impl ::core::default::Default for KEYBOARD_INDICATOR_TRANSLATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_INDICATOR_TRANSLATION {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfIndicatorKeys == other.NumberOfIndicatorKeys && self.IndicatorList == other.IndicatorList
    }
}
impl ::core::cmp::Eq for KEYBOARD_INDICATOR_TRANSLATION {}
impl ::core::fmt::Debug for KEYBOARD_INDICATOR_TRANSLATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_INDICATOR_TRANSLATION").field("NumberOfIndicatorKeys", &self.NumberOfIndicatorKeys).field("IndicatorList", &self.IndicatorList).finish()
    }
}
impl ::core::default::Default for KEYBOARD_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_INPUT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.MakeCode == other.MakeCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ExtraInformation == other.ExtraInformation
    }
}
impl ::core::cmp::Eq for KEYBOARD_INPUT_DATA {}
impl ::core::fmt::Debug for KEYBOARD_INPUT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_INPUT_DATA").field("UnitId", &self.UnitId).field("MakeCode", &self.MakeCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ExtraInformation", &self.ExtraInformation).finish()
    }
}
impl ::core::default::Default for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId && self.Rate == other.Rate && self.Delay == other.Delay
    }
}
impl ::core::cmp::Eq for KEYBOARD_TYPEMATIC_PARAMETERS {}
impl ::core::fmt::Debug for KEYBOARD_TYPEMATIC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_TYPEMATIC_PARAMETERS").field("UnitId", &self.UnitId).field("Rate", &self.Rate).field("Delay", &self.Delay).finish()
    }
}
impl ::core::default::Default for KEYBOARD_UNIT_ID_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for KEYBOARD_UNIT_ID_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId
    }
}
impl ::core::cmp::Eq for KEYBOARD_UNIT_ID_PARAMETER {}
impl ::core::fmt::Debug for KEYBOARD_UNIT_ID_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KEYBOARD_UNIT_ID_PARAMETER").field("UnitId", &self.UnitId).finish()
    }
}
impl ::core::default::Default for MOUSE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MOUSE_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.MouseIdentifier == other.MouseIdentifier && self.NumberOfButtons == other.NumberOfButtons && self.SampleRate == other.SampleRate && self.InputDataQueueLength == other.InputDataQueueLength
    }
}
impl ::core::cmp::Eq for MOUSE_ATTRIBUTES {}
impl ::core::fmt::Debug for MOUSE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_ATTRIBUTES").field("MouseIdentifier", &self.MouseIdentifier).field("NumberOfButtons", &self.NumberOfButtons).field("SampleRate", &self.SampleRate).field("InputDataQueueLength", &self.InputDataQueueLength).finish()
    }
}
impl ::core::default::Default for MOUSE_INPUT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MOUSE_UNIT_ID_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MOUSE_UNIT_ID_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.UnitId == other.UnitId
    }
}
impl ::core::cmp::Eq for MOUSE_UNIT_ID_PARAMETER {}
impl ::core::fmt::Debug for MOUSE_UNIT_ID_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOUSE_UNIT_ID_PARAMETER").field("UnitId", &self.UnitId).finish()
    }
}
impl ::core::default::Default for USAGE_AND_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USAGE_AND_PAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Usage == other.Usage && self.UsagePage == other.UsagePage
    }
}
impl ::core::cmp::Eq for USAGE_AND_PAGE {}
impl ::core::fmt::Debug for USAGE_AND_PAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USAGE_AND_PAGE").field("Usage", &self.Usage).field("UsagePage", &self.UsagePage).finish()
    }
}
