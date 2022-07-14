#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[inline]
pub unsafe fn CreateVssExpressWriterInternal() -> ::windows::core::Result<IVssExpressWriter> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateVssExpressWriterInternal(ppwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateVssExpressWriterInternal(::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssExpressWriter>(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssAdmin(::windows::core::IUnknown);
impl IVssAdmin {
    pub unsafe fn RegisterProvider(&self, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproviderid), ::core::mem::transmute(classid), ::core::mem::transmute(pwszprovidername), eprovidertype, ::core::mem::transmute(pwszproviderversion), ::core::mem::transmute(providerversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryProviders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumObject>(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbortAllSnapshotsInProgress)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IVssAdmin> for ::windows::core::IUnknown {
    fn from(value: IVssAdmin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssAdmin> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssAdmin) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssAdmin> for ::windows::core::IUnknown {
    fn from(value: &IVssAdmin) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssAdmin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssAdmin {}
impl ::core::fmt::Debug for IVssAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssAdmin").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssAdmin {
    type Vtable = IVssAdmin_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x77ed5996_2f63_11d3_8a39_00c04f72d8e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdmin_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RegisterProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AbortAllSnapshotsInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssAdminEx(::windows::core::IUnknown);
impl IVssAdminEx {
    pub unsafe fn RegisterProvider(&self, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RegisterProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproviderid), ::core::mem::transmute(classid), ::core::mem::transmute(pwszprovidername), eprovidertype, ::core::mem::transmute(pwszproviderversion), ::core::mem::transmute(providerversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnregisterProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryProviders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumObject>(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AbortAllSnapshotsInProgress)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProviderCapability(&self, pproviderid: ::windows::core::GUID) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderCapability)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproviderid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn GetProviderContext(&self, providerid: ::windows::core::GUID) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i32>(result__)
    }
    pub unsafe fn SetProviderContext(&self, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProviderContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid), lcontext).ok()
    }
}
impl ::core::convert::From<IVssAdminEx> for ::windows::core::IUnknown {
    fn from(value: IVssAdminEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssAdminEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssAdminEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssAdminEx> for ::windows::core::IUnknown {
    fn from(value: &IVssAdminEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssAdminEx> for IVssAdmin {
    fn from(value: IVssAdminEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssAdminEx> for &'a IVssAdmin {
    fn from(value: &'a IVssAdminEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssAdminEx> for IVssAdmin {
    fn from(value: &IVssAdminEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssAdminEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssAdminEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssAdminEx {}
impl ::core::fmt::Debug for IVssAdminEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssAdminEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssAdminEx {
    type Vtable = IVssAdminEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7858a9f8_b1fa_41a6_964f_b9b36b8cd8d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdminEx_Vtbl {
    pub base__: IVssAdmin_Vtbl,
    pub GetProviderCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderid: ::windows::core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT,
    pub GetProviderContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, plcontext: *mut i32) -> ::windows::core::HRESULT,
    pub SetProviderContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssAsync(::windows::core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Wait)(::windows::core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phrresult), ::core::mem::transmute(preserved)).ok()
    }
}
impl ::core::convert::From<IVssAsync> for ::windows::core::IUnknown {
    fn from(value: IVssAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssAsync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssAsync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssAsync> for ::windows::core::IUnknown {
    fn from(value: &IVssAsync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssAsync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssAsync {}
impl ::core::fmt::Debug for IVssAsync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssAsync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssAsync {
    type Vtable = IVssAsync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507c37b4_cf5b_4e95_b0af_14eb9767467e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAsync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows::core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, preserved: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssComponent(::windows::core::IUnknown);
impl IVssComponent {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogicalPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComponentType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pct)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComponentName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBackupSucceeded)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsucceeded)).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAlternateLocationMappingCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcmappings)).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAlternateLocationMapping)(::windows::core::Interface::as_raw(self), imapping, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssWMFiledesc>(result__)
    }
    pub unsafe fn SetBackupMetadata<'a, P0>(&self, wszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBackupMetadata)(::windows::core::Interface::as_raw(self), wszdata.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBackupMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<'a, P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPartialFile)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilename.into(), wszranges.into(), wszmetadata.into()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPartialFileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcpartialfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPartialFile)(::windows::core::Interface::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsSelectedForRestore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbselectedforrestore)).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAdditionalRestores)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbadditionalrestores)).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNewTargetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcnewtarget)).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetNewTarget)(::windows::core::Interface::as_raw(self), inewtarget, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssWMFiledesc>(result__)
    }
    pub unsafe fn AddDirectedTarget<'a, P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::PCWSTR>,
        P5: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDirectedTarget)(::windows::core::Interface::as_raw(self), wszsourcepath.into(), wszsourcefilename.into(), wszsourcerangelist.into(), wszdestinationpath.into(), wszdestinationfilename.into(), wszdestinationrangelist.into()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDirectedTargetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdirectedtarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDirectedTarget)(::windows::core::Interface::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<'a, P0>(&self, wszrestoremetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRestoreMetadata)(::windows::core::Interface::as_raw(self), wszrestoremetadata.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRestoreMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRestoreTarget)(::windows::core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRestoreTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptarget)).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<'a, P0>(&self, wszprerestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPreRestoreFailureMsg)(::windows::core::Interface::as_raw(self), wszprerestorefailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPreRestoreFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<'a, P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPostRestoreFailureMsg)(::windows::core::Interface::as_raw(self), wszpostrestorefailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPostRestoreFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<'a, P0>(&self, wszbackupstamp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBackupStamp)(::windows::core::Interface::as_raw(self), wszbackupstamp.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBackupStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPreviousBackupStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBackupOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRestoreOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRestoreSubcomponentCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcrestoresubcomponent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRestoreSubcomponent)(::windows::core::Interface::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), ::core::mem::transmute(pbrepair)).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileRestoreStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<'a, P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).AddDifferencedFilesByLastModifyTime)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive.into(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<'a, P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).AddDifferencedFilesByLastModifyLSN)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive.into(), bstrlsnstring.into().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDifferencedFilesCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdifferencedfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDifferencedFile)(::windows::core::Interface::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), ::core::mem::transmute(pbrecursive), ::core::mem::transmute(pbstrlsnstring), ::core::mem::transmute(pftlastmodifytime)).ok()
    }
}
impl ::core::convert::From<IVssComponent> for ::windows::core::IUnknown {
    fn from(value: IVssComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssComponent> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssComponent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponent> for ::windows::core::IUnknown {
    fn from(value: &IVssComponent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssComponent {}
impl ::core::fmt::Debug for IVssComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssComponent {
    type Vtable = IVssComponent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c72c96_c121_4518_b627_e5a93d010ead);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponent_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogicalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogicalPath: usize,
    pub GetComponentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentName: usize,
    pub GetBackupSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows::core::HRESULT,
    pub GetAlternateLocationMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows::core::HRESULT,
    pub GetAlternateLocationMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBackupMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackupMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackupMetadata: usize,
    pub AddPartialFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, wszfilename: ::windows::core::PCWSTR, wszranges: ::windows::core::PCWSTR, wszmetadata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetPartialFileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPartialFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPartialFile: usize,
    pub IsSelectedForRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows::core::HRESULT,
    pub GetAdditionalRestores: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows::core::HRESULT,
    pub GetNewTargetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows::core::HRESULT,
    pub GetNewTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddDirectedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszsourcepath: ::windows::core::PCWSTR, wszsourcefilename: ::windows::core::PCWSTR, wszsourcerangelist: ::windows::core::PCWSTR, wszdestinationpath: ::windows::core::PCWSTR, wszdestinationfilename: ::windows::core::PCWSTR, wszdestinationrangelist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDirectedTargetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDirectedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDirectedTarget: usize,
    pub SetRestoreMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszrestoremetadata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreMetadata: usize,
    pub SetRestoreTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    pub GetRestoreTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::HRESULT,
    pub SetPreRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreRestoreFailureMsg: usize,
    pub SetPostRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPostRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPostRestoreFailureMsg: usize,
    pub SetBackupStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszbackupstamp: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackupStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackupStamp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPreviousBackupStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPreviousBackupStamp: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBackupOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBackupOptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreOptions: usize,
    pub GetRestoreSubcomponentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreSubcomponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreSubcomponent: usize,
    pub GetFileRestoreStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDifferencedFilesByLastModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDifferencedFilesByLastModifyTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDifferencedFilesByLastModifyLSN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDifferencedFilesByLastModifyLSN: usize,
    pub GetDifferencedFilesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDifferencedFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDifferencedFile: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssComponentEx(::windows::core::IUnknown);
impl IVssComponentEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLogicalPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetComponentType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pct)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetComponentName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBackupSucceeded)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsucceeded)).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAlternateLocationMappingCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcmappings)).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAlternateLocationMapping)(::windows::core::Interface::as_raw(self), imapping, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssWMFiledesc>(result__)
    }
    pub unsafe fn SetBackupMetadata<'a, P0>(&self, wszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBackupMetadata)(::windows::core::Interface::as_raw(self), wszdata.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBackupMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<'a, P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddPartialFile)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilename.into(), wszranges.into(), wszmetadata.into()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPartialFileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcpartialfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPartialFile)(::windows::core::Interface::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsSelectedForRestore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbselectedforrestore)).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAdditionalRestores)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbadditionalrestores)).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNewTargetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcnewtarget)).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetNewTarget)(::windows::core::Interface::as_raw(self), inewtarget, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssWMFiledesc>(result__)
    }
    pub unsafe fn AddDirectedTarget<'a, P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::PCWSTR>,
        P5: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDirectedTarget)(::windows::core::Interface::as_raw(self), wszsourcepath.into(), wszsourcefilename.into(), wszsourcerangelist.into(), wszdestinationpath.into(), wszdestinationfilename.into(), wszdestinationrangelist.into()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDirectedTargetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdirectedtarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDirectedTarget)(::windows::core::Interface::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<'a, P0>(&self, wszrestoremetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetRestoreMetadata)(::windows::core::Interface::as_raw(self), wszrestoremetadata.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRestoreMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRestoreTarget)(::windows::core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRestoreTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptarget)).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<'a, P0>(&self, wszprerestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPreRestoreFailureMsg)(::windows::core::Interface::as_raw(self), wszprerestorefailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPreRestoreFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<'a, P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPostRestoreFailureMsg)(::windows::core::Interface::as_raw(self), wszpostrestorefailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPostRestoreFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<'a, P0>(&self, wszbackupstamp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetBackupStamp)(::windows::core::Interface::as_raw(self), wszbackupstamp.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBackupStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPreviousBackupStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetBackupOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRestoreOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRestoreSubcomponentCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcrestoresubcomponent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRestoreSubcomponent)(::windows::core::Interface::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), ::core::mem::transmute(pbrepair)).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFileRestoreStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<'a, P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDifferencedFilesByLastModifyTime)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive.into(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<'a, P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.AddDifferencedFilesByLastModifyLSN)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive.into(), bstrlsnstring.into().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDifferencedFilesCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdifferencedfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDifferencedFile)(::windows::core::Interface::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), ::core::mem::transmute(pbrecursive), ::core::mem::transmute(pbstrlsnstring), ::core::mem::transmute(pftlastmodifytime)).ok()
    }
    pub unsafe fn SetPrepareForBackupFailureMsg<'a, P0>(&self, wszfailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPrepareForBackupFailureMsg)(::windows::core::Interface::as_raw(self), wszfailuremsg.into()).ok()
    }
    pub unsafe fn SetPostSnapshotFailureMsg<'a, P0>(&self, wszfailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetPostSnapshotFailureMsg)(::windows::core::Interface::as_raw(self), wszfailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPrepareForBackupFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPostSnapshotFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::core::Result<bool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAuthoritativeRestore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<bool>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRollForward)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prolltype), ::core::mem::transmute(pbstrpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRestoreName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IVssComponentEx> for ::windows::core::IUnknown {
    fn from(value: IVssComponentEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssComponentEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssComponentEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx> for ::windows::core::IUnknown {
    fn from(value: &IVssComponentEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssComponentEx> for IVssComponent {
    fn from(value: IVssComponentEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssComponentEx> for &'a IVssComponent {
    fn from(value: &'a IVssComponentEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx> for IVssComponent {
    fn from(value: &IVssComponentEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssComponentEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssComponentEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssComponentEx {}
impl ::core::fmt::Debug for IVssComponentEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssComponentEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssComponentEx {
    type Vtable = IVssComponentEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x156c8b5e_f131_4bd7_9c97_d1923be7e1fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx_Vtbl {
    pub base__: IVssComponent_Vtbl,
    pub SetPrepareForBackupFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfailuremsg: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetPostSnapshotFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfailuremsg: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPrepareForBackupFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPrepareForBackupFailureMsg: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPostSnapshotFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPostSnapshotFailureMsg: usize,
    pub GetAuthoritativeRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRollForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRollForward: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetRestoreName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetRestoreName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssComponentEx2(::windows::core::IUnknown);
impl IVssComponentEx2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetLogicalPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pct)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBackupSucceeded)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsucceeded)).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAlternateLocationMappingCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcmappings)).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetAlternateLocationMapping)(::windows::core::Interface::as_raw(self), imapping, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssWMFiledesc>(result__)
    }
    pub unsafe fn SetBackupMetadata<'a, P0>(&self, wszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetBackupMetadata)(::windows::core::Interface::as_raw(self), wszdata.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBackupMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<'a, P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddPartialFile)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilename.into(), wszranges.into(), wszmetadata.into()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPartialFileCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcpartialfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilename: *mut super::super::Foundation::BSTR, pbstrrange: *mut super::super::Foundation::BSTR, pbstrmetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPartialFile)(::windows::core::Interface::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.IsSelectedForRestore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbselectedforrestore)).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAdditionalRestores)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbadditionalrestores)).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetNewTargetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcnewtarget)).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetNewTarget)(::windows::core::Interface::as_raw(self), inewtarget, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssWMFiledesc>(result__)
    }
    pub unsafe fn AddDirectedTarget<'a, P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::PCWSTR>,
        P5: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDirectedTarget)(::windows::core::Interface::as_raw(self), wszsourcepath.into(), wszsourcefilename.into(), wszsourcerangelist.into(), wszdestinationpath.into(), wszdestinationfilename.into(), wszdestinationrangelist.into()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDirectedTargetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdirectedtarget)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut super::super::Foundation::BSTR, pbstrsourcefilename: *mut super::super::Foundation::BSTR, pbstrsourcerangelist: *mut super::super::Foundation::BSTR, pbstrdestinationpath: *mut super::super::Foundation::BSTR, pbstrdestinationfilename: *mut super::super::Foundation::BSTR, pbstrdestinationrangelist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDirectedTarget)(::windows::core::Interface::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<'a, P0>(&self, wszrestoremetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetRestoreMetadata)(::windows::core::Interface::as_raw(self), wszrestoremetadata.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRestoreMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetRestoreTarget)(::windows::core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRestoreTarget)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ptarget)).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<'a, P0>(&self, wszprerestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPreRestoreFailureMsg)(::windows::core::Interface::as_raw(self), wszprerestorefailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPreRestoreFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<'a, P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetPostRestoreFailureMsg)(::windows::core::Interface::as_raw(self), wszpostrestorefailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPostRestoreFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<'a, P0>(&self, wszbackupstamp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetBackupStamp)(::windows::core::Interface::as_raw(self), wszbackupstamp.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBackupStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPreviousBackupStamp)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetBackupOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRestoreOptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRestoreSubcomponentCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcrestoresubcomponent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut super::super::Foundation::BSTR, pbstrcomponentname: *mut super::super::Foundation::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRestoreSubcomponent)(::windows::core::Interface::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), ::core::mem::transmute(pbrepair)).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFileRestoreStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<'a, P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDifferencedFilesByLastModifyTime)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive.into(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<'a, P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::BSTR>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AddDifferencedFilesByLastModifyLSN)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive.into(), bstrlsnstring.into().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDifferencedFilesCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcdifferencedfiles)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut super::super::Foundation::BSTR, pbstrfilespec: *mut super::super::Foundation::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut super::super::Foundation::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetDifferencedFile)(::windows::core::Interface::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), ::core::mem::transmute(pbrecursive), ::core::mem::transmute(pbstrlsnstring), ::core::mem::transmute(pftlastmodifytime)).ok()
    }
    pub unsafe fn SetPrepareForBackupFailureMsg<'a, P0>(&self, wszfailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPrepareForBackupFailureMsg)(::windows::core::Interface::as_raw(self), wszfailuremsg.into()).ok()
    }
    pub unsafe fn SetPostSnapshotFailureMsg<'a, P0>(&self, wszfailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetPostSnapshotFailureMsg)(::windows::core::Interface::as_raw(self), wszfailuremsg.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPrepareForBackupFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPostSnapshotFailureMsg)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::core::Result<bool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAuthoritativeRestore)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<bool>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRollForward)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prolltype), ::core::mem::transmute(pbstrpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetRestoreName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetRestoreName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn SetFailure<'a, P0>(&self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: P0, dwreserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetFailure)(::windows::core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.into(), dwreserved).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFailure(&self, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFailure)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phr), ::core::mem::transmute(phrapplication), ::core::mem::transmute(pbstrapplicationmessage), ::core::mem::transmute(pdwreserved)).ok()
    }
}
impl ::core::convert::From<IVssComponentEx2> for ::windows::core::IUnknown {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssComponentEx2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx2> for ::windows::core::IUnknown {
    fn from(value: &IVssComponentEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssComponentEx2> for IVssComponent {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssComponentEx2> for &'a IVssComponent {
    fn from(value: &'a IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx2> for IVssComponent {
    fn from(value: &IVssComponentEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssComponentEx2> for IVssComponentEx {
    fn from(value: IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssComponentEx2> for &'a IVssComponentEx {
    fn from(value: &'a IVssComponentEx2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssComponentEx2> for IVssComponentEx {
    fn from(value: &IVssComponentEx2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssComponentEx2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssComponentEx2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssComponentEx2 {}
impl ::core::fmt::Debug for IVssComponentEx2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssComponentEx2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssComponentEx2 {
    type Vtable = IVssComponentEx2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b5be0f2_07a9_4e4b_bdd3_cfdc8e2c0d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx2_Vtbl {
    pub base__: IVssComponentEx_Vtbl,
    pub SetFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: ::windows::core::PCWSTR, dwreserved: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phr: *mut ::windows::core::HRESULT, phrapplication: *mut ::windows::core::HRESULT, pbstrapplicationmessage: *mut super::super::Foundation::BSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFailure: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssCreateExpressWriterMetadata(::windows::core::IUnknown);
impl IVssCreateExpressWriterMetadata {
    pub unsafe fn AddExcludeFiles<'a, P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddExcludeFiles)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive).ok()
    }
    pub unsafe fn AddComponent<'a, P0, P1, P2>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P0, wszcomponentname: P1, wszcaption: P2, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddComponent)(::windows::core::Interface::as_raw(self), ct, wszlogicalpath.into(), wszcomponentname.into(), wszcaption.into(), ::core::mem::transmute(pbicon), cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok()
    }
    pub unsafe fn AddFilesToFileGroup<'a, P0, P1, P2, P3, P4>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P4, dwbackuptypemask: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddFilesToFileGroup)(::windows::core::Interface::as_raw(self), wszlogicalpath.into(), wszgroupname.into(), wszpath.into(), wszfilespec.into(), brecursive, wszalternatelocation.into(), dwbackuptypemask).ok()
    }
    pub unsafe fn SetRestoreMethod<'a, P0, P1>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P0, wszuserprocedure: P1, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRestoreMethod)(::windows::core::Interface::as_raw(self), method, wszservice.into(), wszuserprocedure.into(), writerrestore, brebootrequired).ok()
    }
    pub unsafe fn AddComponentDependency<'a, P0, P1, P2, P3>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: ::windows::core::GUID, wszonlogicalpath: P2, wszoncomponentname: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddComponentDependency)(::windows::core::Interface::as_raw(self), wszforlogicalpath.into(), wszforcomponentname.into(), ::core::mem::transmute(onwriterid), wszonlogicalpath.into(), wszoncomponentname.into()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackupSchema)(::windows::core::Interface::as_raw(self), dwschemamask).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAsXML(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).SaveAsXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IVssCreateExpressWriterMetadata> for ::windows::core::IUnknown {
    fn from(value: IVssCreateExpressWriterMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssCreateExpressWriterMetadata> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssCreateExpressWriterMetadata) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssCreateExpressWriterMetadata> for ::windows::core::IUnknown {
    fn from(value: &IVssCreateExpressWriterMetadata) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssCreateExpressWriterMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssCreateExpressWriterMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssCreateExpressWriterMetadata {}
impl ::core::fmt::Debug for IVssCreateExpressWriterMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssCreateExpressWriterMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssCreateExpressWriterMetadata {
    type Vtable = IVssCreateExpressWriterMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c772e77_b26e_427f_92dd_c996f41ea5e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateExpressWriterMetadata_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddExcludeFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: u8) -> ::windows::core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcaption: ::windows::core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows::core::PCWSTR, wszgroupname: ::windows::core::PCWSTR, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows::core::PCWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows::core::PCWSTR, wszuserprocedure: ::windows::core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszforlogicalpath: ::windows::core::PCWSTR, wszforcomponentname: ::windows::core::PCWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: ::windows::core::PCWSTR, wszoncomponentname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAsXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAsXML: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssCreateWriterMetadata(::windows::core::IUnknown);
impl IVssCreateWriterMetadata {
    pub unsafe fn AddIncludeFiles<'a, P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8, wszalternatelocation: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddIncludeFiles)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive, wszalternatelocation.into()).ok()
    }
    pub unsafe fn AddExcludeFiles<'a, P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddExcludeFiles)(::windows::core::Interface::as_raw(self), wszpath.into(), wszfilespec.into(), brecursive).ok()
    }
    pub unsafe fn AddComponent<'a, P0, P1, P2>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P0, wszcomponentname: P1, wszcaption: P2, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddComponent)(::windows::core::Interface::as_raw(self), ct, wszlogicalpath.into(), wszcomponentname.into(), wszcaption.into(), ::core::mem::transmute(pbicon), cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok()
    }
    pub unsafe fn AddDatabaseFiles<'a, P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDatabaseFiles)(::windows::core::Interface::as_raw(self), wszlogicalpath.into(), wszdatabasename.into(), wszpath.into(), wszfilespec.into(), dwbackuptypemask).ok()
    }
    pub unsafe fn AddDatabaseLogFiles<'a, P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddDatabaseLogFiles)(::windows::core::Interface::as_raw(self), wszlogicalpath.into(), wszdatabasename.into(), wszpath.into(), wszfilespec.into(), dwbackuptypemask).ok()
    }
    pub unsafe fn AddFilesToFileGroup<'a, P0, P1, P2, P3, P4>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P4, dwbackuptypemask: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
        P4: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddFilesToFileGroup)(::windows::core::Interface::as_raw(self), wszlogicalpath.into(), wszgroupname.into(), wszpath.into(), wszfilespec.into(), brecursive, wszalternatelocation.into(), dwbackuptypemask).ok()
    }
    pub unsafe fn SetRestoreMethod<'a, P0, P1>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P0, wszuserprocedure: P1, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetRestoreMethod)(::windows::core::Interface::as_raw(self), method, wszservice.into(), wszuserprocedure.into(), writerrestore, brebootrequired).ok()
    }
    pub unsafe fn AddAlternateLocationMapping<'a, P0, P1, P2>(&self, wszsourcepath: P0, wszsourcefilespec: P1, brecursive: u8, wszdestination: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddAlternateLocationMapping)(::windows::core::Interface::as_raw(self), wszsourcepath.into(), wszsourcefilespec.into(), brecursive, wszdestination.into()).ok()
    }
    pub unsafe fn AddComponentDependency<'a, P0, P1, P2, P3>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: ::windows::core::GUID, wszonlogicalpath: P2, wszoncomponentname: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
        P3: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddComponentDependency)(::windows::core::Interface::as_raw(self), wszforlogicalpath.into(), wszforcomponentname.into(), ::core::mem::transmute(onwriterid), wszonlogicalpath.into(), wszoncomponentname.into()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackupSchema)(::windows::core::Interface::as_raw(self), dwschemamask).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocument(&self) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDocument)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Data::Xml::MsXml::IXMLDOMDocument>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SaveAsXML(&self, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SaveAsXML)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrxml)).ok()
    }
}
impl ::core::clone::Clone for IVssCreateWriterMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssCreateWriterMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssCreateWriterMetadata {}
impl ::core::fmt::Debug for IVssCreateWriterMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssCreateWriterMetadata").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssCreateWriterMetadata {
    type Vtable = IVssCreateWriterMetadata_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateWriterMetadata_Vtbl {
    pub AddIncludeFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddExcludeFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: u8) -> ::windows::core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows::core::PCWSTR, wszcomponentname: ::windows::core::PCWSTR, wszcaption: ::windows::core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows::core::HRESULT,
    pub AddDatabaseFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows::core::PCWSTR, wszdatabasename: ::windows::core::PCWSTR, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    pub AddDatabaseLogFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows::core::PCWSTR, wszdatabasename: ::windows::core::PCWSTR, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows::core::PCWSTR, wszgroupname: ::windows::core::PCWSTR, wszpath: ::windows::core::PCWSTR, wszfilespec: ::windows::core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows::core::PCWSTR, dwbackuptypemask: u32) -> ::windows::core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows::core::PCWSTR, wszuserprocedure: ::windows::core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows::core::HRESULT,
    pub AddAlternateLocationMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszsourcepath: ::windows::core::PCWSTR, wszsourcefilespec: ::windows::core::PCWSTR, brecursive: u8, wszdestination: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszforlogicalpath: ::windows::core::PCWSTR, wszforcomponentname: ::windows::core::PCWSTR, onwriterid: ::windows::core::GUID, wszonlogicalpath: ::windows::core::PCWSTR, wszoncomponentname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetDocument: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SaveAsXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SaveAsXML: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt(::windows::core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDiffArea)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeDiffAreaMaximumSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryVolumesSupportedForDiffAreas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszoriginalvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryDiffAreasForVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryDiffAreasOnVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryDiffAreasForSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssDifferentialSoftwareSnapshotMgmt> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssDifferentialSoftwareSnapshotMgmt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssDifferentialSoftwareSnapshotMgmt {}
impl ::core::fmt::Debug for IVssDifferentialSoftwareSnapshotMgmt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssDifferentialSoftwareSnapshotMgmt").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssDifferentialSoftwareSnapshotMgmt {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x214a0f28_b737_4026_b847_4f9e37d79529);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddDiffArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub ChangeDiffAreaMaximumSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::HRESULT,
    pub QueryVolumesSupportedForDiffAreas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryDiffAreasForVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryDiffAreasOnVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryDiffAreasForSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2(::windows::core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddDiffArea)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ChangeDiffAreaMaximumSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryVolumesSupportedForDiffAreas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszoriginalvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryDiffAreasForVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryDiffAreasOnVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryDiffAreasForSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<'a, P0>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).ChangeDiffAreaMaximumSizeEx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace, bvolatile.into()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MigrateDiffAreas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(pwsznewdiffareavolumename)).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryMigrationStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssAsync>(result__)
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSnapshotPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(idsnapshot), priority).ok()
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssDifferentialSoftwareSnapshotMgmt2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt2> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssDifferentialSoftwareSnapshotMgmt2> for &'a IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: &'a IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt2> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssDifferentialSoftwareSnapshotMgmt2 {}
impl ::core::fmt::Debug for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssDifferentialSoftwareSnapshotMgmt2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssDifferentialSoftwareSnapshotMgmt2 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x949d7353_675f_4275_8969_f044c6277815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeDiffAreaMaximumSizeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeDiffAreaMaximumSizeEx: usize,
    pub MigrateDiffAreas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::HRESULT,
    pub QueryMigrationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSnapshotPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3(::windows::core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt3 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddDiffArea)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ChangeDiffAreaMaximumSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.QueryVolumesSupportedForDiffAreas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszoriginalvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.QueryDiffAreasForVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.QueryDiffAreasOnVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.QueryDiffAreasForSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<'a, P0>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.ChangeDiffAreaMaximumSizeEx)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), llmaximumdiffspace, bvolatile.into()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.MigrateDiffAreas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(pwsznewdiffareavolumename)).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.QueryMigrationStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pwszdiffareavolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssAsync>(result__)
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSnapshotPriority)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(idsnapshot), priority).ok()
    }
    pub unsafe fn SetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVolumeProtectLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), protectionlevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16) -> ::windows::core::Result<VSS_VOLUME_PROTECTION_INFO> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetVolumeProtectLevel)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VSS_VOLUME_PROTECTION_INFO>(result__)
    }
    pub unsafe fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearVolumeProtectFault)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename)).ok()
    }
    pub unsafe fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteUnusedDiffAreas)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszdiffareavolumename)).ok()
    }
    pub unsafe fn QuerySnapshotDeltaBitmap(&self, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QuerySnapshotDeltaBitmap)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(idsnapshotolder), ::core::mem::transmute(idsnapshotyounger), ::core::mem::transmute(pcblocksizeperbit), ::core::mem::transmute(pcbitmaplength), ::core::mem::transmute(ppbbitmap)).ok()
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for ::windows::core::IUnknown {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssDifferentialSoftwareSnapshotMgmt3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for ::windows::core::IUnknown {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssDifferentialSoftwareSnapshotMgmt3> for &'a IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: &'a IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn from(value: IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssDifferentialSoftwareSnapshotMgmt3> for &'a IVssDifferentialSoftwareSnapshotMgmt2 {
    fn from(value: &'a IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssDifferentialSoftwareSnapshotMgmt3> for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn from(value: &IVssDifferentialSoftwareSnapshotMgmt3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssDifferentialSoftwareSnapshotMgmt3 {}
impl ::core::fmt::Debug for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssDifferentialSoftwareSnapshotMgmt3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssDifferentialSoftwareSnapshotMgmt3 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x383f7e71_a4c5_401f_b27f_f826289f8458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl,
    pub SetVolumeProtectLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeProtectLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeProtectLevel: usize,
    pub ClearVolumeProtectFault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows::core::HRESULT,
    pub DeleteUnusedDiffAreas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows::core::HRESULT,
    pub QuerySnapshotDeltaBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows::core::GUID, idsnapshotyounger: ::windows::core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssEnumMgmtObject(::windows::core::IUnknown);
impl IVssEnumMgmtObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_MGMT_OBJECT_PROP], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut ::core::option::Option<IVssEnumMgmtObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenum)).ok()
    }
}
impl ::core::convert::From<IVssEnumMgmtObject> for ::windows::core::IUnknown {
    fn from(value: IVssEnumMgmtObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssEnumMgmtObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssEnumMgmtObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssEnumMgmtObject> for ::windows::core::IUnknown {
    fn from(value: &IVssEnumMgmtObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssEnumMgmtObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssEnumMgmtObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssEnumMgmtObject {}
impl ::core::fmt::Debug for IVssEnumMgmtObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssEnumMgmtObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssEnumMgmtObject {
    type Vtable = IVssEnumMgmtObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01954e6b_9254_4e6e_808c_c9e05d007696);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumMgmtObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssEnumObject(::windows::core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_OBJECT_PROP], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(rgelt)), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut ::core::option::Option<IVssEnumObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppenum)).ok()
    }
}
impl ::core::convert::From<IVssEnumObject> for ::windows::core::IUnknown {
    fn from(value: IVssEnumObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssEnumObject> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssEnumObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssEnumObject> for ::windows::core::IUnknown {
    fn from(value: &IVssEnumObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssEnumObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssEnumObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssEnumObject {}
impl ::core::fmt::Debug for IVssEnumObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssEnumObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssEnumObject {
    type Vtable = IVssEnumObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae1c7110_2f60_11d3_8a39_00c04f72d8e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumObject_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(C)]
pub struct IVssExamineWriterMetadata(pub u8);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssExpressWriter(::windows::core::IUnknown);
impl IVssExpressWriter {
    pub unsafe fn CreateMetadata<'a, P0>(&self, writerid: ::windows::core::GUID, writername: P0, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> ::windows::core::Result<IVssCreateExpressWriterMetadata>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(writerid), writername.into(), usagetype, versionmajor, versionminor, reserved, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssCreateExpressWriterMetadata>(result__)
    }
    pub unsafe fn LoadMetadata<'a, P0>(&self, metadata: P0, reserved: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LoadMetadata)(::windows::core::Interface::as_raw(self), metadata.into(), reserved).ok()
    }
    pub unsafe fn Register(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Register)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unregister(&self, writerid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unregister)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(writerid)).ok()
    }
}
impl ::core::convert::From<IVssExpressWriter> for ::windows::core::IUnknown {
    fn from(value: IVssExpressWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssExpressWriter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssExpressWriter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssExpressWriter> for ::windows::core::IUnknown {
    fn from(value: &IVssExpressWriter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssExpressWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssExpressWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssExpressWriter {}
impl ::core::fmt::Debug for IVssExpressWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssExpressWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssExpressWriter {
    type Vtable = IVssExpressWriter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe33affdc_59c7_47b1_97d5_4266598f6235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExpressWriter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CreateMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, writername: ::windows::core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadata: ::windows::core::PCWSTR, reserved: u32) -> ::windows::core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssFileShareSnapshotProvider(::windows::core::IUnknown);
impl IVssFileShareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContext)(::windows::core::Interface::as_raw(self), lcontext).ok()
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<VSS_SNAPSHOT_PROP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSnapshotProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VSS_SNAPSHOT_PROP>(result__)
    }
    pub unsafe fn Query(&self, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Query)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumObject>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<'a, P0>(&self, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: P0, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DeleteSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into(), ::core::mem::transmute(pldeletedsnapshots), ::core::mem::transmute(pnondeletedsnapshotid)).ok()
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPrepareSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), ::core::mem::transmute(pwszsharepath), lnewcontext, ::core::mem::transmute(providerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSupported(&self, pwszsharepath: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsPathSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszsharepath), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsPathSnapshotted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszsharepath), ::core::mem::transmute(pbsnapshotspresent), ::core::mem::transmute(plsnapshotcompatibility)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSnapshotProperty<'a, P0>(&self, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSnapshotProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), esnapshotpropertyid, vproperty.into().abi()).ok()
    }
}
impl ::core::convert::From<IVssFileShareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: IVssFileShareSnapshotProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssFileShareSnapshotProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssFileShareSnapshotProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssFileShareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: &IVssFileShareSnapshotProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssFileShareSnapshotProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssFileShareSnapshotProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssFileShareSnapshotProvider {}
impl ::core::fmt::Debug for IVssFileShareSnapshotProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssFileShareSnapshotProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssFileShareSnapshotProvider {
    type Vtable = IVssFileShareSnapshotProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8636060_7c2e_11df_8c4a_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssFileShareSnapshotProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSnapshots: usize,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathSnapshotted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathSnapshotted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSnapshotProperty: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssHardwareSnapshotProvider(::windows::core::IUnknown);
impl IVssHardwareSnapshotProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AreLunsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lluncount), lcontext, ::core::mem::transmute(rgwszdevices), ::core::mem::transmute(pluninformation), ::core::mem::transmute(pbissupported)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FillInLunInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pluninfo), ::core::mem::transmute(pbissupported)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPrepareSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), lcontext, ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgluninformation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTargetLuns)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgsourceluns), ::core::mem::transmute(rgdestinationluns)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LocateLuns)(::windows::core::Interface::as_raw(self), rgsourceluns.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsourceluns))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLunEmpty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pinformation)).ok()
    }
}
impl ::core::convert::From<IVssHardwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: IVssHardwareSnapshotProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssHardwareSnapshotProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssHardwareSnapshotProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssHardwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: &IVssHardwareSnapshotProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssHardwareSnapshotProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssHardwareSnapshotProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssHardwareSnapshotProvider {}
impl ::core::fmt::Debug for IVssHardwareSnapshotProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssHardwareSnapshotProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssHardwareSnapshotProvider {
    type Vtable = IVssHardwareSnapshotProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9593a157_44e9_4344_bbeb_44fbf9b06b10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub AreLunsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    AreLunsSupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub FillInLunInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    FillInLunInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    BeginPrepareSnapshot: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub GetTargetLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    GetTargetLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub LocateLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    LocateLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnLunEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnLunEmpty: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssHardwareSnapshotProviderEx(::windows::core::IUnknown);
impl IVssHardwareSnapshotProviderEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AreLunsSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lluncount), lcontext, ::core::mem::transmute(rgwszdevices), ::core::mem::transmute(pluninformation), ::core::mem::transmute(pbissupported)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FillInLunInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pluninfo), ::core::mem::transmute(pbissupported)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginPrepareSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), lcontext, ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgluninformation)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTargetLuns)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lluncount), ::core::mem::transmute(rgdevicenames), ::core::mem::transmute(rgsourceluns), ::core::mem::transmute(rgdestinationluns)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LocateLuns)(::windows::core::Interface::as_raw(self), rgsourceluns.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgsourceluns))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnLunEmpty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(wszdevicename), ::core::mem::transmute(pinformation)).ok()
    }
    pub unsafe fn GetProviderCapabilities(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderCapabilities)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnLunStateChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psnapshotluns), ::core::mem::transmute(poriginalluns), ::core::mem::transmute(dwcount), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<IVssAsync> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ResyncLuns)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psourceluns), ::core::mem::transmute(ptargetluns), ::core::mem::transmute(dwcount), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssAsync>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnReuseLuns)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psnapshotluns), ::core::mem::transmute(poriginalluns), ::core::mem::transmute(dwcount)).ok()
    }
}
impl ::core::convert::From<IVssHardwareSnapshotProviderEx> for ::windows::core::IUnknown {
    fn from(value: IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssHardwareSnapshotProviderEx> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssHardwareSnapshotProviderEx> for ::windows::core::IUnknown {
    fn from(value: &IVssHardwareSnapshotProviderEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVssHardwareSnapshotProviderEx> for IVssHardwareSnapshotProvider {
    fn from(value: IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssHardwareSnapshotProviderEx> for &'a IVssHardwareSnapshotProvider {
    fn from(value: &'a IVssHardwareSnapshotProviderEx) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssHardwareSnapshotProviderEx> for IVssHardwareSnapshotProvider {
    fn from(value: &IVssHardwareSnapshotProviderEx) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssHardwareSnapshotProviderEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssHardwareSnapshotProviderEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssHardwareSnapshotProviderEx {}
impl ::core::fmt::Debug for IVssHardwareSnapshotProviderEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssHardwareSnapshotProviderEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssHardwareSnapshotProviderEx {
    type Vtable = IVssHardwareSnapshotProviderEx_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5ba925_cdb1_4d11_a71f_339eb7e709fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProviderEx_Vtbl {
    pub base__: IVssHardwareSnapshotProvider_Vtbl,
    pub GetProviderCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnLunStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnLunStateChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub ResyncLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    ResyncLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnReuseLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnReuseLuns: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssProviderCreateSnapshotSet(::windows::core::IUnknown);
impl IVssProviderCreateSnapshotSet {
    pub unsafe fn EndPrepareSnapshots(&self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndPrepareSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PreCommitSnapshots(&self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreCommitSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn CommitSnapshots(&self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CommitSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PostCommitSnapshots(&self, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PostCommitSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), lsnapshotscount).ok()
    }
    pub unsafe fn PreFinalCommitSnapshots(&self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PreFinalCommitSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PostFinalCommitSnapshots(&self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PostFinalCommitSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn AbortSnapshots(&self, snapshotsetid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AbortSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
}
impl ::core::convert::From<IVssProviderCreateSnapshotSet> for ::windows::core::IUnknown {
    fn from(value: IVssProviderCreateSnapshotSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssProviderCreateSnapshotSet> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssProviderCreateSnapshotSet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssProviderCreateSnapshotSet> for ::windows::core::IUnknown {
    fn from(value: &IVssProviderCreateSnapshotSet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssProviderCreateSnapshotSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssProviderCreateSnapshotSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssProviderCreateSnapshotSet {}
impl ::core::fmt::Debug for IVssProviderCreateSnapshotSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssProviderCreateSnapshotSet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssProviderCreateSnapshotSet {
    type Vtable = IVssProviderCreateSnapshotSet_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f894e5b_1e39_4778_8e23_9abad9f0e08c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderCreateSnapshotSet_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub EndPrepareSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PreCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PostCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, lsnapshotscount: i32) -> ::windows::core::HRESULT,
    pub PreFinalCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PostFinalCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub AbortSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssProviderNotifications(::windows::core::IUnknown);
impl IVssProviderNotifications {
    pub unsafe fn OnLoad<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).OnLoad)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<'a, P0>(&self, bforceunload: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnUnload)(::windows::core::Interface::as_raw(self), bforceunload.into()).ok()
    }
}
impl ::core::convert::From<IVssProviderNotifications> for ::windows::core::IUnknown {
    fn from(value: IVssProviderNotifications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssProviderNotifications> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssProviderNotifications) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssProviderNotifications> for ::windows::core::IUnknown {
    fn from(value: &IVssProviderNotifications) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssProviderNotifications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssProviderNotifications {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssProviderNotifications {}
impl ::core::fmt::Debug for IVssProviderNotifications {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssProviderNotifications").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssProviderNotifications {
    type Vtable = IVssProviderNotifications_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe561901f_03a5_4afe_86d0_72baeece7004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderNotifications_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUnload: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssSnapshotMgmt(::windows::core::IUnknown);
impl IVssSnapshotMgmt {
    pub unsafe fn GetProviderMgmtInterface(&self, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProviderMgmtInterface)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid), ::core::mem::transmute(interfaceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn QueryVolumesSupportedForSnapshots(&self, providerid: ::windows::core::GUID, lcontext: i32) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryVolumesSupportedForSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(providerid), lcontext, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumMgmtObject>(result__)
    }
    pub unsafe fn QuerySnapshotsByVolume(&self, pwszvolumename: *const u16, providerid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QuerySnapshotsByVolume)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(providerid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumObject>(result__)
    }
}
impl ::core::convert::From<IVssSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: IVssSnapshotMgmt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssSnapshotMgmt> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssSnapshotMgmt) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssSnapshotMgmt> for ::windows::core::IUnknown {
    fn from(value: &IVssSnapshotMgmt) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssSnapshotMgmt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssSnapshotMgmt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssSnapshotMgmt {}
impl ::core::fmt::Debug for IVssSnapshotMgmt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssSnapshotMgmt").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssSnapshotMgmt {
    type Vtable = IVssSnapshotMgmt_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfa7df749_66e7_4986_a27f_e2f04ae53772);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetProviderMgmtInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, interfaceid: *const ::windows::core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryVolumesSupportedForSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, lcontext: i32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QuerySnapshotsByVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssSnapshotMgmt2(::windows::core::IUnknown);
impl IVssSnapshotMgmt2 {
    pub unsafe fn GetMinDiffAreaSize(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMinDiffAreaSize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<i64>(result__)
    }
}
impl ::core::convert::From<IVssSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: IVssSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssSnapshotMgmt2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssSnapshotMgmt2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssSnapshotMgmt2> for ::windows::core::IUnknown {
    fn from(value: &IVssSnapshotMgmt2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssSnapshotMgmt2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssSnapshotMgmt2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssSnapshotMgmt2 {}
impl ::core::fmt::Debug for IVssSnapshotMgmt2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssSnapshotMgmt2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssSnapshotMgmt2 {
    type Vtable = IVssSnapshotMgmt2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f61ec39_fe82_45f2_a3f0_768b5d427102);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetMinDiffAreaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssSoftwareSnapshotProvider(::windows::core::IUnknown);
impl IVssSoftwareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContext)(::windows::core::Interface::as_raw(self), lcontext).ok()
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<VSS_SNAPSHOT_PROP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSnapshotProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<VSS_SNAPSHOT_PROP>(result__)
    }
    pub unsafe fn Query(&self, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Query)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssEnumObject>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<'a, P0>(&self, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: P0, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).DeleteSnapshots)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into(), ::core::mem::transmute(pldeletedsnapshots), ::core::mem::transmute(pnondeletedsnapshotid)).ok()
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginPrepareSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), ::core::mem::transmute(pwszvolumename), lnewcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).IsVolumeSupported)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsVolumeSnapshotted)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolumename), ::core::mem::transmute(pbsnapshotspresent), ::core::mem::transmute(plsnapshotcompatibility)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetSnapshotProperty<'a, P0>(&self, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::Com::VARIANT>>,
    {
        (::windows::core::Interface::vtable(self).SetSnapshotProperty)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), esnapshotpropertyid, vproperty.into().abi()).ok()
    }
    pub unsafe fn RevertToSnapshot(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RevertToSnapshot)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(snapshotid)).ok()
    }
    pub unsafe fn QueryRevertStatus(&self, pwszvolume: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).QueryRevertStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwszvolume), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssAsync>(result__)
    }
}
impl ::core::convert::From<IVssSoftwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: IVssSoftwareSnapshotProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssSoftwareSnapshotProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssSoftwareSnapshotProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssSoftwareSnapshotProvider> for ::windows::core::IUnknown {
    fn from(value: &IVssSoftwareSnapshotProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssSoftwareSnapshotProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssSoftwareSnapshotProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssSoftwareSnapshotProvider {}
impl ::core::fmt::Debug for IVssSoftwareSnapshotProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssSoftwareSnapshotProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssSoftwareSnapshotProvider {
    type Vtable = IVssSoftwareSnapshotProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x609e123e_2c5a_44d3_8f01_0b1d9a47d1ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSoftwareSnapshotProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows::core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows::core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows::core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSnapshots: usize,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolumeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolumeSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolumeSnapshotted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolumeSnapshotted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetSnapshotProperty: usize,
    pub RevertToSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub QueryRevertStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWMDependency(::windows::core::IUnknown);
impl IVssWMDependency {
    pub unsafe fn GetWriterId(&self, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWriterId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pwriterid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLogicalPath(&self, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLogicalPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrlogicalpath)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetComponentName(&self, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComponentName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbstrcomponentname)).ok()
    }
}
impl ::core::convert::From<IVssWMDependency> for ::windows::core::IUnknown {
    fn from(value: IVssWMDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssWMDependency> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssWMDependency) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssWMDependency> for ::windows::core::IUnknown {
    fn from(value: &IVssWMDependency) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssWMDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssWMDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssWMDependency {}
impl ::core::fmt::Debug for IVssWMDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssWMDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssWMDependency {
    type Vtable = IVssWMDependency_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMDependency_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetWriterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLogicalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLogicalPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetComponentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetComponentName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWMFiledesc(::windows::core::IUnknown);
impl IVssWMFiledesc {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPath)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilespec(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilespec)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetRecursive(&self) -> ::windows::core::Result<bool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRecursive)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<bool>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAlternateLocation(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetAlternateLocation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn GetBackupTypeMask(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBackupTypeMask)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IVssWMFiledesc> for ::windows::core::IUnknown {
    fn from(value: IVssWMFiledesc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssWMFiledesc> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssWMFiledesc) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssWMFiledesc> for ::windows::core::IUnknown {
    fn from(value: &IVssWMFiledesc) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssWMFiledesc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssWMFiledesc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssWMFiledesc {}
impl ::core::fmt::Debug for IVssWMFiledesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssWMFiledesc").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssWMFiledesc {
    type Vtable = IVssWMFiledesc_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMFiledesc_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPath: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilespec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilespec: usize,
    pub GetRecursive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAlternateLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAlternateLocation: usize,
    pub GetBackupTypeMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWriterComponents(::windows::core::IUnknown);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(&self, pccomponents: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetComponentCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pccomponents)).ok()
    }
    pub unsafe fn GetWriterInfo(&self, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWriterInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidinstance), ::core::mem::transmute(pidwriter)).ok()
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> ::windows::core::Result<IVssComponent> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetComponent)(::windows::core::Interface::as_raw(self), icomponent, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IVssComponent>(result__)
    }
}
impl ::core::clone::Clone for IVssWriterComponents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssWriterComponents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssWriterComponents {}
impl ::core::fmt::Debug for IVssWriterComponents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssWriterComponents").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssWriterComponents {
    type Vtable = IVssWriterComponents_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterComponents_Vtbl {
    pub GetComponentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows::core::HRESULT,
    pub GetWriterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows::core::GUID, pidwriter: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWriterImpl(::windows::core::IUnknown);
impl IVssWriterImpl {
    pub unsafe fn Initialize<'a, P0, P1>(&self, writerid: ::windows::core::GUID, wszwritername: P0, wszwriterinstancename: P1, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(writerid), wszwritername.into(), wszwriterinstancename.into(), dwmajorversion, dwminorversion, ut, st, nlevel, dwtimeout, aws, biothrottlingonly).ok()
    }
    pub unsafe fn Subscribe(&self, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Subscribe)(::windows::core::Interface::as_raw(self), dwsubscribetimeout, dweventflags).ok()
    }
    pub unsafe fn Unsubscribe(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unsubscribe)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Uninitialize(&self) {
        (::windows::core::Interface::vtable(self).Uninitialize)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentVolumeArray(&self) -> *mut ::windows::core::PWSTR {
        (::windows::core::Interface::vtable(self).GetCurrentVolumeArray)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentVolumeCount(&self) -> u32 {
        (::windows::core::Interface::vtable(self).GetCurrentVolumeCount)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetSnapshotDeviceName<'a, P0>(&self, wszoriginalvolume: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSnapshotDeviceName)(::windows::core::Interface::as_raw(self), wszoriginalvolume.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    pub unsafe fn GetCurrentSnapshotSetId(&self) -> ::windows::core::GUID {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetCurrentSnapshotSetId)(::windows::core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetContext(&self) -> i32 {
        (::windows::core::Interface::vtable(self).GetContext)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetCurrentLevel(&self) -> VSS_APPLICATION_LEVEL {
        (::windows::core::Interface::vtable(self).GetCurrentLevel)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn IsPathAffected<'a, P0>(&self, wszpath: P0) -> bool
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).IsPathAffected)(::windows::core::Interface::as_raw(self), wszpath.into())
    }
    pub unsafe fn IsBootableSystemStateBackedUp(&self) -> bool {
        (::windows::core::Interface::vtable(self).IsBootableSystemStateBackedUp)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn AreComponentsSelected(&self) -> bool {
        (::windows::core::Interface::vtable(self).AreComponentsSelected)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetBackupType(&self) -> VSS_BACKUP_TYPE {
        (::windows::core::Interface::vtable(self).GetBackupType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn GetRestoreType(&self) -> VSS_RESTORE_TYPE {
        (::windows::core::Interface::vtable(self).GetRestoreType)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetWriterFailure(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWriterFailure)(::windows::core::Interface::as_raw(self), hr).ok()
    }
    pub unsafe fn IsPartialFileSupportEnabled(&self) -> bool {
        (::windows::core::Interface::vtable(self).IsPartialFileSupportEnabled)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn InstallAlternateWriter(&self, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InstallAlternateWriter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(idwriter), ::core::mem::transmute(clsid)).ok()
    }
    pub unsafe fn GetIdentityInformation(&self) -> *mut IVssExamineWriterMetadata {
        (::windows::core::Interface::vtable(self).GetIdentityInformation)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn SetWriterFailureEx<'a, P0>(&self, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetWriterFailureEx)(::windows::core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.into()).ok()
    }
    pub unsafe fn GetSessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSessionId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn IsWriterShuttingDown(&self) -> bool {
        (::windows::core::Interface::vtable(self).IsWriterShuttingDown)(::windows::core::Interface::as_raw(self))
    }
}
impl ::core::convert::From<IVssWriterImpl> for ::windows::core::IUnknown {
    fn from(value: IVssWriterImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVssWriterImpl> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVssWriterImpl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVssWriterImpl> for ::windows::core::IUnknown {
    fn from(value: &IVssWriterImpl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IVssWriterImpl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVssWriterImpl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVssWriterImpl {}
impl ::core::fmt::Debug for IVssWriterImpl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVssWriterImpl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVssWriterImpl {
    type Vtable = IVssWriterImpl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterImpl_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writerid: ::windows::core::GUID, wszwritername: ::windows::core::PCWSTR, wszwriterinstancename: ::windows::core::PCWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: u8) -> ::windows::core::HRESULT,
    pub Subscribe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwsubscribetimeout: u32, dweventflags: u32) -> ::windows::core::HRESULT,
    pub Unsubscribe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub GetCurrentVolumeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut ::windows::core::PWSTR,
    pub GetCurrentVolumeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetSnapshotDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszoriginalvolume: ::windows::core::PCWSTR, ppwszsnapshotdevice: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetCurrentSnapshotSetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID),
    pub GetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> i32,
    pub GetCurrentLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> VSS_APPLICATION_LEVEL,
    pub IsPathAffected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows::core::PCWSTR) -> bool,
    pub IsBootableSystemStateBackedUp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub AreComponentsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub GetBackupType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> VSS_BACKUP_TYPE,
    pub GetRestoreType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> VSS_RESTORE_TYPE,
    pub SetWriterFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub IsPartialFileSupportEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
    pub InstallAlternateWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idwriter: ::windows::core::GUID, clsid: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetIdentityInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> *mut IVssExamineWriterMetadata,
    pub SetWriterFailureEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, hrapplication: ::windows::core::HRESULT, wszapplicationmessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idsession: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IsWriterShuttingDown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> bool,
}
pub const VSSCoordinator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(3i32);
impl ::core::marker::Copy for VSS_ALTERNATE_WRITER_STATE {}
impl ::core::clone::Clone for VSS_ALTERNATE_WRITER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_ALTERNATE_WRITER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_ALTERNATE_WRITER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_ALTERNATE_WRITER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_ALTERNATE_WRITER_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_APPLICATION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(-1i32);
impl ::core::marker::Copy for VSS_APPLICATION_LEVEL {}
impl ::core::clone::Clone for VSS_APPLICATION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_APPLICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_APPLICATION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_APPLICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_APPLICATION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_BACKUP_SCHEMA(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(64i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(128i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4096i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8192i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16384i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32768i32);
impl ::core::marker::Copy for VSS_BACKUP_SCHEMA {}
impl ::core::clone::Clone for VSS_BACKUP_SCHEMA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_BACKUP_SCHEMA {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_BACKUP_SCHEMA {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_BACKUP_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_BACKUP_SCHEMA").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_BACKUP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(6i32);
impl ::core::marker::Copy for VSS_BACKUP_TYPE {}
impl ::core::clone::Clone for VSS_BACKUP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_BACKUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_BACKUP_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_BACKUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_BACKUP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_COMPONENT_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(4i32);
impl ::core::marker::Copy for VSS_COMPONENT_FLAGS {}
impl ::core::clone::Clone for VSS_COMPONENT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_COMPONENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_COMPONENT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_COMPONENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(2i32);
impl ::core::marker::Copy for VSS_COMPONENT_TYPE {}
impl ::core::clone::Clone for VSS_COMPONENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_COMPONENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_COMPONENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_COMPONENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_DIFF_AREA_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszDiffAreaVolumeName: *mut u16,
    pub m_llMaximumDiffSpace: i64,
    pub m_llAllocatedDiffSpace: i64,
    pub m_llUsedDiffSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_AREA_PROP {}
impl ::core::clone::Clone for VSS_DIFF_AREA_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSS_DIFF_AREA_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_DIFF_AREA_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszDiffAreaVolumeName", &self.m_pwszDiffAreaVolumeName).field("m_llMaximumDiffSpace", &self.m_llMaximumDiffSpace).field("m_llAllocatedDiffSpace", &self.m_llAllocatedDiffSpace).field("m_llUsedDiffSpace", &self.m_llUsedDiffSpace).finish()
    }
}
unsafe impl ::windows::core::Abi for VSS_DIFF_AREA_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_DIFF_AREA_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_DIFF_AREA_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_DIFF_AREA_PROP {}
impl ::core::default::Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_DIFF_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
    pub m_llVolumeFreeSpace: i64,
    pub m_llVolumeTotalSpace: i64,
}
impl ::core::marker::Copy for VSS_DIFF_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_DIFF_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSS_DIFF_VOLUME_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_DIFF_VOLUME_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName).field("m_llVolumeFreeSpace", &self.m_llVolumeFreeSpace).field("m_llVolumeTotalSpace", &self.m_llVolumeTotalSpace).finish()
    }
}
unsafe impl ::windows::core::Abi for VSS_DIFF_VOLUME_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_DIFF_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_DIFF_VOLUME_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_DIFF_VOLUME_PROP {}
impl ::core::default::Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212280i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212267i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212282i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212287i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212286i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212278i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212268i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212284i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212285i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212269i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212281i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212270i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212283i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212266i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_AUTORECOVERY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212293i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BAD_STATE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212543i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212298i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212290i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212288i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212498i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212528i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212271i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212292i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212525i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FSS_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212265i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212524i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212513i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212527i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_LEGACY_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212297i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212514i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212510i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212521i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212526i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_DISK: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212296i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212295i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212294i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212500i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212291i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212497i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212531i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_NOT_FOUND: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212536i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212541i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212537i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212540i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_VETO: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212538i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REBOOT_REQUIRED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212505i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212509i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212508i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212289i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212507i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_VOLUME_LOST: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212506i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212501i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212522i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212511i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212504i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212503i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212542i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212529i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212523i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSELECTED_VOLUME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212502i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212517i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_IN_USE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212515i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212499i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212532i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212530i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212304i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212300i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212303i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212490i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212299i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212301i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212302i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212518i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212520i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212519i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2147212279i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(3i32);
impl ::core::marker::Copy for VSS_FILE_RESTORE_STATUS {}
impl ::core::clone::Clone for VSS_FILE_RESTORE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_FILE_RESTORE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_FILE_RESTORE_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_FILE_RESTORE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_FILE_RESTORE_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_FILE_SPEC_BACKUP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2048i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(65536i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(3840i32);
impl ::core::marker::Copy for VSS_FILE_SPEC_BACKUP_TYPE {}
impl ::core::clone::Clone for VSS_FILE_SPEC_BACKUP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_FILE_SPEC_BACKUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_FILE_SPEC_BACKUP_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_FILE_SPEC_BACKUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_FILE_SPEC_BACKUP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_HARDWARE_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2048i32);
impl ::core::marker::Copy for VSS_HARDWARE_OPTIONS {}
impl ::core::clone::Clone for VSS_HARDWARE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_HARDWARE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_HARDWARE_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_HARDWARE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_HARDWARE_OPTIONS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_MGMT_OBJECT_PROP {
    pub Type: VSS_MGMT_OBJECT_TYPE,
    pub Obj: VSS_MGMT_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VSS_MGMT_OBJECT_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_MGMT_OBJECT_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_MGMT_OBJECT_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_MGMT_OBJECT_PROP {}
impl ::core::default::Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(3i32);
impl ::core::marker::Copy for VSS_MGMT_OBJECT_TYPE {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_MGMT_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_MGMT_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_MGMT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_MGMT_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub union VSS_MGMT_OBJECT_UNION {
    pub Vol: VSS_VOLUME_PROP,
    pub DiffVol: VSS_DIFF_VOLUME_PROP,
    pub DiffArea: VSS_DIFF_AREA_PROP,
}
impl ::core::marker::Copy for VSS_MGMT_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_MGMT_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VSS_MGMT_OBJECT_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_MGMT_OBJECT_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_MGMT_OBJECT_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_MGMT_OBJECT_UNION {}
impl ::core::default::Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_OBJECT_PROP {
    pub Type: VSS_OBJECT_TYPE,
    pub Obj: VSS_OBJECT_UNION,
}
impl ::core::marker::Copy for VSS_OBJECT_PROP {}
impl ::core::clone::Clone for VSS_OBJECT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VSS_OBJECT_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_OBJECT_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_OBJECT_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_OBJECT_PROP {}
impl ::core::default::Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(5i32);
impl ::core::marker::Copy for VSS_OBJECT_TYPE {}
impl ::core::clone::Clone for VSS_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub union VSS_OBJECT_UNION {
    pub Snap: VSS_SNAPSHOT_PROP,
    pub Prov: VSS_PROVIDER_PROP,
}
impl ::core::marker::Copy for VSS_OBJECT_UNION {}
impl ::core::clone::Clone for VSS_OBJECT_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for VSS_OBJECT_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_OBJECT_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_OBJECT_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_OBJECT_UNION {}
impl ::core::default::Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_PROTECTION_FAULT(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(14i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(16i32);
impl ::core::marker::Copy for VSS_PROTECTION_FAULT {}
impl ::core::clone::Clone for VSS_PROTECTION_FAULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_PROTECTION_FAULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROTECTION_FAULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_PROTECTION_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROTECTION_FAULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_PROTECTION_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(1i32);
impl ::core::marker::Copy for VSS_PROTECTION_LEVEL {}
impl ::core::clone::Clone for VSS_PROTECTION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROTECTION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_PROVIDER_CAPABILITIES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(32i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(64i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(128i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(512i32);
impl ::core::marker::Copy for VSS_PROVIDER_CAPABILITIES {}
impl ::core::clone::Clone for VSS_PROVIDER_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_PROVIDER_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROVIDER_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_PROVIDER_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows::core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows::core::GUID,
    pub m_ClassId: ::windows::core::GUID,
}
impl ::core::marker::Copy for VSS_PROVIDER_PROP {}
impl ::core::clone::Clone for VSS_PROVIDER_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSS_PROVIDER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_PROVIDER_PROP").field("m_ProviderId", &self.m_ProviderId).field("m_pwszProviderName", &self.m_pwszProviderName).field("m_eProviderType", &self.m_eProviderType).field("m_pwszProviderVersion", &self.m_pwszProviderVersion).field("m_ProviderVersionId", &self.m_ProviderVersionId).field("m_ClassId", &self.m_ClassId).finish()
    }
}
unsafe impl ::windows::core::Abi for VSS_PROVIDER_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_PROVIDER_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_PROVIDER_PROP {}
impl ::core::default::Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_PROVIDER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(4i32);
impl ::core::marker::Copy for VSS_PROVIDER_TYPE {}
impl ::core::clone::Clone for VSS_PROVIDER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_PROVIDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_PROVIDER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_PROVIDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_RECOVERY_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(512i32);
impl ::core::marker::Copy for VSS_RECOVERY_OPTIONS {}
impl ::core::clone::Clone for VSS_RECOVERY_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_RECOVERY_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_RECOVERY_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_RECOVERY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RECOVERY_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_RESTOREMETHOD_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(8i32);
impl ::core::marker::Copy for VSS_RESTOREMETHOD_ENUM {}
impl ::core::clone::Clone for VSS_RESTOREMETHOD_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_RESTOREMETHOD_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_RESTOREMETHOD_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_RESTOREMETHOD_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RESTOREMETHOD_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_RESTORE_TARGET(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(4i32);
impl ::core::marker::Copy for VSS_RESTORE_TARGET {}
impl ::core::clone::Clone for VSS_RESTORE_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_RESTORE_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_RESTORE_TARGET {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_RESTORE_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RESTORE_TARGET").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_RESTORE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(3i32);
impl ::core::marker::Copy for VSS_RESTORE_TYPE {}
impl ::core::clone::Clone for VSS_RESTORE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_RESTORE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_RESTORE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_RESTORE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RESTORE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_ROLLFORWARD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(3i32);
impl ::core::marker::Copy for VSS_ROLLFORWARD_TYPE {}
impl ::core::clone::Clone for VSS_ROLLFORWARD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_ROLLFORWARD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_ROLLFORWARD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_ROLLFORWARD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_ROLLFORWARD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SNAPSHOT_COMPATIBILITY(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(2i32);
impl ::core::marker::Copy for VSS_SNAPSHOT_COMPATIBILITY {}
impl ::core::clone::Clone for VSS_SNAPSHOT_COMPATIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_COMPATIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_COMPATIBILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_COMPATIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_COMPATIBILITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SNAPSHOT_CONTEXT(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(25i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(29i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(-1i32);
impl ::core::marker::Copy for VSS_SNAPSHOT_CONTEXT {}
impl ::core::clone::Clone for VSS_SNAPSHOT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_CONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_CONTEXT {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_CONTEXT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows::core::GUID,
    pub m_SnapshotSetId: ::windows::core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows::core::GUID,
    pub m_lSnapshotAttributes: i32,
    pub m_tsCreationTimestamp: i64,
    pub m_eStatus: VSS_SNAPSHOT_STATE,
}
impl ::core::marker::Copy for VSS_SNAPSHOT_PROP {}
impl ::core::clone::Clone for VSS_SNAPSHOT_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_SNAPSHOT_PROP")
            .field("m_SnapshotId", &self.m_SnapshotId)
            .field("m_SnapshotSetId", &self.m_SnapshotSetId)
            .field("m_lSnapshotsCount", &self.m_lSnapshotsCount)
            .field("m_pwszSnapshotDeviceObject", &self.m_pwszSnapshotDeviceObject)
            .field("m_pwszOriginalVolumeName", &self.m_pwszOriginalVolumeName)
            .field("m_pwszOriginatingMachine", &self.m_pwszOriginatingMachine)
            .field("m_pwszServiceMachine", &self.m_pwszServiceMachine)
            .field("m_pwszExposedName", &self.m_pwszExposedName)
            .field("m_pwszExposedPath", &self.m_pwszExposedPath)
            .field("m_ProviderId", &self.m_ProviderId)
            .field("m_lSnapshotAttributes", &self.m_lSnapshotAttributes)
            .field("m_tsCreationTimestamp", &self.m_tsCreationTimestamp)
            .field("m_eStatus", &self.m_eStatus)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_SNAPSHOT_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_SNAPSHOT_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_SNAPSHOT_PROP {}
impl ::core::default::Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(13i32);
impl ::core::marker::Copy for VSS_SNAPSHOT_PROPERTY_ID {}
impl ::core::clone::Clone for VSS_SNAPSHOT_PROPERTY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_PROPERTY_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_PROPERTY_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SNAPSHOT_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(14i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(16i32);
impl ::core::marker::Copy for VSS_SNAPSHOT_STATE {}
impl ::core::clone::Clone for VSS_SNAPSHOT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_SNAPSHOT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SOURCE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(3i32);
impl ::core::marker::Copy for VSS_SOURCE_TYPE {}
impl ::core::clone::Clone for VSS_SOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_SOURCE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SOURCE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SUBSCRIBE_MASK(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(-1i32);
impl ::core::marker::Copy for VSS_SUBSCRIBE_MASK {}
impl ::core::clone::Clone for VSS_SUBSCRIBE_MASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_SUBSCRIBE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_SUBSCRIBE_MASK {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_SUBSCRIBE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SUBSCRIBE_MASK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(271115i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_FINISHED: ::windows::core::HRESULT = ::windows::core::HRESULT(271114i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_PENDING: ::windows::core::HRESULT = ::windows::core::HRESULT(271113i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows::core::HRESULT = ::windows::core::HRESULT(271137i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_USAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = VSS_USAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = VSS_USAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = VSS_USAGE_TYPE(4i32);
impl ::core::marker::Copy for VSS_USAGE_TYPE {}
impl ::core::clone::Clone for VSS_USAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_USAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_USAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_USAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_USAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_VOLUME_PROP {
    pub m_pwszVolumeName: *mut u16,
    pub m_pwszVolumeDisplayName: *mut u16,
}
impl ::core::marker::Copy for VSS_VOLUME_PROP {}
impl ::core::clone::Clone for VSS_VOLUME_PROP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VSS_VOLUME_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_VOLUME_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName).finish()
    }
}
unsafe impl ::windows::core::Abi for VSS_VOLUME_PROP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for VSS_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_VOLUME_PROP>()) == 0 }
    }
}
impl ::core::cmp::Eq for VSS_VOLUME_PROP {}
impl ::core::default::Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VSS_VOLUME_PROTECTION_INFO {
    pub m_protectionLevel: VSS_PROTECTION_LEVEL,
    pub m_volumeIsOfflineForProtection: super::super::Foundation::BOOL,
    pub m_protectionFault: VSS_PROTECTION_FAULT,
    pub m_failureStatus: i32,
    pub m_volumeHasUnusedDiffArea: super::super::Foundation::BOOL,
    pub m_reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VSS_VOLUME_PROTECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VSS_VOLUME_PROTECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_VOLUME_PROTECTION_INFO").field("m_protectionLevel", &self.m_protectionLevel).field("m_volumeIsOfflineForProtection", &self.m_volumeIsOfflineForProtection).field("m_protectionFault", &self.m_protectionFault).field("m_failureStatus", &self.m_failureStatus).field("m_volumeHasUnusedDiffArea", &self.m_volumeHasUnusedDiffArea).field("m_reserved", &self.m_reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VSS_VOLUME_PROTECTION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VSS_VOLUME_PROTECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VSS_VOLUME_PROTECTION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VSS_VOLUME_PROTECTION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VSS_VOLUME_PROTECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(32i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(64i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(128i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(65536i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(131072i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(262144i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(524288i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1048576i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2097152i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4194304i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8388608i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16777216i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(33554432i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(67108864i32);
impl ::core::marker::Copy for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {}
impl ::core::clone::Clone for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_VOLUME_SNAPSHOT_ATTRIBUTES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_WRITERRESTORE_ENUM(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(3i32);
impl ::core::marker::Copy for VSS_WRITERRESTORE_ENUM {}
impl ::core::clone::Clone for VSS_WRITERRESTORE_ENUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_WRITERRESTORE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_WRITERRESTORE_ENUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_WRITERRESTORE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_WRITERRESTORE_ENUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_WRITER_STATE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_STABLE: VSS_WRITER_STATE = VSS_WRITER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = VSS_WRITER_STATE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = VSS_WRITER_STATE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(14i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_COUNT: VSS_WRITER_STATE = VSS_WRITER_STATE(16i32);
impl ::core::marker::Copy for VSS_WRITER_STATE {}
impl ::core::clone::Clone for VSS_WRITER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VSS_WRITER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VSS_WRITER_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for VSS_WRITER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_WRITER_STATE").field(&self.0).finish()
    }
}
pub const VssSnapshotMgmt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
