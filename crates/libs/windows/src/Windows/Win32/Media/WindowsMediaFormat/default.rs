impl ::core::default::Default for AM_WMT_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AM_WMT_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.hrStatus == other.hrStatus && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for AM_WMT_EVENT_DATA {}
impl ::core::fmt::Debug for AM_WMT_EVENT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AM_WMT_EVENT_DATA").field("hrStatus", &self.hrStatus).field("pData", &self.pData).finish()
    }
}
impl ::core::default::Default for DRM_COPY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_COPY_OPL {
    fn eq(&self, other: &Self) -> bool {
        self.wMinimumCopyLevel == other.wMinimumCopyLevel && self.oplIdIncludes == other.oplIdIncludes && self.oplIdExcludes == other.oplIdExcludes
    }
}
impl ::core::cmp::Eq for DRM_COPY_OPL {}
impl ::core::fmt::Debug for DRM_COPY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_COPY_OPL").field("wMinimumCopyLevel", &self.wMinimumCopyLevel).field("oplIdIncludes", &self.oplIdIncludes).field("oplIdExcludes", &self.oplIdExcludes).finish()
    }
}
impl ::core::default::Default for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.wCompressedDigitalVideo == other.wCompressedDigitalVideo && self.wUncompressedDigitalVideo == other.wUncompressedDigitalVideo && self.wAnalogVideo == other.wAnalogVideo && self.wCompressedDigitalAudio == other.wCompressedDigitalAudio && self.wUncompressedDigitalAudio == other.wUncompressedDigitalAudio
    }
}
impl ::core::cmp::Eq for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {}
impl ::core::fmt::Debug for DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_MINIMUM_OUTPUT_PROTECTION_LEVELS").field("wCompressedDigitalVideo", &self.wCompressedDigitalVideo).field("wUncompressedDigitalVideo", &self.wUncompressedDigitalVideo).field("wAnalogVideo", &self.wAnalogVideo).field("wCompressedDigitalAudio", &self.wCompressedDigitalAudio).field("wUncompressedDigitalAudio", &self.wUncompressedDigitalAudio).finish()
    }
}
impl ::core::default::Default for DRM_OPL_OUTPUT_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_OPL_OUTPUT_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.cIds == other.cIds && self.rgIds == other.rgIds
    }
}
impl ::core::cmp::Eq for DRM_OPL_OUTPUT_IDS {}
impl ::core::fmt::Debug for DRM_OPL_OUTPUT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OPL_OUTPUT_IDS").field("cIds", &self.cIds).field("rgIds", &self.rgIds).finish()
    }
}
impl ::core::default::Default for DRM_OUTPUT_PROTECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_OUTPUT_PROTECTION {
    fn eq(&self, other: &Self) -> bool {
        self.guidId == other.guidId && self.bConfigData == other.bConfigData
    }
}
impl ::core::cmp::Eq for DRM_OUTPUT_PROTECTION {}
impl ::core::fmt::Debug for DRM_OUTPUT_PROTECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_OUTPUT_PROTECTION").field("guidId", &self.guidId).field("bConfigData", &self.bConfigData).finish()
    }
}
impl ::core::default::Default for DRM_PLAY_OPL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_PLAY_OPL {
    fn eq(&self, other: &Self) -> bool {
        self.minOPL == other.minOPL && self.oplIdReserved == other.oplIdReserved && self.vopi == other.vopi
    }
}
impl ::core::cmp::Eq for DRM_PLAY_OPL {}
impl ::core::fmt::Debug for DRM_PLAY_OPL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_PLAY_OPL").field("minOPL", &self.minOPL).field("oplIdReserved", &self.oplIdReserved).field("vopi", &self.vopi).finish()
    }
}
impl ::core::default::Default for DRM_VAL16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_VAL16 {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
impl ::core::cmp::Eq for DRM_VAL16 {}
impl ::core::fmt::Debug for DRM_VAL16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VAL16").field("val", &self.val).finish()
    }
}
impl ::core::default::Default for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.cEntries == other.cEntries && self.rgVop == other.rgVop
    }
}
impl ::core::cmp::Eq for DRM_VIDEO_OUTPUT_PROTECTION_IDS {}
impl ::core::fmt::Debug for DRM_VIDEO_OUTPUT_PROTECTION_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_VIDEO_OUTPUT_PROTECTION_IDS").field("cEntries", &self.cEntries).field("rgVop", &self.rgVop).finish()
    }
}
impl ::core::cmp::PartialEq for INSNetSourceCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSNetSourceCreator {}
impl ::core::fmt::Debug for INSNetSourceCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSNetSourceCreator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INSSBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer {}
impl ::core::fmt::Debug for INSSBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INSSBuffer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer2 {}
impl ::core::fmt::Debug for INSSBuffer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer2").field(&self.0).finish()
    }
}
impl INSSBuffer2 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLength)(::windows::core::Vtable::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBufferAndLength)(::windows::core::Vtable::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
}
impl ::core::cmp::PartialEq for INSSBuffer3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer3 {}
impl ::core::fmt::Debug for INSSBuffer3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer3").field(&self.0).finish()
    }
}
impl INSSBuffer3 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLength)(::windows::core::Vtable::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaxLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferAndLength)(::windows::core::Vtable::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSampleProperties)(::windows::core::Vtable::as_raw(self), cbproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSampleProperties)(::windows::core::Vtable::as_raw(self), cbproperties, pbproperties).ok()
    }
}
impl ::core::cmp::PartialEq for INSSBuffer4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INSSBuffer4 {}
impl ::core::fmt::Debug for INSSBuffer4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INSSBuffer4").field(&self.0).finish()
    }
}
impl INSSBuffer4 {
    pub unsafe fn GetLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetLength)(::windows::core::Vtable::as_raw(self), dwlength).ok()
    }
    pub unsafe fn GetMaxLength(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaxLength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBuffer(&self) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBufferAndLength)(::windows::core::Vtable::as_raw(self), ppdwbuffer, pdwlength).ok()
    }
    pub unsafe fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSampleProperties)(::windows::core::Vtable::as_raw(self), cbproperties, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSampleProperties)(::windows::core::Vtable::as_raw(self), cbproperties, pbproperties).ok()
    }
    pub unsafe fn SetProperty(&self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidbufferproperty), pvbufferproperty, dwbufferpropertysize).ok()
    }
    pub unsafe fn GetProperty(&self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidbufferproperty), pvbufferproperty, pdwbufferpropertysize).ok()
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess {}
impl ::core::fmt::Debug for IWMAddressAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMAddressAccess2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAddressAccess2 {}
impl ::core::fmt::Debug for IWMAddressAccess2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAddressAccess2").field(&self.0).finish()
    }
}
impl IWMAddressAccess2 {
    pub unsafe fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAccessEntryCount)(::windows::core::Vtable::as_raw(self), aetype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAccessEntry)(::windows::core::Vtable::as_raw(self), aetype, dwentrynum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddAccessEntry)(::windows::core::Vtable::as_raw(self), aetype, paddraccessentry).ok()
    }
    pub unsafe fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAccessEntry)(::windows::core::Vtable::as_raw(self), aetype, dwentrynum).ok()
    }
}
impl ::core::cmp::PartialEq for IWMAuthorizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMAuthorizer {}
impl ::core::fmt::Debug for IWMAuthorizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMAuthorizer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMBackupRestoreProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBackupRestoreProps {}
impl ::core::fmt::Debug for IWMBackupRestoreProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBackupRestoreProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMBandwidthSharing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMBandwidthSharing {}
impl ::core::fmt::Debug for IWMBandwidthSharing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMBandwidthSharing").field(&self.0).finish()
    }
}
impl IWMBandwidthSharing {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStreams)(::windows::core::Vtable::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveStream)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections {}
impl ::core::fmt::Debug for IWMClientConnections {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMClientConnections2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMClientConnections2 {}
impl ::core::fmt::Debug for IWMClientConnections2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMClientConnections2").field(&self.0).finish()
    }
}
impl IWMClientConnections2 {
    pub unsafe fn GetClientCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClientCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClientProperties)(::windows::core::Vtable::as_raw(self), dwclientnum, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo {}
impl ::core::fmt::Debug for IWMCodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo2 {}
impl ::core::fmt::Debug for IWMCodecInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo2").field(&self.0).finish()
    }
}
impl IWMCodecInfo2 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCodecInfoCount)(::windows::core::Vtable::as_raw(self), guidtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCodecFormatCount)(::windows::core::Vtable::as_raw(self), guidtype, dwcodecindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCodecFormat)(::windows::core::Vtable::as_raw(self), guidtype, dwcodecindex, dwformatindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMCodecInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCodecInfo3 {}
impl ::core::fmt::Debug for IWMCodecInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCodecInfo3").field(&self.0).finish()
    }
}
impl IWMCodecInfo3 {
    pub unsafe fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCodecInfoCount)(::windows::core::Vtable::as_raw(self), guidtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCodecFormatCount)(::windows::core::Vtable::as_raw(self), guidtype, dwcodecindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCodecFormat)(::windows::core::Vtable::as_raw(self), guidtype, dwcodecindex, dwformatindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCodecName)(::windows::core::Vtable::as_raw(self), guidtype, dwcodecindex, ::core::mem::transmute(wszname), pcchname).ok()
    }
    pub unsafe fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCodecFormatDesc)(::windows::core::Vtable::as_raw(self), guidtype, dwcodecindex, dwformatindex, ::core::mem::transmute(ppistreamconfig), ::core::mem::transmute(wszdesc), pcchdesc).ok()
    }
}
impl ::core::cmp::PartialEq for IWMCredentialCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMCredentialCallback {}
impl ::core::fmt::Debug for IWMCredentialCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMCredentialCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMEditor {}
impl ::core::fmt::Debug for IWMDRMEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMEditor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMMessageParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMMessageParser {}
impl ::core::fmt::Debug for IWMDRMMessageParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMMessageParser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader {}
impl ::core::fmt::Debug for IWMDRMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader2 {}
impl ::core::fmt::Debug for IWMDRMReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader2").field(&self.0).finish()
    }
}
impl IWMDRMReader2 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AcquireLicense)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelLicenseAcquisition)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Individualize)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelIndividualization)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MonitorLicenseAcquisition)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelMonitorLicenseAcquisition)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDRMProperty)(::windows::core::Vtable::as_raw(self), pwstrname.into().abi(), dwtype, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDRMProperty)(::windows::core::Vtable::as_raw(self), pwstrname.into().abi(), pdwtype, pvalue, pcblength).ok()
    }
}
impl ::core::cmp::PartialEq for IWMDRMReader3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMReader3 {}
impl ::core::fmt::Debug for IWMDRMReader3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMReader3").field(&self.0).finish()
    }
}
impl IWMDRMReader3 {
    pub unsafe fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AcquireLicense)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CancelLicenseAcquisition)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Individualize)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CancelIndividualization(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CancelIndividualization)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MonitorLicenseAcquisition)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CancelMonitorLicenseAcquisition)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetDRMProperty<P0>(&self, pwstrname: P0, dwtype: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDRMProperty)(::windows::core::Vtable::as_raw(self), pwstrname.into().abi(), dwtype, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetDRMProperty<P0>(&self, pwstrname: P0, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDRMProperty)(::windows::core::Vtable::as_raw(self), pwstrname.into().abi(), pdwtype, pvalue, pcblength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEvaluateOutputLevelLicenses<P0>(&self, fevaluate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEvaluateOutputLevelLicenses)(::windows::core::Vtable::as_raw(self), fevaluate.into()).ok()
    }
    pub unsafe fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPlayOutputLevels)(::windows::core::Vtable::as_raw(self), pplayopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCopyOutputLevels)(::windows::core::Vtable::as_raw(self), pcopyopl, pcblength, pdwminappcompliancelevel).ok()
    }
    pub unsafe fn TryNextLicense(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.TryNextLicense)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptionManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptionManager {}
impl ::core::fmt::Debug for IWMDRMTranscryptionManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptionManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor {}
impl ::core::fmt::Debug for IWMDRMTranscryptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMTranscryptor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMTranscryptor2 {}
impl ::core::fmt::Debug for IWMDRMTranscryptor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMTranscryptor2").field(&self.0).finish()
    }
}
impl IWMDRMTranscryptor2 {
    pub unsafe fn Initialize<P0>(&self, bstrfilename: &::windows::core::BSTR, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: P0, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrfilename), pblicenserequestmsg, cblicenserequestmsg, ::core::mem::transmute(pplicenseresponsemsg), pcallback.into().abi(), pvcontext).ok()
    }
    pub unsafe fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), hnstime).ok()
    }
    pub unsafe fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), pbdata, pcbdata).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter {}
impl ::core::fmt::Debug for IWMDRMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter2 {}
impl ::core::fmt::Debug for IWMDRMWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter2").field(&self.0).finish()
    }
}
impl IWMDRMWriter2 {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GenerateKeySeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszkeyseed), pcwchlength).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GenerateKeyID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszkeyid), pcwchlength).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GenerateSigningKeyPair)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszprivkey), pcwchprivkeylength, ::core::mem::transmute(pwszpubkey), pcwchpubkeylength).ok()
    }
    pub unsafe fn SetDRMAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDRMAttribute)(::windows::core::Vtable::as_raw(self), wstreamnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for IWMDRMWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDRMWriter3 {}
impl ::core::fmt::Debug for IWMDRMWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDRMWriter3").field(&self.0).finish()
    }
}
impl IWMDRMWriter3 {
    pub unsafe fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GenerateKeySeed)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszkeyseed), pcwchlength).ok()
    }
    pub unsafe fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GenerateKeyID)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszkeyid), pcwchlength).ok()
    }
    pub unsafe fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GenerateSigningKeyPair)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszprivkey), pcwchprivkeylength, ::core::mem::transmute(pwszpubkey), pcwchpubkeylength).ok()
    }
    pub unsafe fn SetDRMAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDRMAttribute)(::windows::core::Vtable::as_raw(self), wstreamnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWMDRMNetEncryption<P0>(&self, fsamplesencrypted: P0, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWMDRMNetEncryption)(::windows::core::Vtable::as_raw(self), fsamplesencrypted.into(), pbkeyid, cbkeyid).ok()
    }
}
impl ::core::cmp::PartialEq for IWMDeviceRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMDeviceRegistration {}
impl ::core::fmt::Debug for IWMDeviceRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMDeviceRegistration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMGetSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMGetSecureChannel {}
impl ::core::fmt::Debug for IWMGetSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMGetSecureChannel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo {}
impl ::core::fmt::Debug for IWMHeaderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo2 {}
impl ::core::fmt::Debug for IWMHeaderInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo2").field(&self.0).finish()
    }
}
impl IWMHeaderInfo2 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAttributeCount)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAttributeByIndex)(::windows::core::Vtable::as_raw(self), windex, pwstreamnum, ::core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, pwstreamnum: *mut u16, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAttributeByName)(::windows::core::Vtable::as_raw(self), pwstreamnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAttribute)(::windows::core::Vtable::as_raw(self), wstreamnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMarkerCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMarker)(::windows::core::Vtable::as_raw(self), windex, ::core::mem::transmute(pwszmarkername), pcchmarkernamelen, pcnsmarkertime).ok()
    }
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddMarker)(::windows::core::Vtable::as_raw(self), pwszmarkername.into().abi(), cnsmarkertime).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveMarker)(::windows::core::Vtable::as_raw(self), windex).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetScriptCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetScript)(::windows::core::Vtable::as_raw(self), windex, ::core::mem::transmute(pwsztype), pcchtypelen, ::core::mem::transmute(pwszcommand), pcchcommandlen, pcnsscripttime).ok()
    }
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddScript)(::windows::core::Vtable::as_raw(self), pwsztype.into().abi(), pwszcommand.into().abi(), cnsscripttime).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveScript)(::windows::core::Vtable::as_raw(self), windex).ok()
    }
}
impl ::core::cmp::PartialEq for IWMHeaderInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMHeaderInfo3 {}
impl ::core::fmt::Debug for IWMHeaderInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMHeaderInfo3").field(&self.0).finish()
    }
}
impl IWMHeaderInfo3 {
    pub unsafe fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributeCount)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributeByIndex)(::windows::core::Vtable::as_raw(self), windex, pwstreamnum, ::core::mem::transmute(pwszname), pcchnamelen, ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn GetAttributeByName<P0>(&self, pwstreamnum: *mut u16, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAttributeByName)(::windows::core::Vtable::as_raw(self), pwstreamnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetAttribute<P0>(&self, wstreamnum: u16, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAttribute)(::windows::core::Vtable::as_raw(self), wstreamnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetMarkerCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMarkerCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMarker)(::windows::core::Vtable::as_raw(self), windex, ::core::mem::transmute(pwszmarkername), pcchmarkernamelen, pcnsmarkertime).ok()
    }
    pub unsafe fn AddMarker<P0>(&self, pwszmarkername: P0, cnsmarkertime: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddMarker)(::windows::core::Vtable::as_raw(self), pwszmarkername.into().abi(), cnsmarkertime).ok()
    }
    pub unsafe fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveMarker)(::windows::core::Vtable::as_raw(self), windex).ok()
    }
    pub unsafe fn GetScriptCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetScriptCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetScript)(::windows::core::Vtable::as_raw(self), windex, ::core::mem::transmute(pwsztype), pcchtypelen, ::core::mem::transmute(pwszcommand), pcchcommandlen, pcnsscripttime).ok()
    }
    pub unsafe fn AddScript<P0, P1>(&self, pwsztype: P0, pwszcommand: P1, cnsscripttime: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddScript)(::windows::core::Vtable::as_raw(self), pwsztype.into().abi(), pwszcommand.into().abi(), cnsscripttime).ok()
    }
    pub unsafe fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveScript)(::windows::core::Vtable::as_raw(self), windex).ok()
    }
    pub unsafe fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCodecInfoCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCodecInfo)(::windows::core::Vtable::as_raw(self), windex, pcchname, ::core::mem::transmute(pwszname), pcchdescription, ::core::mem::transmute(pwszdescription), pcodectype, pcbcodecinfo, pbcodecinfo).ok()
    }
}
impl ::core::cmp::PartialEq for IWMIStreamProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIStreamProps {}
impl ::core::fmt::Debug for IWMIStreamProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIStreamProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMImageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMImageInfo {}
impl ::core::fmt::Debug for IWMImageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMImageInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMIndexer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer {}
impl ::core::fmt::Debug for IWMIndexer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMIndexer2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMIndexer2 {}
impl ::core::fmt::Debug for IWMIndexer2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIndexer2").field(&self.0).finish()
    }
}
impl IWMIndexer2 {
    pub unsafe fn StartIndexing<P0, P1>(&self, pwszurl: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMStatusCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartIndexing)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), pcallback.into().abi(), pvcontext).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMInputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMInputMediaProps {}
impl ::core::fmt::Debug for IWMInputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMInputMediaProps").field(&self.0).finish()
    }
}
impl IWMInputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMediaType)(::windows::core::Vtable::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMediaType)(::windows::core::Vtable::as_raw(self), ptype).ok()
    }
}
impl ::core::cmp::PartialEq for IWMLanguageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLanguageList {}
impl ::core::fmt::Debug for IWMLanguageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLanguageList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMLicenseBackup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseBackup {}
impl ::core::fmt::Debug for IWMLicenseBackup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseBackup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRestore {}
impl ::core::fmt::Debug for IWMLicenseRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRestore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMLicenseRevocationAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMLicenseRevocationAgent {}
impl ::core::fmt::Debug for IWMLicenseRevocationAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMLicenseRevocationAgent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMediaProps {}
impl ::core::fmt::Debug for IWMMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMediaProps").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor {}
impl ::core::fmt::Debug for IWMMetadataEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMMetadataEditor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMetadataEditor2 {}
impl ::core::fmt::Debug for IWMMetadataEditor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMetadataEditor2").field(&self.0).finish()
    }
}
impl IWMMetadataEditor2 {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Flush(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion {}
impl ::core::fmt::Debug for IWMMutualExclusion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion").field(&self.0).finish()
    }
}
impl IWMMutualExclusion {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStreams)(::windows::core::Vtable::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveStream)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
}
impl ::core::cmp::PartialEq for IWMMutualExclusion2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMMutualExclusion2 {}
impl ::core::fmt::Debug for IWMMutualExclusion2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMMutualExclusion2").field(&self.0).finish()
    }
}
impl IWMMutualExclusion2 {
    pub unsafe fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreams)(::windows::core::Vtable::as_raw(self), pwstreamnumarray, pcstreams).ok()
    }
    pub unsafe fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddStream)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveStream)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetType)(::windows::core::Vtable::as_raw(self), guidtype).ok()
    }
}
impl ::core::cmp::PartialEq for IWMOutputMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMOutputMediaProps {}
impl ::core::fmt::Debug for IWMOutputMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMOutputMediaProps").field(&self.0).finish()
    }
}
impl IWMOutputMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMediaType)(::windows::core::Vtable::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMediaType)(::windows::core::Vtable::as_raw(self), ptype).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize {}
impl ::core::fmt::Debug for IWMPacketSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPacketSize2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPacketSize2 {}
impl ::core::fmt::Debug for IWMPacketSize2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPacketSize2").field(&self.0).finish()
    }
}
impl IWMPacketSize2 {
    pub unsafe fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxPacketSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaxPacketSize)(::windows::core::Vtable::as_raw(self), dwmaxpacketsize).ok()
    }
}
impl ::core::cmp::PartialEq for IWMPlayerHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerHook {}
impl ::core::fmt::Debug for IWMPlayerHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerHook").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPlayerTimestampHook {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPlayerTimestampHook {}
impl ::core::fmt::Debug for IWMPlayerTimestampHook {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPlayerTimestampHook").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile {}
impl ::core::fmt::Debug for IWMProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMProfile2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile2 {}
impl ::core::fmt::Debug for IWMProfile2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile2").field(&self.0).finish()
    }
}
impl IWMProfile2 {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), pwszname.into().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszdescription), pcchdescription).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), pwszdescription.into().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), dwstreamindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamByNumber)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStreamConfig>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveStream)(::windows::core::Vtable::as_raw(self), pconfig.into().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveStreamByNumber)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStreamConfig>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddStream)(::windows::core::Vtable::as_raw(self), pconfig.into().abi()).ok()
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStreamConfig>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReconfigStream)(::windows::core::Vtable::as_raw(self), pconfig.into().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateNewStream)(::windows::core::Vtable::as_raw(self), guidstreamtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMutualExclusionCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMutualExclusion)(::windows::core::Vtable::as_raw(self), dwmeindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMMutualExclusion>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveMutualExclusion)(::windows::core::Vtable::as_raw(self), pme.into().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMMutualExclusion>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddMutualExclusion)(::windows::core::Vtable::as_raw(self), pme.into().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateNewMutualExclusion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMProfile3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfile3 {}
impl ::core::fmt::Debug for IWMProfile3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfile3").field(&self.0).finish()
    }
}
impl IWMProfile3 {
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszname), pcchname).ok()
    }
    pub unsafe fn SetName<P0>(&self, pwszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), pwszname.into().abi()).ok()
    }
    pub unsafe fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszdescription), pcchdescription).ok()
    }
    pub unsafe fn SetDescription<P0>(&self, pwszdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescription)(::windows::core::Vtable::as_raw(self), pwszdescription.into().abi()).ok()
    }
    pub unsafe fn GetStreamCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStream)(::windows::core::Vtable::as_raw(self), dwstreamindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamByNumber)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStreamConfig>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveStream)(::windows::core::Vtable::as_raw(self), pconfig.into().abi()).ok()
    }
    pub unsafe fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveStreamByNumber)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn AddStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStreamConfig>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddStream)(::windows::core::Vtable::as_raw(self), pconfig.into().abi()).ok()
    }
    pub unsafe fn ReconfigStream<P0>(&self, pconfig: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMStreamConfig>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ReconfigStream)(::windows::core::Vtable::as_raw(self), pconfig.into().abi()).ok()
    }
    pub unsafe fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateNewStream)(::windows::core::Vtable::as_raw(self), guidstreamtype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMutualExclusionCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMutualExclusion)(::windows::core::Vtable::as_raw(self), dwmeindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMMutualExclusion>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveMutualExclusion)(::windows::core::Vtable::as_raw(self), pme.into().abi()).ok()
    }
    pub unsafe fn AddMutualExclusion<P0>(&self, pme: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMMutualExclusion>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddMutualExclusion)(::windows::core::Vtable::as_raw(self), pme.into().abi()).ok()
    }
    pub unsafe fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateNewMutualExclusion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProfileID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager {}
impl ::core::fmt::Debug for IWMProfileManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMProfileManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManager2 {}
impl ::core::fmt::Debug for IWMProfileManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManager2").field(&self.0).finish()
    }
}
impl IWMProfileManager2 {
    pub unsafe fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEmptyProfile)(::windows::core::Vtable::as_raw(self), dwversion, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LoadProfileByID)(::windows::core::Vtable::as_raw(self), guidprofile, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadProfileByData<P0>(&self, pwszprofile: P0) -> ::windows::core::Result<IWMProfile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LoadProfileByData)(::windows::core::Vtable::as_raw(self), pwszprofile.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SaveProfile<P0, P1>(&self, piwmprofile: P0, pwszprofile: P1, pdwlength: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMProfile>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveProfile)(::windows::core::Vtable::as_raw(self), piwmprofile.into().abi(), pwszprofile.into().abi(), pdwlength).ok()
    }
    pub unsafe fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSystemProfileCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LoadSystemProfile)(::windows::core::Vtable::as_raw(self), dwprofileindex, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMProfileManagerLanguage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProfileManagerLanguage {}
impl ::core::fmt::Debug for IWMProfileManagerLanguage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProfileManagerLanguage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMPropertyVault {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMPropertyVault {}
impl ::core::fmt::Debug for IWMPropertyVault {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMPropertyVault").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMProximityDetection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMProximityDetection {}
impl ::core::fmt::Debug for IWMProximityDetection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMProximityDetection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReader {}
impl ::core::fmt::Debug for IWMReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAccelerator {}
impl ::core::fmt::Debug for IWMReaderAccelerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAccelerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced {}
impl ::core::fmt::Debug for IWMReaderAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced2 {}
impl ::core::fmt::Debug for IWMReaderAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced2").field(&self.0).finish()
    }
}
impl IWMReaderAdvanced2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUserProvidedClock)(::windows::core::Vtable::as_raw(self), fuserclock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUserProvidedClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeliverTime)(::windows::core::Vtable::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetManualStreamSelection)(::windows::core::Vtable::as_raw(self), fselection.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetManualStreamSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStreamsSelected)(::windows::core::Vtable::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamSelected)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), fgetcallbacks.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, freceivestreamsamples.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAllocateForStream)(::windows::core::Vtable::as_raw(self), wstreamnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAllocateForStream)(::windows::core::Vtable::as_raw(self), dwsreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStatistics)(::windows::core::Vtable::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientInfo)(::windows::core::Vtable::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxOutputSampleSize)(::windows::core::Vtable::as_raw(self), dwoutput, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxStreamSampleSize)(::windows::core::Vtable::as_raw(self), wstream, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NotifyLateDelivery)(::windows::core::Vtable::as_raw(self), cnslateness).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced3 {}
impl ::core::fmt::Debug for IWMReaderAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced3").field(&self.0).finish()
    }
}
impl IWMReaderAdvanced3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUserProvidedClock)(::windows::core::Vtable::as_raw(self), fuserclock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetUserProvidedClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeliverTime)(::windows::core::Vtable::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetManualStreamSelection)(::windows::core::Vtable::as_raw(self), fselection.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetManualStreamSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetStreamsSelected)(::windows::core::Vtable::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamSelected)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), fgetcallbacks.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, freceivestreamsamples.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAllocateForStream)(::windows::core::Vtable::as_raw(self), wstreamnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAllocateForStream)(::windows::core::Vtable::as_raw(self), dwsreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStatistics)(::windows::core::Vtable::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetClientInfo)(::windows::core::Vtable::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaxOutputSampleSize)(::windows::core::Vtable::as_raw(self), dwoutput, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaxStreamSampleSize)(::windows::core::Vtable::as_raw(self), wstream, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.NotifyLateDelivery)(::windows::core::Vtable::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPlayMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPlayMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBufferProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDownloadProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSaveAsProgress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SaveFileAs)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProtocolName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartAtMarker)(::windows::core::Vtable::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Preroll)(::windows::core::Vtable::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLogClientID)(::windows::core::Vtable::as_raw(self), flogclientid.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLogClientID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StopBuffering)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMReaderCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenStream)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pcallback.into().abi(), pvcontext).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced4 {}
impl ::core::fmt::Debug for IWMReaderAdvanced4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced4").field(&self.0).finish()
    }
}
impl IWMReaderAdvanced4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUserProvidedClock)(::windows::core::Vtable::as_raw(self), fuserclock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetUserProvidedClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DeliverTime)(::windows::core::Vtable::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetManualStreamSelection)(::windows::core::Vtable::as_raw(self), fselection.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetManualStreamSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetStreamsSelected)(::windows::core::Vtable::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStreamSelected)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), fgetcallbacks.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, freceivestreamsamples.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAllocateForStream)(::windows::core::Vtable::as_raw(self), wstreamnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAllocateForStream)(::windows::core::Vtable::as_raw(self), dwsreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStatistics)(::windows::core::Vtable::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetClientInfo)(::windows::core::Vtable::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaxOutputSampleSize)(::windows::core::Vtable::as_raw(self), dwoutput, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaxStreamSampleSize)(::windows::core::Vtable::as_raw(self), wstream, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NotifyLateDelivery)(::windows::core::Vtable::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPlayMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPlayMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDownloadProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSaveAsProgress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SaveFileAs)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetProtocolName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StartAtMarker)(::windows::core::Vtable::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Preroll)(::windows::core::Vtable::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLogClientID)(::windows::core::Vtable::as_raw(self), flogclientid.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLogClientID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StopBuffering)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMReaderCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenStream)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pcallback.into().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StopNetStreaming)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StartAtPosition)(::windows::core::Vtable::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced5 {}
impl ::core::fmt::Debug for IWMReaderAdvanced5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced5").field(&self.0).finish()
    }
}
impl IWMReaderAdvanced5 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetUserProvidedClock)(::windows::core::Vtable::as_raw(self), fuserclock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetUserProvidedClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DeliverTime)(::windows::core::Vtable::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetManualStreamSelection)(::windows::core::Vtable::as_raw(self), fselection.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetManualStreamSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetStreamsSelected)(::windows::core::Vtable::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetStreamSelected)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), fgetcallbacks.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, freceivestreamsamples.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetAllocateForStream)(::windows::core::Vtable::as_raw(self), wstreamnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetAllocateForStream)(::windows::core::Vtable::as_raw(self), dwsreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetStatistics)(::windows::core::Vtable::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetClientInfo)(::windows::core::Vtable::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMaxOutputSampleSize)(::windows::core::Vtable::as_raw(self), dwoutput, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetMaxStreamSampleSize)(::windows::core::Vtable::as_raw(self), wstream, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.NotifyLateDelivery)(::windows::core::Vtable::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPlayMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPlayMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBufferProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDownloadProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSaveAsProgress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SaveFileAs)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetProtocolName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StartAtMarker)(::windows::core::Vtable::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Preroll)(::windows::core::Vtable::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetLogClientID)(::windows::core::Vtable::as_raw(self), flogclientid.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLogClientID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StopBuffering)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMReaderCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenStream)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pcallback.into().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StopNetStreaming)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.StartAtPosition)(::windows::core::Vtable::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguageCount)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLanguage)(::windows::core::Vtable::as_raw(self), dwoutputnum, wlanguage, ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxSpeedFactor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsUsingFastCache)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddLogParam)(::windows::core::Vtable::as_raw(self), wsznamespace.into().abi(), wszname.into().abi(), wszvalue.into().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SendLogParams)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CanSaveFileAs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CancelSaveFileAs)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAdvanced6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAdvanced6 {}
impl ::core::fmt::Debug for IWMReaderAdvanced6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAdvanced6").field(&self.0).finish()
    }
}
impl IWMReaderAdvanced6 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserProvidedClock<P0>(&self, fuserclock: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetUserProvidedClock)(::windows::core::Vtable::as_raw(self), fuserclock.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetUserProvidedClock)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DeliverTime)(::windows::core::Vtable::as_raw(self), cnstime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetManualStreamSelection<P0>(&self, fselection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetManualStreamSelection)(::windows::core::Vtable::as_raw(self), fselection.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetManualStreamSelection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetStreamsSelected)(::windows::core::Vtable::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetStreamSelected)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveSelectionCallbacks<P0>(&self, fgetcallbacks: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), fgetcallbacks.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetReceiveSelectionCallbacks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReceiveStreamSamples<P0>(&self, wstreamnum: u16, freceivestreamsamples: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, freceivestreamsamples.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetReceiveStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForOutput<P0>(&self, dwoutputnum: u32, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetAllocateForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllocateForStream<P0>(&self, wstreamnum: u16, fallocate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetAllocateForStream)(::windows::core::Vtable::as_raw(self), wstreamnum, fallocate.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetAllocateForStream)(::windows::core::Vtable::as_raw(self), dwsreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetStatistics)(::windows::core::Vtable::as_raw(self), pstatistics).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetClientInfo)(::windows::core::Vtable::as_raw(self), pclientinfo).ok()
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetMaxOutputSampleSize)(::windows::core::Vtable::as_raw(self), dwoutput, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetMaxStreamSampleSize)(::windows::core::Vtable::as_raw(self), wstream, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.NotifyLateDelivery)(::windows::core::Vtable::as_raw(self), cnslateness).ok()
    }
    pub unsafe fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPlayMode)(::windows::core::Vtable::as_raw(self), mode).ok()
    }
    pub unsafe fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPlayMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetBufferProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pcnsbuffering).ok()
    }
    pub unsafe fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDownloadProgress)(::windows::core::Vtable::as_raw(self), pdwpercent, pqwbytesdownloaded, pcnsdownload).ok()
    }
    pub unsafe fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSaveAsProgress)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SaveFileAs<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SaveFileAs)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetProtocolName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszprotocol), pcchprotocol).ok()
    }
    pub unsafe fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.StartAtMarker)(::windows::core::Vtable::as_raw(self), wmarkerindex, cnsduration, frate, pvcontext).ok()
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Preroll)(::windows::core::Vtable::as_raw(self), cnsstart, cnsduration, frate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLogClientID<P0>(&self, flogclientid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetLogClientID)(::windows::core::Vtable::as_raw(self), flogclientid.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLogClientID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn StopBuffering(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.StopBuffering)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0, P1>(&self, pstream: P0, pcallback: P1, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWMReaderCallback>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenStream)(::windows::core::Vtable::as_raw(self), pstream.into().abi(), pcallback.into().abi(), pvcontext).ok()
    }
    pub unsafe fn StopNetStreaming(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StopNetStreaming)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.StartAtPosition)(::windows::core::Vtable::as_raw(self), wstreamnum, pvoffsetstart, pvduration, dwoffsetformat, frate, pvcontext).ok()
    }
    pub unsafe fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLanguageCount)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLanguage)(::windows::core::Vtable::as_raw(self), dwoutputnum, wlanguage, ::core::mem::transmute(pwszlanguagestring), pcchlanguagestringlength).ok()
    }
    pub unsafe fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaxSpeedFactor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsUsingFastCache)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddLogParam<P0, P1, P2>(&self, wsznamespace: P0, wszname: P1, wszvalue: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddLogParam)(::windows::core::Vtable::as_raw(self), wsznamespace.into().abi(), wszname.into().abi(), wszvalue.into().abi()).ok()
    }
    pub unsafe fn SendLogParams(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SendLogParams)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CanSaveFileAs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CancelSaveFileAs(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CancelSaveFileAs)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn SetPlayerHook<P0>(&self, dwoutputnum: u32, phook: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMPlayerHook>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPlayerHook)(::windows::core::Vtable::as_raw(self), dwoutputnum, phook.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderAllocatorEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderAllocatorEx {}
impl ::core::fmt::Debug for IWMReaderAllocatorEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderAllocatorEx").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallback {}
impl ::core::fmt::Debug for IWMReaderCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallback").field(&self.0).finish()
    }
}
impl IWMReaderCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnStatus)(::windows::core::Vtable::as_raw(self), status, hr, dwtype, pvalue, pvcontext).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderCallbackAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderCallbackAdvanced {}
impl ::core::fmt::Debug for IWMReaderCallbackAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderCallbackAdvanced").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderNetworkConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderNetworkConfig2 {}
impl ::core::fmt::Debug for IWMReaderNetworkConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderNetworkConfig2").field(&self.0).finish()
    }
}
impl IWMReaderNetworkConfig2 {
    pub unsafe fn GetBufferingTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBufferingTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBufferingTime)(::windows::core::Vtable::as_raw(self), cnsbufferingtime).ok()
    }
    pub unsafe fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUDPPortRanges)(::windows::core::Vtable::as_raw(self), prangearray, pcranges).ok()
    }
    pub unsafe fn SetUDPPortRanges(&self, prangearray: &[WM_PORT_NUMBER_RANGE]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUDPPortRanges)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(prangearray.as_ptr()), prangearray.len() as _).ok()
    }
    pub unsafe fn GetProxySettings<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<WMT_PROXY_SETTINGS>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProxySettings)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProxySettings<P0>(&self, pwszprotocol: P0, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProxySettings)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), proxysetting).ok()
    }
    pub unsafe fn GetProxyHostName<P0>(&self, pwszprotocol: P0, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetProxyHostName)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), ::core::mem::transmute(pwszhostname), pcchhostname).ok()
    }
    pub unsafe fn SetProxyHostName<P0, P1>(&self, pwszprotocol: P0, pwszhostname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProxyHostName)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), pwszhostname.into().abi()).ok()
    }
    pub unsafe fn GetProxyPort<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProxyPort)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProxyPort<P0>(&self, pwszprotocol: P0, dwport: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProxyPort)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), dwport).ok()
    }
    pub unsafe fn GetProxyExceptionList<P0>(&self, pwszprotocol: P0, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetProxyExceptionList)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), ::core::mem::transmute(pwszexceptionlist), pcchexceptionlist).ok()
    }
    pub unsafe fn SetProxyExceptionList<P0, P1>(&self, pwszprotocol: P0, pwszexceptionlist: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProxyExceptionList)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), pwszexceptionlist.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProxyBypassForLocal<P0>(&self, pwszprotocol: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProxyBypassForLocal)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxyBypassForLocal<P0, P1>(&self, pwszprotocol: P0, fbypassforlocal: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProxyBypassForLocal)(::windows::core::Vtable::as_raw(self), pwszprotocol.into().abi(), fbypassforlocal.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetForceRerunAutoProxyDetection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetForceRerunAutoProxyDetection<P0>(&self, fforcererundetection: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetForceRerunAutoProxyDetection)(::windows::core::Vtable::as_raw(self), fforcererundetection.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnableMulticast)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableMulticast<P0>(&self, fenablemulticast: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableMulticast)(::windows::core::Vtable::as_raw(self), fenablemulticast.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnableHTTP)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableHTTP<P0>(&self, fenablehttp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableHTTP)(::windows::core::Vtable::as_raw(self), fenablehttp.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnableUDP)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableUDP<P0>(&self, fenableudp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableUDP)(::windows::core::Vtable::as_raw(self), fenableudp.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnableTCP)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableTCP<P0>(&self, fenabletcp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableTCP)(::windows::core::Vtable::as_raw(self), fenabletcp.into()).ok()
    }
    pub unsafe fn ResetProtocolRollover(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetProtocolRollover)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetConnectionBandwidth)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetConnectionBandwidth)(::windows::core::Vtable::as_raw(self), dwconnectionbandwidth).ok()
    }
    pub unsafe fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNumProtocolsSupported)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSupportedProtocolName)(::windows::core::Vtable::as_raw(self), dwprotocolnum, ::core::mem::transmute(pwszprotocolname), pcchprotocolname).ok()
    }
    pub unsafe fn AddLoggingUrl<P0>(&self, pwszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddLoggingUrl)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi()).ok()
    }
    pub unsafe fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLoggingUrl)(::windows::core::Vtable::as_raw(self), dwindex, ::core::mem::transmute(pwszurl), pcchurl).ok()
    }
    pub unsafe fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLoggingUrlCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetLoggingUrlList)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMReaderPlaylistBurn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderPlaylistBurn {}
impl ::core::fmt::Debug for IWMReaderPlaylistBurn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderPlaylistBurn").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderStreamClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderStreamClock {}
impl ::core::fmt::Debug for IWMReaderStreamClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderStreamClock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderTimecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTimecode {}
impl ::core::fmt::Debug for IWMReaderTimecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTimecode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMReaderTypeNegotiation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMReaderTypeNegotiation {}
impl ::core::fmt::Debug for IWMReaderTypeNegotiation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMReaderTypeNegotiation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMRegisterCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisterCallback {}
impl ::core::fmt::Debug for IWMRegisterCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisterCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMRegisteredDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMRegisteredDevice {}
impl ::core::fmt::Debug for IWMRegisteredDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMRegisteredDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSBufferAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSBufferAllocator {}
impl ::core::fmt::Debug for IWMSBufferAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSBufferAllocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource2 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSInternalAdminNetSource3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSInternalAdminNetSource3 {}
impl ::core::fmt::Debug for IWMSInternalAdminNetSource3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSInternalAdminNetSource3").field(&self.0).finish()
    }
}
impl IWMSInternalAdminNetSource3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCredentialsEx<P0, P1, P2>(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: P0, bstrname: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, fpersist: P1, fconfirmedgood: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCredentialsEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrealm), ::core::mem::transmute_copy(bstrurl), fproxy.into(), ::core::mem::transmute_copy(bstrname), ::core::mem::transmute_copy(bstrpassword), fpersist.into(), fconfirmedgood.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCredentialsEx<P0>(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: P0, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetCredentialsEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrealm), ::core::mem::transmute_copy(bstrurl), fproxy.into(), pdwurlpolicy, ::core::mem::transmute(pbstrname), ::core::mem::transmute(pbstrpassword), pfconfirmedgood).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteCredentialsEx<P0>(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteCredentialsEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrrealm), ::core::mem::transmute_copy(bstrurl), fproxy.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindProxyForURLEx(&self, bstrprotocol: &::windows::core::BSTR, bstrhost: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FindProxyForURLEx)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprotocol), ::core::mem::transmute_copy(bstrhost), ::core::mem::transmute_copy(bstrurl), pfproxyenabled, ::core::mem::transmute(pbstrproxyserver), pdwproxyport, pdwproxycontext).ok()
    }
}
impl ::core::cmp::PartialEq for IWMSecureChannel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSecureChannel {}
impl ::core::fmt::Debug for IWMSecureChannel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSecureChannel").field(&self.0).finish()
    }
}
impl IWMSecureChannel {
    pub unsafe fn GetCertCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCertCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCert)(::windows::core::Vtable::as_raw(self), dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSharedData)(::windows::core::Vtable::as_raw(self), dwcertindex, pbshareddata, pbcert, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMStatusCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStatusCallback {}
impl ::core::fmt::Debug for IWMStatusCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStatusCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig {}
impl ::core::fmt::Debug for IWMStreamConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig2 {}
impl ::core::fmt::Debug for IWMStreamConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig2").field(&self.0).finish()
    }
}
impl IWMStreamConfig2 {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStreamNumber)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStreamName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszstreamname), pcchstreamname).ok()
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStreamName)(::windows::core::Vtable::as_raw(self), pwszstreamname.into().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetConnectionName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszinputname), pcchinputname).ok()
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetConnectionName)(::windows::core::Vtable::as_raw(self), pwszinputname.into().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitrate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBitrate)(::windows::core::Vtable::as_raw(self), pdwbitrate).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBufferWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBufferWindow)(::windows::core::Vtable::as_raw(self), msbufferwindow).ok()
    }
}
impl ::core::cmp::PartialEq for IWMStreamConfig3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamConfig3 {}
impl ::core::fmt::Debug for IWMStreamConfig3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamConfig3").field(&self.0).finish()
    }
}
impl IWMStreamConfig3 {
    pub unsafe fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamNumber(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetStreamNumber)(::windows::core::Vtable::as_raw(self), wstreamnum).ok()
    }
    pub unsafe fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStreamName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszstreamname), pcchstreamname).ok()
    }
    pub unsafe fn SetStreamName<P0>(&self, pwszstreamname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetStreamName)(::windows::core::Vtable::as_raw(self), pwszstreamname.into().abi()).ok()
    }
    pub unsafe fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetConnectionName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwszinputname), pcchinputname).ok()
    }
    pub unsafe fn SetConnectionName<P0>(&self, pwszinputname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetConnectionName)(::windows::core::Vtable::as_raw(self), pwszinputname.into().abi()).ok()
    }
    pub unsafe fn GetBitrate(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBitrate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBitrate)(::windows::core::Vtable::as_raw(self), pdwbitrate).ok()
    }
    pub unsafe fn GetBufferWindow(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBufferWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBufferWindow)(::windows::core::Vtable::as_raw(self), msbufferwindow).ok()
    }
    pub unsafe fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransportType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTransportType)(::windows::core::Vtable::as_raw(self), ntransporttype).ok()
    }
    pub unsafe fn AddDataUnitExtension(&self, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddDataUnitExtension)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(guidextensionsystemid), cbextensiondatasize, ::core::mem::transmute(pbextensionsysteminfo.as_ptr()), pbextensionsysteminfo.len() as _).ok()
    }
    pub unsafe fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDataUnitExtensionCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDataUnitExtension)(::windows::core::Vtable::as_raw(self), wdataunitextensionnumber, pguidextensionsystemid, pcbextensiondatasize, pbextensionsysteminfo, pcbextensionsysteminfo).ok()
    }
    pub unsafe fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RemoveAllDataUnitExtensions)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMStreamList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamList {}
impl ::core::fmt::Debug for IWMStreamList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMStreamPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMStreamPrioritization {}
impl ::core::fmt::Debug for IWMStreamPrioritization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMStreamPrioritization").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader {}
impl ::core::fmt::Debug for IWMSyncReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMSyncReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMSyncReader2 {}
impl ::core::fmt::Debug for IWMSyncReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMSyncReader2").field(&self.0).finish()
    }
}
impl IWMSyncReader2 {
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRange)(::windows::core::Vtable::as_raw(self), cnsstarttime, cnsduration).ok()
    }
    pub unsafe fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRangeByFrame)(::windows::core::Vtable::as_raw(self), wstreamnum, qwframenumber, cframestoread).ok()
    }
    pub unsafe fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNextSample)(::windows::core::Vtable::as_raw(self), wstreamnum, ::core::mem::transmute(ppsample), pcnssampletime, pcnsduration, pdwflags, pdwoutputnum, pwstreamnum).ok()
    }
    pub unsafe fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetStreamsSelected)(::windows::core::Vtable::as_raw(self), cstreamcount, pwstreamnumbers, pselections).ok()
    }
    pub unsafe fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamSelected)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReadStreamSamples<P0>(&self, wstreamnum: u16, fcompressed: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetReadStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, fcompressed.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReadStreamSamples)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetOutputSetting<P0>(&self, dwoutputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputSetting)(::windows::core::Vtable::as_raw(self), dwoutputnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
    pub unsafe fn GetOutputCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputProps)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOutputProps<P0>(&self, dwoutputnum: u32, poutput: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMOutputMediaProps>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOutputProps)(::windows::core::Vtable::as_raw(self), dwoutputnum, poutput.into().abi()).ok()
    }
    pub unsafe fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputFormatCount)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputFormat)(::windows::core::Vtable::as_raw(self), dwoutputnum, dwformatnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOutputNumberForStream)(::windows::core::Vtable::as_raw(self), wstreamnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStreamNumberForOutput)(::windows::core::Vtable::as_raw(self), dwoutputnum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxOutputSampleSize)(::windows::core::Vtable::as_raw(self), dwoutput, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaxStreamSampleSize)(::windows::core::Vtable::as_raw(self), wstream, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenStream<P0>(&self, pstream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenStream)(::windows::core::Vtable::as_raw(self), pstream.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWMVideoMediaProps {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMVideoMediaProps {}
impl ::core::fmt::Debug for IWMVideoMediaProps {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMVideoMediaProps").field(&self.0).finish()
    }
}
impl IWMVideoMediaProps {
    pub unsafe fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMediaType)(::windows::core::Vtable::as_raw(self), ptype, pcbtype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMediaType)(::windows::core::Vtable::as_raw(self), ptype).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWatermarkInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWatermarkInfo {}
impl ::core::fmt::Debug for IWMWatermarkInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWatermarkInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriter {}
impl ::core::fmt::Debug for IWMWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced {}
impl ::core::fmt::Debug for IWMWriterAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced2 {}
impl ::core::fmt::Debug for IWMWriterAdvanced2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced2").field(&self.0).finish()
    }
}
impl IWMWriterAdvanced2 {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSinkCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSink)(::windows::core::Vtable::as_raw(self), dwsinknum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMWriterSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddSink)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMWriterSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveSink)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.WriteStreamSample)(::windows::core::Vtable::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<P0>(&self, fislivesource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLiveSource)(::windows::core::Vtable::as_raw(self), fislivesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWriterTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStatistics)(::windows::core::Vtable::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSyncTolerance)(::windows::core::Vtable::as_raw(self), mswindow).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSyncTolerance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMWriterAdvanced3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterAdvanced3 {}
impl ::core::fmt::Debug for IWMWriterAdvanced3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterAdvanced3").field(&self.0).finish()
    }
}
impl IWMWriterAdvanced3 {
    pub unsafe fn GetSinkCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSinkCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSink)(::windows::core::Vtable::as_raw(self), dwsinknum, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMWriterSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddSink)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn RemoveSink<P0>(&self, psink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWMWriterSink>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveSink)(::windows::core::Vtable::as_raw(self), psink.into().abi()).ok()
    }
    pub unsafe fn WriteStreamSample<P0>(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteStreamSample)(::windows::core::Vtable::as_raw(self), wstreamnum, cnssampletime, mssamplesendtime, cnssampleduration, dwflags, psample.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLiveSource<P0>(&self, fislivesource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetLiveSource)(::windows::core::Vtable::as_raw(self), fislivesource.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetWriterTime(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWriterTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStatistics)(::windows::core::Vtable::as_raw(self), wstreamnum, pstats).ok()
    }
    pub unsafe fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSyncTolerance)(::windows::core::Vtable::as_raw(self), mswindow).ok()
    }
    pub unsafe fn GetSyncTolerance(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSyncTolerance)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetInputSetting)(::windows::core::Vtable::as_raw(self), dwinputnum, pszname.into().abi(), ptype, pvalue, pcblength).ok()
    }
    pub unsafe fn SetInputSetting<P0>(&self, dwinputnum: u32, pszname: P0, r#type: WMT_ATTR_DATATYPE, pvalue: &[u8]) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetInputSetting)(::windows::core::Vtable::as_raw(self), dwinputnum, pszname.into().abi(), r#type, ::core::mem::transmute(pvalue.as_ptr()), pvalue.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink {}
impl ::core::fmt::Debug for IWMWriterFileSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink").field(&self.0).finish()
    }
}
impl IWMWriterFileSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnHeader)(::windows::core::Vtable::as_raw(self), pheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AllocateDataUnit)(::windows::core::Vtable::as_raw(self), cbdataunit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnDataUnit)(::windows::core::Vtable::as_raw(self), pdataunit.into().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnEndWriting)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink2 {}
impl ::core::fmt::Debug for IWMWriterFileSink2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink2").field(&self.0).finish()
    }
}
impl IWMWriterFileSink2 {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnHeader)(::windows::core::Vtable::as_raw(self), pheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AllocateDataUnit)(::windows::core::Vtable::as_raw(self), cbdataunit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OnDataUnit)(::windows::core::Vtable::as_raw(self), pdataunit.into().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OnEndWriting)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWriterFileSink3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterFileSink3 {}
impl ::core::fmt::Debug for IWMWriterFileSink3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterFileSink3").field(&self.0).finish()
    }
}
impl IWMWriterFileSink3 {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OnHeader)(::windows::core::Vtable::as_raw(self), pheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AllocateDataUnit)(::windows::core::Vtable::as_raw(self), cbdataunit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OnDataUnit)(::windows::core::Vtable::as_raw(self), pdataunit.into().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OnEndWriting)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Open<P0>(&self, pwszfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Open)(::windows::core::Vtable::as_raw(self), pwszfilename.into().abi()).ok()
    }
    pub unsafe fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self), cnsstarttime).ok()
    }
    pub unsafe fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stop)(::windows::core::Vtable::as_raw(self), cnsstoptime).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsStopped)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileDuration(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileDuration)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsClosed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWMWriterNetworkSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterNetworkSink {}
impl ::core::fmt::Debug for IWMWriterNetworkSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterNetworkSink").field(&self.0).finish()
    }
}
impl IWMWriterNetworkSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnHeader)(::windows::core::Vtable::as_raw(self), pheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AllocateDataUnit)(::windows::core::Vtable::as_raw(self), cbdataunit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnDataUnit)(::windows::core::Vtable::as_raw(self), pdataunit.into().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnEndWriting)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostView {}
impl ::core::fmt::Debug for IWMWriterPostView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMWriterPostViewCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPostViewCallback {}
impl ::core::fmt::Debug for IWMWriterPostViewCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPostViewCallback").field(&self.0).finish()
    }
}
impl IWMWriterPostViewCallback {
    pub unsafe fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnStatus)(::windows::core::Vtable::as_raw(self), status, hr, dwtype, pvalue, pvcontext).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWriterPreprocess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPreprocess {}
impl ::core::fmt::Debug for IWMWriterPreprocess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPreprocess").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWMWriterPushSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterPushSink {}
impl ::core::fmt::Debug for IWMWriterPushSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterPushSink").field(&self.0).finish()
    }
}
impl IWMWriterPushSink {
    pub unsafe fn OnHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnHeader)(::windows::core::Vtable::as_raw(self), pheader.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsRealTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AllocateDataUnit)(::windows::core::Vtable::as_raw(self), cbdataunit, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OnDataUnit<P0>(&self, pdataunit: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<INSSBuffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OnDataUnit)(::windows::core::Vtable::as_raw(self), pdataunit.into().abi()).ok()
    }
    pub unsafe fn OnEndWriting(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnEndWriting)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWMWriterSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWMWriterSink {}
impl ::core::fmt::Debug for IWMWriterSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMWriterSink").field(&self.0).finish()
    }
}
impl ::core::default::Default for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NETSOURCE_URLCREDPOLICY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETSOURCE_URLCREDPOLICY_SETTINGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WEBSTREAM_SAMPLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEBSTREAM_SAMPLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEBSTREAM_SAMPLE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMDRM_IMPORT_INIT_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMDRM_IMPORT_INIT_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.cbEncryptedSessionKeyMessage == other.cbEncryptedSessionKeyMessage && self.pbEncryptedSessionKeyMessage == other.pbEncryptedSessionKeyMessage && self.cbEncryptedKeyMessage == other.cbEncryptedKeyMessage && self.pbEncryptedKeyMessage == other.pbEncryptedKeyMessage
    }
}
impl ::core::cmp::Eq for WMDRM_IMPORT_INIT_STRUCT {}
impl ::core::fmt::Debug for WMDRM_IMPORT_INIT_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMDRM_IMPORT_INIT_STRUCT").field("dwVersion", &self.dwVersion).field("cbEncryptedSessionKeyMessage", &self.cbEncryptedSessionKeyMessage).field("pbEncryptedSessionKeyMessage", &self.pbEncryptedSessionKeyMessage).field("cbEncryptedKeyMessage", &self.cbEncryptedKeyMessage).field("pbEncryptedKeyMessage", &self.pbEncryptedKeyMessage).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMMPEG2VIDEOINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMMPEG2VIDEOINFO {
    fn eq(&self, other: &Self) -> bool {
        self.hdr == other.hdr && self.dwStartTimeCode == other.dwStartTimeCode && self.cbSequenceHeader == other.cbSequenceHeader && self.dwProfile == other.dwProfile && self.dwLevel == other.dwLevel && self.dwFlags == other.dwFlags && self.dwSequenceHeader == other.dwSequenceHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMMPEG2VIDEOINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMMPEG2VIDEOINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMMPEG2VIDEOINFO").field("hdr", &self.hdr).field("dwStartTimeCode", &self.dwStartTimeCode).field("cbSequenceHeader", &self.cbSequenceHeader).field("dwProfile", &self.dwProfile).field("dwLevel", &self.dwLevel).field("dwFlags", &self.dwFlags).field("dwSequenceHeader", &self.dwSequenceHeader).finish()
    }
}
impl ::core::default::Default for WMSCRIPTFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMSCRIPTFORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.scriptType == other.scriptType
    }
}
impl ::core::cmp::Eq for WMSCRIPTFORMAT {}
impl ::core::fmt::Debug for WMSCRIPTFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMSCRIPTFORMAT").field("scriptType", &self.scriptType).finish()
    }
}
impl ::core::default::Default for WMT_ATTR_DATATYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_ATTR_DATATYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_DATATYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_ATTR_IMAGETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_ATTR_IMAGETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_ATTR_IMAGETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_BUFFER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_BUFFER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.pBuffer == other.pBuffer && self.cbOffset == other.cbOffset && self.cbLength == other.cbLength
    }
}
impl ::core::cmp::Eq for WMT_BUFFER_SEGMENT {}
impl ::core::fmt::Debug for WMT_BUFFER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_BUFFER_SEGMENT").field("pBuffer", &self.pBuffer).field("cbOffset", &self.cbOffset).field("cbLength", &self.cbLength).finish()
    }
}
impl ::core::default::Default for WMT_CODEC_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_CODEC_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CODEC_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ucColorPrimaries == other.ucColorPrimaries && self.ucColorTransferChar == other.ucColorTransferChar && self.ucColorMatrixCoef == other.ucColorMatrixCoef
    }
}
impl ::core::cmp::Eq for WMT_COLORSPACEINFO_EXTENSION_DATA {}
impl ::core::fmt::Debug for WMT_COLORSPACEINFO_EXTENSION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_COLORSPACEINFO_EXTENSION_DATA").field("ucColorPrimaries", &self.ucColorPrimaries).field("ucColorTransferChar", &self.ucColorTransferChar).field("ucColorMatrixCoef", &self.ucColorMatrixCoef).finish()
    }
}
impl ::core::default::Default for WMT_CREDENTIAL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_CREDENTIAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_CREDENTIAL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_DRMLA_TRUST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_DRMLA_TRUST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_DRMLA_TRUST").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_FILESINK_DATA_UNIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_FILESINK_DATA_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.packetHeaderBuffer == other.packetHeaderBuffer && self.cPayloads == other.cPayloads && self.pPayloadHeaderBuffers == other.pPayloadHeaderBuffers && self.cPayloadDataFragments == other.cPayloadDataFragments && self.pPayloadDataFragments == other.pPayloadDataFragments
    }
}
impl ::core::cmp::Eq for WMT_FILESINK_DATA_UNIT {}
impl ::core::fmt::Debug for WMT_FILESINK_DATA_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_FILESINK_DATA_UNIT").field("packetHeaderBuffer", &self.packetHeaderBuffer).field("cPayloads", &self.cPayloads).field("pPayloadHeaderBuffers", &self.pPayloadHeaderBuffers).field("cPayloadDataFragments", &self.cPayloadDataFragments).field("pPayloadDataFragments", &self.pPayloadDataFragments).finish()
    }
}
impl ::core::default::Default for WMT_FILESINK_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_FILESINK_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_FILESINK_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_IMAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_INDEXER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_INDEXER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEXER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_INDEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_INDEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_INDEX_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_MUSICSPEECH_CLASS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_MUSICSPEECH_CLASS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_MUSICSPEECH_CLASS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_NET_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_NET_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_NET_PROTOCOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_OFFSET_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_OFFSET_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_OFFSET_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_PAYLOAD_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_PAYLOAD_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.dwPayloadIndex == other.dwPayloadIndex && self.segmentData == other.segmentData
    }
}
impl ::core::cmp::Eq for WMT_PAYLOAD_FRAGMENT {}
impl ::core::fmt::Debug for WMT_PAYLOAD_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_PAYLOAD_FRAGMENT").field("dwPayloadIndex", &self.dwPayloadIndex).field("segmentData", &self.segmentData).finish()
    }
}
impl ::core::default::Default for WMT_PLAY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_PLAY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PLAY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_PROXY_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_PROXY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_PROXY_SETTINGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_STORAGE_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_STORAGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STORAGE_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_STREAM_SELECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_STREAM_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_STREAM_SELECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_TIMECODE_EXTENSION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WMT_TIMECODE_FRAMERATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_TIMECODE_FRAMERATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TIMECODE_FRAMERATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_TRANSPORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_TRANSPORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_TRANSPORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic
            && self.cbStruct == other.cbStruct
            && self.dwControlFlags == other.dwControlFlags
            && self.dwInputFlagsCur == other.dwInputFlagsCur
            && self.lCurMotionXtoX == other.lCurMotionXtoX
            && self.lCurMotionYtoX == other.lCurMotionYtoX
            && self.lCurMotionXoffset == other.lCurMotionXoffset
            && self.lCurMotionXtoY == other.lCurMotionXtoY
            && self.lCurMotionYtoY == other.lCurMotionYtoY
            && self.lCurMotionYoffset == other.lCurMotionYoffset
            && self.lCurBlendCoef1 == other.lCurBlendCoef1
            && self.lCurBlendCoef2 == other.lCurBlendCoef2
            && self.dwInputFlagsPrev == other.dwInputFlagsPrev
            && self.lPrevMotionXtoX == other.lPrevMotionXtoX
            && self.lPrevMotionYtoX == other.lPrevMotionYtoX
            && self.lPrevMotionXoffset == other.lPrevMotionXoffset
            && self.lPrevMotionXtoY == other.lPrevMotionXtoY
            && self.lPrevMotionYtoY == other.lPrevMotionYtoY
            && self.lPrevMotionYoffset == other.lPrevMotionYoffset
            && self.lPrevBlendCoef1 == other.lPrevBlendCoef1
            && self.lPrevBlendCoef2 == other.lPrevBlendCoef2
    }
}
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE {}
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE")
            .field("dwMagic", &self.dwMagic)
            .field("cbStruct", &self.cbStruct)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwInputFlagsCur", &self.dwInputFlagsCur)
            .field("lCurMotionXtoX", &self.lCurMotionXtoX)
            .field("lCurMotionYtoX", &self.lCurMotionYtoX)
            .field("lCurMotionXoffset", &self.lCurMotionXoffset)
            .field("lCurMotionXtoY", &self.lCurMotionXtoY)
            .field("lCurMotionYtoY", &self.lCurMotionYtoY)
            .field("lCurMotionYoffset", &self.lCurMotionYoffset)
            .field("lCurBlendCoef1", &self.lCurBlendCoef1)
            .field("lCurBlendCoef2", &self.lCurBlendCoef2)
            .field("dwInputFlagsPrev", &self.dwInputFlagsPrev)
            .field("lPrevMotionXtoX", &self.lPrevMotionXtoX)
            .field("lPrevMotionYtoX", &self.lPrevMotionYtoX)
            .field("lPrevMotionXoffset", &self.lPrevMotionXoffset)
            .field("lPrevMotionXtoY", &self.lPrevMotionXtoY)
            .field("lPrevMotionYtoY", &self.lPrevMotionYtoY)
            .field("lPrevMotionYoffset", &self.lPrevMotionYoffset)
            .field("lPrevBlendCoef1", &self.lPrevBlendCoef1)
            .field("lPrevBlendCoef2", &self.lPrevBlendCoef2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WMT_VIDEOIMAGE_SAMPLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WMT_VIDEOIMAGE_SAMPLE2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwMagic == other.dwMagic
            && self.dwStructSize == other.dwStructSize
            && self.dwControlFlags == other.dwControlFlags
            && self.dwViewportWidth == other.dwViewportWidth
            && self.dwViewportHeight == other.dwViewportHeight
            && self.dwCurrImageWidth == other.dwCurrImageWidth
            && self.dwCurrImageHeight == other.dwCurrImageHeight
            && self.fCurrRegionX0 == other.fCurrRegionX0
            && self.fCurrRegionY0 == other.fCurrRegionY0
            && self.fCurrRegionWidth == other.fCurrRegionWidth
            && self.fCurrRegionHeight == other.fCurrRegionHeight
            && self.fCurrBlendCoef == other.fCurrBlendCoef
            && self.dwPrevImageWidth == other.dwPrevImageWidth
            && self.dwPrevImageHeight == other.dwPrevImageHeight
            && self.fPrevRegionX0 == other.fPrevRegionX0
            && self.fPrevRegionY0 == other.fPrevRegionY0
            && self.fPrevRegionWidth == other.fPrevRegionWidth
            && self.fPrevRegionHeight == other.fPrevRegionHeight
            && self.fPrevBlendCoef == other.fPrevBlendCoef
            && self.dwEffectType == other.dwEffectType
            && self.dwNumEffectParas == other.dwNumEffectParas
            && self.fEffectPara0 == other.fEffectPara0
            && self.fEffectPara1 == other.fEffectPara1
            && self.fEffectPara2 == other.fEffectPara2
            && self.fEffectPara3 == other.fEffectPara3
            && self.fEffectPara4 == other.fEffectPara4
            && self.bKeepPrevImage == other.bKeepPrevImage
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WMT_VIDEOIMAGE_SAMPLE2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WMT_VIDEOIMAGE_SAMPLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_VIDEOIMAGE_SAMPLE2")
            .field("dwMagic", &self.dwMagic)
            .field("dwStructSize", &self.dwStructSize)
            .field("dwControlFlags", &self.dwControlFlags)
            .field("dwViewportWidth", &self.dwViewportWidth)
            .field("dwViewportHeight", &self.dwViewportHeight)
            .field("dwCurrImageWidth", &self.dwCurrImageWidth)
            .field("dwCurrImageHeight", &self.dwCurrImageHeight)
            .field("fCurrRegionX0", &self.fCurrRegionX0)
            .field("fCurrRegionY0", &self.fCurrRegionY0)
            .field("fCurrRegionWidth", &self.fCurrRegionWidth)
            .field("fCurrRegionHeight", &self.fCurrRegionHeight)
            .field("fCurrBlendCoef", &self.fCurrBlendCoef)
            .field("dwPrevImageWidth", &self.dwPrevImageWidth)
            .field("dwPrevImageHeight", &self.dwPrevImageHeight)
            .field("fPrevRegionX0", &self.fPrevRegionX0)
            .field("fPrevRegionY0", &self.fPrevRegionY0)
            .field("fPrevRegionWidth", &self.fPrevRegionWidth)
            .field("fPrevRegionHeight", &self.fPrevRegionHeight)
            .field("fPrevBlendCoef", &self.fPrevBlendCoef)
            .field("dwEffectType", &self.dwEffectType)
            .field("dwNumEffectParas", &self.dwNumEffectParas)
            .field("fEffectPara0", &self.fEffectPara0)
            .field("fEffectPara1", &self.fEffectPara1)
            .field("fEffectPara2", &self.fEffectPara2)
            .field("fEffectPara3", &self.fEffectPara3)
            .field("fEffectPara4", &self.fEffectPara4)
            .field("bKeepPrevImage", &self.bKeepPrevImage)
            .finish()
    }
}
impl ::core::default::Default for WMT_WATERMARK_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_WATERMARK_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.wmetType == other.wmetType && self.clsid == other.clsid && self.cbDisplayName == other.cbDisplayName && self.pwszDisplayName == other.pwszDisplayName
    }
}
impl ::core::cmp::Eq for WMT_WATERMARK_ENTRY {}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WATERMARK_ENTRY").field("wmetType", &self.wmetType).field("clsid", &self.clsid).field("cbDisplayName", &self.cbDisplayName).field("pwszDisplayName", &self.pwszDisplayName).finish()
    }
}
impl ::core::default::Default for WMT_WATERMARK_ENTRY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMT_WATERMARK_ENTRY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMT_WATERMARK_ENTRY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMT_WEBSTREAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.cbSampleHeaderFixedData == other.cbSampleHeaderFixedData && self.wVersion == other.wVersion && self.wReserved == other.wReserved
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_FORMAT {}
impl ::core::fmt::Debug for WMT_WEBSTREAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_FORMAT").field("cbSize", &self.cbSize).field("cbSampleHeaderFixedData", &self.cbSampleHeaderFixedData).field("wVersion", &self.wVersion).field("wReserved", &self.wReserved).finish()
    }
}
impl ::core::default::Default for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.cbLength == other.cbLength && self.wPart == other.wPart && self.cTotalParts == other.cTotalParts && self.wSampleType == other.wSampleType && self.wszURL == other.wszURL
    }
}
impl ::core::cmp::Eq for WMT_WEBSTREAM_SAMPLE_HEADER {}
impl ::core::fmt::Debug for WMT_WEBSTREAM_SAMPLE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMT_WEBSTREAM_SAMPLE_HEADER").field("cbLength", &self.cbLength).field("wPart", &self.wPart).field("cTotalParts", &self.cTotalParts).field("wSampleType", &self.wSampleType).field("wszURL", &self.wszURL).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER").field("rcSource", &self.rcSource).field("rcTarget", &self.rcTarget).field("dwBitRate", &self.dwBitRate).field("dwBitErrorRate", &self.dwBitErrorRate).field("AvgTimePerFrame", &self.AvgTimePerFrame).field("bmiHeader", &self.bmiHeader).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for WMVIDEOINFOHEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for WMVIDEOINFOHEADER2 {
    fn eq(&self, other: &Self) -> bool {
        self.rcSource == other.rcSource && self.rcTarget == other.rcTarget && self.dwBitRate == other.dwBitRate && self.dwBitErrorRate == other.dwBitErrorRate && self.AvgTimePerFrame == other.AvgTimePerFrame && self.dwInterlaceFlags == other.dwInterlaceFlags && self.dwCopyProtectFlags == other.dwCopyProtectFlags && self.dwPictAspectRatioX == other.dwPictAspectRatioX && self.dwPictAspectRatioY == other.dwPictAspectRatioY && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.bmiHeader == other.bmiHeader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for WMVIDEOINFOHEADER2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for WMVIDEOINFOHEADER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WMVIDEOINFOHEADER2")
            .field("rcSource", &self.rcSource)
            .field("rcTarget", &self.rcTarget)
            .field("dwBitRate", &self.dwBitRate)
            .field("dwBitErrorRate", &self.dwBitErrorRate)
            .field("AvgTimePerFrame", &self.AvgTimePerFrame)
            .field("dwInterlaceFlags", &self.dwInterlaceFlags)
            .field("dwCopyProtectFlags", &self.dwCopyProtectFlags)
            .field("dwPictAspectRatioX", &self.dwPictAspectRatioX)
            .field("dwPictAspectRatioY", &self.dwPictAspectRatioY)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("bmiHeader", &self.bmiHeader)
            .finish()
    }
}
impl ::core::default::Default for WM_ADDRESS_ACCESSENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_ADDRESS_ACCESSENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPAddress == other.dwIPAddress && self.dwMask == other.dwMask
    }
}
impl ::core::cmp::Eq for WM_ADDRESS_ACCESSENTRY {}
impl ::core::fmt::Debug for WM_ADDRESS_ACCESSENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_ADDRESS_ACCESSENTRY").field("dwIPAddress", &self.dwIPAddress).field("dwMask", &self.dwMask).finish()
    }
}
impl ::core::default::Default for WM_AETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WM_AETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_AETYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WM_CLIENT_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPAddress == other.dwIPAddress && self.dwPort == other.dwPort
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES {}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES").field("dwIPAddress", &self.dwIPAddress).field("dwPort", &self.dwPort).finish()
    }
}
impl ::core::default::Default for WM_CLIENT_PROPERTIES_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_CLIENT_PROPERTIES_EX {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.pwszIPAddress == other.pwszIPAddress && self.pwszPort == other.pwszPort && self.pwszDNSName == other.pwszDNSName
    }
}
impl ::core::cmp::Eq for WM_CLIENT_PROPERTIES_EX {}
impl ::core::fmt::Debug for WM_CLIENT_PROPERTIES_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_CLIENT_PROPERTIES_EX").field("cbSize", &self.cbSize).field("pwszIPAddress", &self.pwszIPAddress).field("pwszPort", &self.pwszPort).field("pwszDNSName", &self.pwszDNSName).finish()
    }
}
impl ::core::default::Default for WM_DM_INTERLACED_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WM_DM_INTERLACED_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_INTERLACED_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WM_DM_IT_FIRST_FRAME_COHERENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_DM_IT_FIRST_FRAME_COHERENCY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WM_LEAKY_BUCKET_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_MEDIA_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.majortype == other.majortype && self.subtype == other.subtype && self.bFixedSizeSamples == other.bFixedSizeSamples && self.bTemporalCompression == other.bTemporalCompression && self.lSampleSize == other.lSampleSize && self.formattype == other.formattype && self.pUnk == other.pUnk && self.cbFormat == other.cbFormat && self.pbFormat == other.pbFormat
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_MEDIA_TYPE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_MEDIA_TYPE").field("majortype", &self.majortype).field("subtype", &self.subtype).field("bFixedSizeSamples", &self.bFixedSizeSamples).field("bTemporalCompression", &self.bTemporalCompression).field("lSampleSize", &self.lSampleSize).field("formattype", &self.formattype).field("pUnk", &self.pUnk).field("cbFormat", &self.cbFormat).field("pbFormat", &self.pbFormat).finish()
    }
}
impl ::core::default::Default for WM_PICTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WM_PLAYBACK_DRC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WM_PLAYBACK_DRC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_PLAYBACK_DRC_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for WM_PORT_NUMBER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_PORT_NUMBER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wPortBegin == other.wPortBegin && self.wPortEnd == other.wPortEnd
    }
}
impl ::core::cmp::Eq for WM_PORT_NUMBER_RANGE {}
impl ::core::fmt::Debug for WM_PORT_NUMBER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_PORT_NUMBER_RANGE").field("wPortBegin", &self.wPortBegin).field("wPortEnd", &self.wPortEnd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_READER_CLIENTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WM_READER_CLIENTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.wszLang == other.wszLang && self.wszBrowserUserAgent == other.wszBrowserUserAgent && self.wszBrowserWebPage == other.wszBrowserWebPage && self.qwReserved == other.qwReserved && self.pReserved == other.pReserved && self.wszHostExe == other.wszHostExe && self.qwHostVersion == other.qwHostVersion && self.wszPlayerUserAgent == other.wszPlayerUserAgent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WM_READER_CLIENTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WM_READER_CLIENTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_CLIENTINFO").field("cbSize", &self.cbSize).field("wszLang", &self.wszLang).field("wszBrowserUserAgent", &self.wszBrowserUserAgent).field("wszBrowserWebPage", &self.wszBrowserWebPage).field("qwReserved", &self.qwReserved).field("pReserved", &self.pReserved).field("wszHostExe", &self.wszHostExe).field("qwHostVersion", &self.qwHostVersion).field("wszPlayerUserAgent", &self.wszPlayerUserAgent).finish()
    }
}
impl ::core::default::Default for WM_READER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_READER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwBandwidth == other.dwBandwidth && self.cPacketsReceived == other.cPacketsReceived && self.cPacketsRecovered == other.cPacketsRecovered && self.cPacketsLost == other.cPacketsLost && self.wQuality == other.wQuality
    }
}
impl ::core::cmp::Eq for WM_READER_STATISTICS {}
impl ::core::fmt::Debug for WM_READER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_READER_STATISTICS").field("cbSize", &self.cbSize).field("dwBandwidth", &self.dwBandwidth).field("cPacketsReceived", &self.cPacketsReceived).field("cPacketsRecovered", &self.cPacketsRecovered).field("cPacketsLost", &self.cPacketsLost).field("wQuality", &self.wQuality).finish()
    }
}
impl ::core::default::Default for WM_SFEX_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WM_SFEX_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SFEX_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WM_SF_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WM_SF_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WM_SF_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WM_STREAM_PRIORITY_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WM_STREAM_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WM_SYNCHRONISED_LYRICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WM_USER_TEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WM_USER_WEB_URL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WM_WRITER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.qwSampleCount == other.qwSampleCount && self.qwByteCount == other.qwByteCount && self.qwDroppedSampleCount == other.qwDroppedSampleCount && self.qwDroppedByteCount == other.qwDroppedByteCount && self.dwCurrentBitrate == other.dwCurrentBitrate && self.dwAverageBitrate == other.dwAverageBitrate && self.dwExpectedBitrate == other.dwExpectedBitrate && self.dwCurrentSampleRate == other.dwCurrentSampleRate && self.dwAverageSampleRate == other.dwAverageSampleRate && self.dwExpectedSampleRate == other.dwExpectedSampleRate
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS {}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS")
            .field("qwSampleCount", &self.qwSampleCount)
            .field("qwByteCount", &self.qwByteCount)
            .field("qwDroppedSampleCount", &self.qwDroppedSampleCount)
            .field("qwDroppedByteCount", &self.qwDroppedByteCount)
            .field("dwCurrentBitrate", &self.dwCurrentBitrate)
            .field("dwAverageBitrate", &self.dwAverageBitrate)
            .field("dwExpectedBitrate", &self.dwExpectedBitrate)
            .field("dwCurrentSampleRate", &self.dwCurrentSampleRate)
            .field("dwAverageSampleRate", &self.dwAverageSampleRate)
            .field("dwExpectedSampleRate", &self.dwExpectedSampleRate)
            .finish()
    }
}
impl ::core::default::Default for WM_WRITER_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WM_WRITER_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.dwBitratePlusOverhead == other.dwBitratePlusOverhead && self.dwCurrentSampleDropRateInQueue == other.dwCurrentSampleDropRateInQueue && self.dwCurrentSampleDropRateInCodec == other.dwCurrentSampleDropRateInCodec && self.dwCurrentSampleDropRateInMultiplexer == other.dwCurrentSampleDropRateInMultiplexer && self.dwTotalSampleDropsInQueue == other.dwTotalSampleDropsInQueue && self.dwTotalSampleDropsInCodec == other.dwTotalSampleDropsInCodec && self.dwTotalSampleDropsInMultiplexer == other.dwTotalSampleDropsInMultiplexer
    }
}
impl ::core::cmp::Eq for WM_WRITER_STATISTICS_EX {}
impl ::core::fmt::Debug for WM_WRITER_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WM_WRITER_STATISTICS_EX")
            .field("dwBitratePlusOverhead", &self.dwBitratePlusOverhead)
            .field("dwCurrentSampleDropRateInQueue", &self.dwCurrentSampleDropRateInQueue)
            .field("dwCurrentSampleDropRateInCodec", &self.dwCurrentSampleDropRateInCodec)
            .field("dwCurrentSampleDropRateInMultiplexer", &self.dwCurrentSampleDropRateInMultiplexer)
            .field("dwTotalSampleDropsInQueue", &self.dwTotalSampleDropsInQueue)
            .field("dwTotalSampleDropsInCodec", &self.dwTotalSampleDropsInCodec)
            .field("dwTotalSampleDropsInMultiplexer", &self.dwTotalSampleDropsInMultiplexer)
            .finish()
    }
}
impl ::core::default::Default for _AM_ASFWRITERCONFIG_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for _AM_ASFWRITERCONFIG_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AM_ASFWRITERCONFIG_PARAM").field(&self.0).finish()
    }
}
