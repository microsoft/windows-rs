impl ::core::default::Default for DELETE_OBJECT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DELETE_OBJECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DELETE_OBJECT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICE_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICE_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_RADIO_STATE").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IConnectionRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConnectionRequestCallback {}
impl ::core::fmt::Debug for IConnectionRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConnectionRequestCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumPortableDeviceConnectors {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPortableDeviceConnectors {}
impl ::core::fmt::Debug for IEnumPortableDeviceConnectors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPortableDeviceConnectors").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumPortableDeviceObjectIDs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumPortableDeviceObjectIDs {}
impl ::core::fmt::Debug for IEnumPortableDeviceObjectIDs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumPortableDeviceObjectIDs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMediaRadioManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaRadioManager {}
impl ::core::fmt::Debug for IMediaRadioManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaRadioManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMediaRadioManagerNotifySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMediaRadioManagerNotifySink {}
impl ::core::fmt::Debug for IMediaRadioManagerNotifySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMediaRadioManagerNotifySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevice {}
impl ::core::fmt::Debug for IPortableDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceCapabilities {}
impl ::core::fmt::Debug for IPortableDeviceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceCapabilities").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceConnector {}
impl ::core::fmt::Debug for IPortableDeviceConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceConnector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceContent {}
impl ::core::fmt::Debug for IPortableDeviceContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceContent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceContent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceContent2 {}
impl ::core::fmt::Debug for IPortableDeviceContent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceContent2").field(&self.0).finish()
    }
}
impl IPortableDeviceContent2 {
    pub unsafe fn EnumObjects<P0, P1>(&self, dwflags: u32, pszparentobjectid: P0, pfilter: P1) -> ::windows::core::Result<IEnumPortableDeviceObjectIDs>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IPortableDeviceValues>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumObjects)(::windows::core::Vtable::as_raw(self), dwflags, pszparentobjectid.into().abi(), pfilter.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Properties(&self) -> ::windows::core::Result<IPortableDeviceProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Properties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Transfer(&self) -> ::windows::core::Result<IPortableDeviceResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Transfer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateObjectWithPropertiesOnly<P0>(&self, pvalues: P0, ppszobjectid: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPortableDeviceValues>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateObjectWithPropertiesOnly)(::windows::core::Vtable::as_raw(self), pvalues.into().abi(), ppszobjectid).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateObjectWithPropertiesAndData<P0>(&self, pvalues: P0, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPortableDeviceValues>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateObjectWithPropertiesAndData)(::windows::core::Vtable::as_raw(self), pvalues.into().abi(), ::core::mem::transmute(ppdata), pdwoptimalwritebuffersize, ppszcookie).ok()
    }
    pub unsafe fn Delete<P0>(&self, dwoptions: u32, pobjectids: P0, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPortableDevicePropVariantCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self), dwoptions, pobjectids.into().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn GetObjectIDsFromPersistentUniqueIDs<P0>(&self, ppersistentuniqueids: P0) -> ::windows::core::Result<IPortableDevicePropVariantCollection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPortableDevicePropVariantCollection>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetObjectIDsFromPersistentUniqueIDs)(::windows::core::Vtable::as_raw(self), ppersistentuniqueids.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Move<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPortableDevicePropVariantCollection>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Move)(::windows::core::Vtable::as_raw(self), pobjectids.into().abi(), pszdestinationfolderobjectid.into().abi(), ::core::mem::transmute(ppresults)).ok()
    }
    pub unsafe fn Copy<P0, P1>(&self, pobjectids: P0, pszdestinationfolderobjectid: P1, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IPortableDevicePropVariantCollection>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Copy)(::windows::core::Vtable::as_raw(self), pobjectids.into().abi(), pszdestinationfolderobjectid.into().abi(), ::core::mem::transmute(ppresults)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPortableDeviceDataStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPortableDeviceDataStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPortableDeviceDataStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDataStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IPortableDeviceDataStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSize)(::windows::core::Vtable::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Revert)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stat)(::windows::core::Vtable::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceDispatchFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceDispatchFactory {}
impl ::core::fmt::Debug for IPortableDeviceDispatchFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceDispatchFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceEventCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceEventCallback {}
impl ::core::fmt::Debug for IPortableDeviceEventCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceEventCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceKeyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceKeyCollection {}
impl ::core::fmt::Debug for IPortableDeviceKeyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceKeyCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceManager {}
impl ::core::fmt::Debug for IPortableDeviceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDevicePropVariantCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropVariantCollection {}
impl ::core::fmt::Debug for IPortableDevicePropVariantCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropVariantCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceProperties {}
impl ::core::fmt::Debug for IPortableDeviceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDevicePropertiesBulk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropertiesBulk {}
impl ::core::fmt::Debug for IPortableDevicePropertiesBulk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropertiesBulk").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDevicePropertiesBulkCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDevicePropertiesBulkCallback {}
impl ::core::fmt::Debug for IPortableDevicePropertiesBulkCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDevicePropertiesBulkCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceResources {}
impl ::core::fmt::Debug for IPortableDeviceResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceResources").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceService {}
impl ::core::fmt::Debug for IPortableDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceActivation {}
impl ::core::fmt::Debug for IPortableDeviceServiceActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceActivation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceCapabilities {}
impl ::core::fmt::Debug for IPortableDeviceServiceCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceCapabilities").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceManager {}
impl ::core::fmt::Debug for IPortableDeviceServiceManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceMethodCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceMethodCallback {}
impl ::core::fmt::Debug for IPortableDeviceServiceMethodCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceMethodCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceMethods {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceMethods {}
impl ::core::fmt::Debug for IPortableDeviceServiceMethods {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceMethods").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceServiceOpenCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceServiceOpenCallback {}
impl ::core::fmt::Debug for IPortableDeviceServiceOpenCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceServiceOpenCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceUnitsStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceUnitsStream {}
impl ::core::fmt::Debug for IPortableDeviceUnitsStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceUnitsStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceValues {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceValues {}
impl ::core::fmt::Debug for IPortableDeviceValues {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceValues").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPortableDeviceValuesCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPortableDeviceValuesCollection {}
impl ::core::fmt::Debug for IPortableDeviceValuesCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceValuesCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPortableDeviceWebControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPortableDeviceWebControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPortableDeviceWebControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPortableDeviceWebControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRadioInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadioInstance {}
impl ::core::fmt::Debug for IRadioInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadioInstance").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRadioInstanceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadioInstanceCollection {}
impl ::core::fmt::Debug for IRadioInstanceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadioInstanceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWpdSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWpdSerializer {}
impl ::core::fmt::Debug for IWpdSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWpdSerializer").field(&self.0).finish()
    }
}
impl ::core::default::Default for SMS_MESSAGE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SMS_MESSAGE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SMS_MESSAGE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SYSTEM_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SYSTEM_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYSTEM_RADIO_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_BITRATE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_BITRATE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_BITRATE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_CAPTURE_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_CAPTURE_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_CAPTURE_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_COLOR_CORRECTED_STATUS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_COLOR_CORRECTED_STATUS_VALUES").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::default::Default for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Command == other.Command && self.AccessType == other.AccessType && self.AccessProperty == other.AccessProperty
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_LOOKUP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WPD_COMMAND_ACCESS_LOOKUP_ENTRY").field("Command", &self.Command).field("AccessType", &self.AccessType).field("AccessProperty", &self.AccessProperty).finish()
    }
}
impl ::core::default::Default for WPD_COMMAND_ACCESS_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_COMMAND_ACCESS_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_COMMAND_ACCESS_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_CROPPED_STATUS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_CROPPED_STATUS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_CROPPED_STATUS_VALUES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_DEVICE_TRANSPORTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_DEVICE_TRANSPORTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_DEVICE_TRANSPORTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_DEVICE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_DEVICE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_DEVICE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_EFFECT_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_EFFECT_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EFFECT_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_EXPOSURE_METERING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_EXPOSURE_METERING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EXPOSURE_METERING_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_EXPOSURE_PROGRAM_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_EXPOSURE_PROGRAM_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_EXPOSURE_PROGRAM_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_FLASH_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_FLASH_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FLASH_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_FOCUS_METERING_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_FOCUS_METERING_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FOCUS_METERING_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_FOCUS_MODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_FOCUS_MODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_FOCUS_MODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_META_GENRES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_META_GENRES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_META_GENRES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_OPERATION_STATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_OPERATION_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_OPERATION_STATES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_PARAMETER_USAGE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_PARAMETER_USAGE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_PARAMETER_USAGE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_POWER_SOURCES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_POWER_SOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_POWER_SOURCES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_RENDERING_INFORMATION_PROFILE_ENTRY_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_SECTION_DATA_UNITS_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_SECTION_DATA_UNITS_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SECTION_DATA_UNITS_VALUES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_SERVICE_INHERITANCE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_SERVICE_INHERITANCE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SERVICE_INHERITANCE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_SMS_ENCODING_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_SMS_ENCODING_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_SMS_ENCODING_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_STORAGE_ACCESS_CAPABILITY_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STORAGE_ACCESS_CAPABILITY_VALUES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_STORAGE_TYPE_VALUES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_STORAGE_TYPE_VALUES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STORAGE_TYPE_VALUES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_STREAM_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_STREAM_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_STREAM_UNITS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_VIDEO_SCAN_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_VIDEO_SCAN_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_VIDEO_SCAN_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WPD_WHITE_BALANCE_SETTINGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WPD_WHITE_BALANCE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WPD_WHITE_BALANCE_SETTINGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WpdAttributeForm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WpdAttributeForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WpdAttributeForm").field(&self.0).finish()
    }
}
impl ::core::default::Default for WpdParameterAttributeForm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WpdParameterAttributeForm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WpdParameterAttributeForm").field(&self.0).finish()
    }
}
