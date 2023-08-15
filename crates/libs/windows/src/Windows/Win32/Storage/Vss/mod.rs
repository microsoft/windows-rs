#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[inline]
pub unsafe fn CreateVssExpressWriterInternal() -> ::windows_core::Result<IVssExpressWriter> {
    ::windows_targets::link!("vssapi.dll" "system" fn CreateVssExpressWriterInternal(ppwriter : *mut * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateVssExpressWriterInternal(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssAdmin(::windows_core::IUnknown);
impl IVssAdmin {
    pub unsafe fn RegisterProvider(&self, pproviderid: ::windows_core::GUID, classid: ::windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RegisterProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pproviderid), ::core::mem::transmute(classid), pwszprovidername, eprovidertype, pwszproviderversion, ::core::mem::transmute(providerversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).UnregisterProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows_core::Result<IVssEnumObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryProviders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortAllSnapshotsInProgress)(::windows_core::Interface::as_raw(self)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssAdmin, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssAdmin {
    type Vtable = IVssAdmin_Vtbl;
}
impl ::core::clone::Clone for IVssAdmin {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssAdmin {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x77ed5996_2f63_11d3_8a39_00c04f72d8e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdmin_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub RegisterProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderid: ::windows_core::GUID, classid: ::windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub UnregisterProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub QueryProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AbortAllSnapshotsInProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssAdminEx(::windows_core::IUnknown);
impl IVssAdminEx {
    pub unsafe fn RegisterProvider(&self, pproviderid: ::windows_core::GUID, classid: ::windows_core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.RegisterProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pproviderid), ::core::mem::transmute(classid), pwszprovidername, eprovidertype, pwszproviderversion, ::core::mem::transmute(providerversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.UnregisterProvider)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows_core::Result<IVssEnumObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryProviders)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AbortAllSnapshotsInProgress)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetProviderCapability(&self, pproviderid: ::windows_core::GUID) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProviderCapability)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pproviderid), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProviderContext(&self, providerid: ::windows_core::GUID) -> ::windows_core::Result<i32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProviderContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(providerid), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetProviderContext(&self, providerid: ::windows_core::GUID, lcontext: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProviderContext)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(providerid), lcontext).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssAdminEx, ::windows_core::IUnknown, IVssAdmin);
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
unsafe impl ::windows_core::Interface for IVssAdminEx {
    type Vtable = IVssAdminEx_Vtbl;
}
impl ::core::clone::Clone for IVssAdminEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssAdminEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7858a9f8_b1fa_41a6_964f_b9b36b8cd8d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAdminEx_Vtbl {
    pub base__: IVssAdmin_Vtbl,
    pub GetProviderCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderid: ::windows_core::GUID, plloriginalcapabilitymask: *mut u64) -> ::windows_core::HRESULT,
    pub GetProviderContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, plcontext: *mut i32) -> ::windows_core::HRESULT,
    pub SetProviderContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, lcontext: i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssAsync(::windows_core::IUnknown);
impl IVssAsync {
    pub unsafe fn Cancel(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Cancel)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Wait(&self, dwmilliseconds: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Wait)(::windows_core::Interface::as_raw(self), dwmilliseconds).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut ::windows_core::HRESULT, preserved: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QueryStatus)(::windows_core::Interface::as_raw(self), phrresult, preserved).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssAsync, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssAsync {
    type Vtable = IVssAsync_Vtbl;
}
impl ::core::clone::Clone for IVssAsync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssAsync {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x507c37b4_cf5b_4e95_b0af_14eb9767467e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssAsync_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Wait: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwmilliseconds: u32) -> ::windows_core::HRESULT,
    pub QueryStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, preserved: *mut i32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssComponent(::windows_core::IUnknown);
impl IVssComponent {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLogicalPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComponentType)(::windows_core::Interface::as_raw(self), pct).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComponentName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBackupSucceeded)(::windows_core::Interface::as_raw(self), pbsucceeded).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAlternateLocationMappingCount)(::windows_core::Interface::as_raw(self), pcmappings).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows_core::Result<IVssWMFiledesc> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAlternateLocationMapping)(::windows_core::Interface::as_raw(self), imapping, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetBackupMetadata)(::windows_core::Interface::as_raw(self), wszdata.into_param().abi()).ok()
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBackupMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddPartialFile)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilename.into_param().abi(), wszranges.into_param().abi(), wszmetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPartialFileCount)(::windows_core::Interface::as_raw(self), pcpartialfiles).ok()
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilename: *mut ::windows_core::BSTR, pbstrrange: *mut ::windows_core::BSTR, pbstrmetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPartialFile)(::windows_core::Interface::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsSelectedForRestore)(::windows_core::Interface::as_raw(self), pbselectedforrestore).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAdditionalRestores)(::windows_core::Interface::as_raw(self), pbadditionalrestores).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetNewTargetCount)(::windows_core::Interface::as_raw(self), pcnewtarget).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows_core::Result<IVssWMFiledesc> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNewTarget)(::windows_core::Interface::as_raw(self), inewtarget, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddDirectedTarget)(::windows_core::Interface::as_raw(self), wszsourcepath.into_param().abi(), wszsourcefilename.into_param().abi(), wszsourcerangelist.into_param().abi(), wszdestinationpath.into_param().abi(), wszdestinationfilename.into_param().abi(), wszdestinationrangelist.into_param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDirectedTargetCount)(::windows_core::Interface::as_raw(self), pcdirectedtarget).ok()
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut ::windows_core::BSTR, pbstrsourcefilename: *mut ::windows_core::BSTR, pbstrsourcerangelist: *mut ::windows_core::BSTR, pbstrdestinationpath: *mut ::windows_core::BSTR, pbstrdestinationfilename: *mut ::windows_core::BSTR, pbstrdestinationrangelist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDirectedTarget)(::windows_core::Interface::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRestoreMetadata)(::windows_core::Interface::as_raw(self), wszrestoremetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRestoreMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRestoreTarget)(::windows_core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRestoreTarget)(::windows_core::Interface::as_raw(self), ptarget).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPreRestoreFailureMsg)(::windows_core::Interface::as_raw(self), wszprerestorefailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPreRestoreFailureMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPostRestoreFailureMsg)(::windows_core::Interface::as_raw(self), wszpostrestorefailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPostRestoreFailureMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetBackupStamp)(::windows_core::Interface::as_raw(self), wszbackupstamp.into_param().abi()).ok()
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBackupStamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetPreviousBackupStamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetBackupOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRestoreOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRestoreSubcomponentCount)(::windows_core::Interface::as_raw(self), pcrestoresubcomponent).ok()
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut ::windows_core::BSTR, pbstrcomponentname: *mut ::windows_core::BSTR, pbrepair: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRestoreSubcomponent)(::windows_core::Interface::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), pbrepair).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFileRestoreStatus)(::windows_core::Interface::as_raw(self), pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyTime)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyLSN)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), bstrlsnstring.into_param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDifferencedFilesCount)(::windows_core::Interface::as_raw(self), pcdifferencedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilespec: *mut ::windows_core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetDifferencedFile)(::windows_core::Interface::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), pbrecursive, ::core::mem::transmute(pbstrlsnstring), pftlastmodifytime).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssComponent, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssComponent {
    type Vtable = IVssComponent_Vtbl;
}
impl ::core::clone::Clone for IVssComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssComponent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd2c72c96_c121_4518_b627_e5a93d010ead);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponent_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetLogicalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetComponentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetBackupSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsucceeded: *mut bool) -> ::windows_core::HRESULT,
    pub GetAlternateLocationMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcmappings: *mut u32) -> ::windows_core::HRESULT,
    pub GetAlternateLocationMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetBackupMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszdata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetBackupMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub AddPartialFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilename: ::windows_core::PCWSTR, wszranges: ::windows_core::PCWSTR, wszmetadata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPartialFileCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcpartialfiles: *mut u32) -> ::windows_core::HRESULT,
    pub GetPartialFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrrange: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrmetadata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub IsSelectedForRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbselectedforrestore: *mut bool) -> ::windows_core::HRESULT,
    pub GetAdditionalRestores: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbadditionalrestores: *mut bool) -> ::windows_core::HRESULT,
    pub GetNewTargetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcnewtarget: *mut u32) -> ::windows_core::HRESULT,
    pub GetNewTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddDirectedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszsourcepath: ::windows_core::PCWSTR, wszsourcefilename: ::windows_core::PCWSTR, wszsourcerangelist: ::windows_core::PCWSTR, wszdestinationpath: ::windows_core::PCWSTR, wszdestinationfilename: ::windows_core::PCWSTR, wszdestinationrangelist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetDirectedTargetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdirectedtarget: *mut u32) -> ::windows_core::HRESULT,
    pub GetDirectedTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsourcefilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsourcerangelist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdestinationpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdestinationfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdestinationrangelist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRestoreMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszrestoremetadata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRestoreMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrestoremetadata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetRestoreTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: VSS_RESTORE_TARGET) -> ::windows_core::HRESULT,
    pub GetRestoreTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_core::HRESULT,
    pub SetPreRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszprerestorefailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPreRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrprerestorefailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetPostRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpostrestorefailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPostRestoreFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpostrestorefailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub SetBackupStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszbackupstamp: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetBackupStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPreviousBackupStamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupstamp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetBackupOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrbackupoptions: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetRestoreOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrrestoreoptions: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetRestoreSubcomponentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> ::windows_core::HRESULT,
    pub GetRestoreSubcomponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrcomponentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbrepair: *mut bool) -> ::windows_core::HRESULT,
    pub GetFileRestoreStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDifferencedFilesByLastModifyTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDifferencedFilesByLastModifyTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddDifferencedFilesByLastModifyLSN: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: super::super::Foundation::BOOL, bstrlsnstring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddDifferencedFilesByLastModifyLSN: usize,
    pub GetDifferencedFilesCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcdifferencedfiles: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDifferencedFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrfilespec: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDifferencedFile: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssComponentEx(::windows_core::IUnknown);
impl IVssComponentEx {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetLogicalPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetComponentType)(::windows_core::Interface::as_raw(self), pct).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetComponentName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBackupSucceeded)(::windows_core::Interface::as_raw(self), pbsucceeded).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAlternateLocationMappingCount)(::windows_core::Interface::as_raw(self), pcmappings).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows_core::Result<IVssWMFiledesc> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAlternateLocationMapping)(::windows_core::Interface::as_raw(self), imapping, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetBackupMetadata)(::windows_core::Interface::as_raw(self), wszdata.into_param().abi()).ok()
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBackupMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.AddPartialFile)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilename.into_param().abi(), wszranges.into_param().abi(), wszmetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPartialFileCount)(::windows_core::Interface::as_raw(self), pcpartialfiles).ok()
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilename: *mut ::windows_core::BSTR, pbstrrange: *mut ::windows_core::BSTR, pbstrmetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPartialFile)(::windows_core::Interface::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.IsSelectedForRestore)(::windows_core::Interface::as_raw(self), pbselectedforrestore).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetAdditionalRestores)(::windows_core::Interface::as_raw(self), pbadditionalrestores).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetNewTargetCount)(::windows_core::Interface::as_raw(self), pcnewtarget).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows_core::Result<IVssWMFiledesc> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetNewTarget)(::windows_core::Interface::as_raw(self), inewtarget, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.AddDirectedTarget)(::windows_core::Interface::as_raw(self), wszsourcepath.into_param().abi(), wszsourcefilename.into_param().abi(), wszsourcerangelist.into_param().abi(), wszdestinationpath.into_param().abi(), wszdestinationfilename.into_param().abi(), wszdestinationrangelist.into_param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDirectedTargetCount)(::windows_core::Interface::as_raw(self), pcdirectedtarget).ok()
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut ::windows_core::BSTR, pbstrsourcefilename: *mut ::windows_core::BSTR, pbstrsourcerangelist: *mut ::windows_core::BSTR, pbstrdestinationpath: *mut ::windows_core::BSTR, pbstrdestinationfilename: *mut ::windows_core::BSTR, pbstrdestinationrangelist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDirectedTarget)(::windows_core::Interface::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetRestoreMetadata)(::windows_core::Interface::as_raw(self), wszrestoremetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRestoreMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetRestoreTarget)(::windows_core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRestoreTarget)(::windows_core::Interface::as_raw(self), ptarget).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPreRestoreFailureMsg)(::windows_core::Interface::as_raw(self), wszprerestorefailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPreRestoreFailureMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPostRestoreFailureMsg)(::windows_core::Interface::as_raw(self), wszpostrestorefailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPostRestoreFailureMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetBackupStamp)(::windows_core::Interface::as_raw(self), wszbackupstamp.into_param().abi()).ok()
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBackupStamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetPreviousBackupStamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetBackupOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRestoreOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRestoreSubcomponentCount)(::windows_core::Interface::as_raw(self), pcrestoresubcomponent).ok()
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut ::windows_core::BSTR, pbstrcomponentname: *mut ::windows_core::BSTR, pbrepair: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRestoreSubcomponent)(::windows_core::Interface::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), pbrepair).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetFileRestoreStatus)(::windows_core::Interface::as_raw(self), pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.AddDifferencedFilesByLastModifyTime)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.AddDifferencedFilesByLastModifyLSN)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), bstrlsnstring.into_param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDifferencedFilesCount)(::windows_core::Interface::as_raw(self), pcdifferencedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilespec: *mut ::windows_core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetDifferencedFile)(::windows_core::Interface::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), pbrecursive, ::core::mem::transmute(pbstrlsnstring), pftlastmodifytime).ok()
    }
    pub unsafe fn SetPrepareForBackupFailureMsg<P0>(&self, wszfailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPrepareForBackupFailureMsg)(::windows_core::Interface::as_raw(self), wszfailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn SetPostSnapshotFailureMsg<P0>(&self, wszfailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetPostSnapshotFailureMsg)(::windows_core::Interface::as_raw(self), wszfailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrepareForBackupFailureMsg)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPostSnapshotFailureMsg)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows_core::Result<bool> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAuthoritativeRestore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetRollForward)(::windows_core::Interface::as_raw(self), prolltype, ::core::mem::transmute(pbstrpoint)).ok()
    }
    pub unsafe fn GetRestoreName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRestoreName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssComponentEx, ::windows_core::IUnknown, IVssComponent);
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
unsafe impl ::windows_core::Interface for IVssComponentEx {
    type Vtable = IVssComponentEx_Vtbl;
}
impl ::core::clone::Clone for IVssComponentEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssComponentEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x156c8b5e_f131_4bd7_9c97_d1923be7e1fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx_Vtbl {
    pub base__: IVssComponent_Vtbl,
    pub SetPrepareForBackupFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetPostSnapshotFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszfailuremsg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetPrepareForBackupFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetPostSnapshotFailureMsg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfailuremsg: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetAuthoritativeRestore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbauth: *mut bool) -> ::windows_core::HRESULT,
    pub GetRollForward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetRestoreName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssComponentEx2(::windows_core::IUnknown);
impl IVssComponentEx2 {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetLogicalPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetComponentType)(::windows_core::Interface::as_raw(self), pct).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetComponentName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBackupSucceeded)(::windows_core::Interface::as_raw(self), pbsucceeded).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAlternateLocationMappingCount)(::windows_core::Interface::as_raw(self), pcmappings).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows_core::Result<IVssWMFiledesc> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetAlternateLocationMapping)(::windows_core::Interface::as_raw(self), imapping, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetBackupMetadata)(::windows_core::Interface::as_raw(self), wszdata.into_param().abi()).ok()
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBackupMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.AddPartialFile)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilename.into_param().abi(), wszranges.into_param().abi(), wszmetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPartialFileCount)(::windows_core::Interface::as_raw(self), pcpartialfiles).ok()
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilename: *mut ::windows_core::BSTR, pbstrrange: *mut ::windows_core::BSTR, pbstrmetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPartialFile)(::windows_core::Interface::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.IsSelectedForRestore)(::windows_core::Interface::as_raw(self), pbselectedforrestore).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetAdditionalRestores)(::windows_core::Interface::as_raw(self), pbadditionalrestores).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetNewTargetCount)(::windows_core::Interface::as_raw(self), pcnewtarget).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows_core::Result<IVssWMFiledesc> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetNewTarget)(::windows_core::Interface::as_raw(self), inewtarget, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P5: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.AddDirectedTarget)(::windows_core::Interface::as_raw(self), wszsourcepath.into_param().abi(), wszsourcefilename.into_param().abi(), wszsourcerangelist.into_param().abi(), wszdestinationpath.into_param().abi(), wszdestinationfilename.into_param().abi(), wszdestinationrangelist.into_param().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDirectedTargetCount)(::windows_core::Interface::as_raw(self), pcdirectedtarget).ok()
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut ::windows_core::BSTR, pbstrsourcefilename: *mut ::windows_core::BSTR, pbstrsourcerangelist: *mut ::windows_core::BSTR, pbstrdestinationpath: *mut ::windows_core::BSTR, pbstrdestinationfilename: *mut ::windows_core::BSTR, pbstrdestinationrangelist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDirectedTarget)(::windows_core::Interface::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetRestoreMetadata)(::windows_core::Interface::as_raw(self), wszrestoremetadata.into_param().abi()).ok()
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRestoreMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetRestoreTarget)(::windows_core::Interface::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRestoreTarget)(::windows_core::Interface::as_raw(self), ptarget).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPreRestoreFailureMsg)(::windows_core::Interface::as_raw(self), wszprerestorefailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPreRestoreFailureMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPostRestoreFailureMsg)(::windows_core::Interface::as_raw(self), wszpostrestorefailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPostRestoreFailureMsg)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetBackupStamp)(::windows_core::Interface::as_raw(self), wszbackupstamp.into_param().abi()).ok()
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBackupStamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetPreviousBackupStamp)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetBackupOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRestoreOptions)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRestoreSubcomponentCount)(::windows_core::Interface::as_raw(self), pcrestoresubcomponent).ok()
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut ::windows_core::BSTR, pbstrcomponentname: *mut ::windows_core::BSTR, pbrepair: *mut bool) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetRestoreSubcomponent)(::windows_core::Interface::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), pbrepair).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetFileRestoreStatus)(::windows_core::Interface::as_raw(self), pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.AddDifferencedFilesByLastModifyTime)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1, P2, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P3: ::windows_core::IntoParam<::windows_core::BSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.AddDifferencedFilesByLastModifyLSN)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive.into_param().abi(), bstrlsnstring.into_param().abi()).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDifferencedFilesCount)(::windows_core::Interface::as_raw(self), pcdifferencedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut ::windows_core::BSTR, pbstrfilespec: *mut ::windows_core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::windows_core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.GetDifferencedFile)(::windows_core::Interface::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), pbrecursive, ::core::mem::transmute(pbstrlsnstring), pftlastmodifytime).ok()
    }
    pub unsafe fn SetPrepareForBackupFailureMsg<P0>(&self, wszfailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPrepareForBackupFailureMsg)(::windows_core::Interface::as_raw(self), wszfailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn SetPostSnapshotFailureMsg<P0>(&self, wszfailuremsg: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPostSnapshotFailureMsg)(::windows_core::Interface::as_raw(self), wszfailuremsg.into_param().abi()).ok()
    }
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPrepareForBackupFailureMsg)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPostSnapshotFailureMsg)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows_core::Result<bool> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetAuthoritativeRestore)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetRollForward)(::windows_core::Interface::as_raw(self), prolltype, ::core::mem::transmute(pbstrpoint)).ok()
    }
    pub unsafe fn GetRestoreName(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetRestoreName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFailure<P0>(&self, hr: ::windows_core::HRESULT, hrapplication: ::windows_core::HRESULT, wszapplicationmessage: P0, dwreserved: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFailure)(::windows_core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.into_param().abi(), dwreserved).ok()
    }
    pub unsafe fn GetFailure(&self, phr: *mut ::windows_core::HRESULT, phrapplication: *mut ::windows_core::HRESULT, pbstrapplicationmessage: *mut ::windows_core::BSTR, pdwreserved: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetFailure)(::windows_core::Interface::as_raw(self), phr, phrapplication, ::core::mem::transmute(pbstrapplicationmessage), pdwreserved).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssComponentEx2, ::windows_core::IUnknown, IVssComponent, IVssComponentEx);
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
unsafe impl ::windows_core::Interface for IVssComponentEx2 {
    type Vtable = IVssComponentEx2_Vtbl;
}
impl ::core::clone::Clone for IVssComponentEx2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssComponentEx2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b5be0f2_07a9_4e4b_bdd3_cfdc8e2c0d2d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx2_Vtbl {
    pub base__: IVssComponentEx_Vtbl,
    pub SetFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, hrapplication: ::windows_core::HRESULT, wszapplicationmessage: ::windows_core::PCWSTR, dwreserved: u32) -> ::windows_core::HRESULT,
    pub GetFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phr: *mut ::windows_core::HRESULT, phrapplication: *mut ::windows_core::HRESULT, pbstrapplicationmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwreserved: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssCreateExpressWriterMetadata(::windows_core::IUnknown);
impl IVssCreateExpressWriterMetadata {
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddExcludeFiles)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive).ok()
    }
    pub unsafe fn AddComponent<P0, P1, P2>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P0, wszcomponentname: P1, wszcaption: P2, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddComponent)(::windows_core::Interface::as_raw(self), ct, wszlogicalpath.into_param().abi(), wszcomponentname.into_param().abi(), wszcaption.into_param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok()
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P4>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P4, dwbackuptypemask: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddFilesToFileGroup)(::windows_core::Interface::as_raw(self), wszlogicalpath.into_param().abi(), wszgroupname.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive, wszalternatelocation.into_param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn SetRestoreMethod<P0, P1>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P0, wszuserprocedure: P1, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRestoreMethod)(::windows_core::Interface::as_raw(self), method, wszservice.into_param().abi(), wszuserprocedure.into_param().abi(), writerrestore, brebootrequired).ok()
    }
    pub unsafe fn AddComponentDependency<P0, P1, P2, P3>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: ::windows_core::GUID, wszonlogicalpath: P2, wszoncomponentname: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddComponentDependency)(::windows_core::Interface::as_raw(self), wszforlogicalpath.into_param().abi(), wszforcomponentname.into_param().abi(), ::core::mem::transmute(onwriterid), wszonlogicalpath.into_param().abi(), wszoncomponentname.into_param().abi()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackupSchema)(::windows_core::Interface::as_raw(self), dwschemamask).ok()
    }
    pub unsafe fn SaveAsXML(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).SaveAsXML)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssCreateExpressWriterMetadata, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssCreateExpressWriterMetadata {
    type Vtable = IVssCreateExpressWriterMetadata_Vtbl;
}
impl ::core::clone::Clone for IVssCreateExpressWriterMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssCreateExpressWriterMetadata {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9c772e77_b26e_427f_92dd_c996f41ea5e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateExpressWriterMetadata_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddExcludeFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8) -> ::windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcaption: ::windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszgroupname: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows_core::PCWSTR, wszuserprocedure: ::windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszforlogicalpath: ::windows_core::PCWSTR, wszforcomponentname: ::windows_core::PCWSTR, onwriterid: ::windows_core::GUID, wszonlogicalpath: ::windows_core::PCWSTR, wszoncomponentname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows_core::HRESULT,
    pub SaveAsXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssCreateWriterMetadata(::std::ptr::NonNull<::std::ffi::c_void>);
impl IVssCreateWriterMetadata {
    pub unsafe fn AddIncludeFiles<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8, wszalternatelocation: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddIncludeFiles)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive, wszalternatelocation.into_param().abi()).ok()
    }
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddExcludeFiles)(::windows_core::Interface::as_raw(self), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive).ok()
    }
    pub unsafe fn AddComponent<P0, P1, P2>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P0, wszcomponentname: P1, wszcaption: P2, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddComponent)(::windows_core::Interface::as_raw(self), ct, wszlogicalpath.into_param().abi(), wszcomponentname.into_param().abi(), wszcaption.into_param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags).ok()
    }
    pub unsafe fn AddDatabaseFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddDatabaseFiles)(::windows_core::Interface::as_raw(self), wszlogicalpath.into_param().abi(), wszdatabasename.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn AddDatabaseLogFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddDatabaseLogFiles)(::windows_core::Interface::as_raw(self), wszlogicalpath.into_param().abi(), wszdatabasename.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P4>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: u8, wszalternatelocation: P4, dwbackuptypemask: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P4: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddFilesToFileGroup)(::windows_core::Interface::as_raw(self), wszlogicalpath.into_param().abi(), wszgroupname.into_param().abi(), wszpath.into_param().abi(), wszfilespec.into_param().abi(), brecursive, wszalternatelocation.into_param().abi(), dwbackuptypemask).ok()
    }
    pub unsafe fn SetRestoreMethod<P0, P1>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P0, wszuserprocedure: P1, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRestoreMethod)(::windows_core::Interface::as_raw(self), method, wszservice.into_param().abi(), wszuserprocedure.into_param().abi(), writerrestore, brebootrequired).ok()
    }
    pub unsafe fn AddAlternateLocationMapping<P0, P1, P2>(&self, wszsourcepath: P0, wszsourcefilespec: P1, brecursive: u8, wszdestination: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddAlternateLocationMapping)(::windows_core::Interface::as_raw(self), wszsourcepath.into_param().abi(), wszsourcefilespec.into_param().abi(), brecursive, wszdestination.into_param().abi()).ok()
    }
    pub unsafe fn AddComponentDependency<P0, P1, P2, P3>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: ::windows_core::GUID, wszonlogicalpath: P2, wszoncomponentname: P3) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P3: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).AddComponentDependency)(::windows_core::Interface::as_raw(self), wszforlogicalpath.into_param().abi(), wszforcomponentname.into_param().abi(), ::core::mem::transmute(onwriterid), wszonlogicalpath.into_param().abi(), wszoncomponentname.into_param().abi()).ok()
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBackupSchema)(::windows_core::Interface::as_raw(self), dwschemamask).ok()
    }
    #[doc = "*Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocument(&self) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMDocument> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocument)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SaveAsXML)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrxml)).ok()
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
unsafe impl ::windows_core::Interface for IVssCreateWriterMetadata {
    type Vtable = IVssCreateWriterMetadata_Vtbl;
}
impl ::core::clone::Clone for IVssCreateWriterMetadata {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateWriterMetadata_Vtbl {
    pub AddIncludeFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AddExcludeFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8) -> ::windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: ::windows_core::PCWSTR, wszcomponentname: ::windows_core::PCWSTR, wszcaption: ::windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: u8, bnotifyonbackupcomplete: u8, bselectable: u8, bselectableforrestore: u8, dwcomponentflags: u32) -> ::windows_core::HRESULT,
    pub AddDatabaseFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszdatabasename: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT,
    pub AddDatabaseLogFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszdatabasename: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszlogicalpath: ::windows_core::PCWSTR, wszgroupname: ::windows_core::PCWSTR, wszpath: ::windows_core::PCWSTR, wszfilespec: ::windows_core::PCWSTR, brecursive: u8, wszalternatelocation: ::windows_core::PCWSTR, dwbackuptypemask: u32) -> ::windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: ::windows_core::PCWSTR, wszuserprocedure: ::windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: u8) -> ::windows_core::HRESULT,
    pub AddAlternateLocationMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszsourcepath: ::windows_core::PCWSTR, wszsourcefilespec: ::windows_core::PCWSTR, brecursive: u8, wszdestination: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub AddComponentDependency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszforlogicalpath: ::windows_core::PCWSTR, wszforcomponentname: ::windows_core::PCWSTR, onwriterid: ::windows_core::GUID, wszonlogicalpath: ::windows_core::PCWSTR, wszoncomponentname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub SetBackupSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwschemamask: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
    pub GetDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com")))]
    GetDocument: usize,
    pub SaveAsXML: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt(::windows_core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AddDiffArea)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ChangeDiffAreaMaximumSize)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryVolumesSupportedForDiffAreas)(::windows_core::Interface::as_raw(self), pwszoriginalvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryDiffAreasForVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryDiffAreasOnVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows_core::GUID) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryDiffAreasForSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssDifferentialSoftwareSnapshotMgmt {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt_Vtbl;
}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssDifferentialSoftwareSnapshotMgmt {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x214a0f28_b737_4026_b847_4f9e37d79529);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddDiffArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::HRESULT,
    pub ChangeDiffAreaMaximumSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::HRESULT,
    pub QueryVolumesSupportedForDiffAreas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszoriginalvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QueryDiffAreasForVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QueryDiffAreasOnVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QueryDiffAreasForSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2(::windows_core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AddDiffArea)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.ChangeDiffAreaMaximumSize)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryVolumesSupportedForDiffAreas)(::windows_core::Interface::as_raw(self), pwszoriginalvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryDiffAreasForVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryDiffAreasOnVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows_core::GUID) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryDiffAreasForSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<P0>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).ChangeDiffAreaMaximumSizeEx)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace, bvolatile.into_param().abi()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MigrateDiffAreas)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, pwsznewdiffareavolumename).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows_core::Result<IVssAsync> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryMigrationStatus)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: ::windows_core::GUID, priority: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSnapshotPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idsnapshot), priority).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt2, ::windows_core::IUnknown, IVssDifferentialSoftwareSnapshotMgmt);
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
unsafe impl ::windows_core::Interface for IVssDifferentialSoftwareSnapshotMgmt2 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt2_Vtbl;
}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssDifferentialSoftwareSnapshotMgmt2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x949d7353_675f_4275_8969_f044c6277815);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt2_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangeDiffAreaMaximumSizeEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangeDiffAreaMaximumSizeEx: usize,
    pub MigrateDiffAreas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows_core::HRESULT,
    pub QueryMigrationStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetSnapshotPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idsnapshot: ::windows_core::GUID, priority: u8) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3(::windows_core::IUnknown);
impl IVssDifferentialSoftwareSnapshotMgmt3 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.AddDiffArea)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.ChangeDiffAreaMaximumSize)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QueryVolumesSupportedForDiffAreas)(::windows_core::Interface::as_raw(self), pwszoriginalvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QueryDiffAreasForVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QueryDiffAreasOnVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows_core::GUID) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.QueryDiffAreasForSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<P0>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.ChangeDiffAreaMaximumSizeEx)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace, bvolatile.into_param().abi()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.MigrateDiffAreas)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, pwsznewdiffareavolumename).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows_core::Result<IVssAsync> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.QueryMigrationStatus)(::windows_core::Interface::as_raw(self), pwszvolumename, pwszdiffareavolumename, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: ::windows_core::GUID, priority: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSnapshotPriority)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idsnapshot), priority).ok()
    }
    pub unsafe fn SetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetVolumeProtectLevel)(::windows_core::Interface::as_raw(self), pwszvolumename, protectionlevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVolumeProtectLevel(&self, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetVolumeProtectLevel)(::windows_core::Interface::as_raw(self), pwszvolumename, protectionlevel).ok()
    }
    pub unsafe fn ClearVolumeProtectFault(&self, pwszvolumename: *const u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ClearVolumeProtectFault)(::windows_core::Interface::as_raw(self), pwszvolumename).ok()
    }
    pub unsafe fn DeleteUnusedDiffAreas(&self, pwszdiffareavolumename: *const u16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DeleteUnusedDiffAreas)(::windows_core::Interface::as_raw(self), pwszdiffareavolumename).ok()
    }
    pub unsafe fn QuerySnapshotDeltaBitmap(&self, idsnapshotolder: ::windows_core::GUID, idsnapshotyounger: ::windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).QuerySnapshotDeltaBitmap)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(idsnapshotolder), ::core::mem::transmute(idsnapshotyounger), pcblocksizeperbit, pcbitmaplength, ppbbitmap).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssDifferentialSoftwareSnapshotMgmt3, ::windows_core::IUnknown, IVssDifferentialSoftwareSnapshotMgmt, IVssDifferentialSoftwareSnapshotMgmt2);
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
unsafe impl ::windows_core::Interface for IVssDifferentialSoftwareSnapshotMgmt3 {
    type Vtable = IVssDifferentialSoftwareSnapshotMgmt3_Vtbl;
}
impl ::core::clone::Clone for IVssDifferentialSoftwareSnapshotMgmt3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssDifferentialSoftwareSnapshotMgmt3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x383f7e71_a4c5_401f_b27f_f826289f8458);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssDifferentialSoftwareSnapshotMgmt3_Vtbl {
    pub base__: IVssDifferentialSoftwareSnapshotMgmt2_Vtbl,
    pub SetVolumeProtectLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: VSS_PROTECTION_LEVEL) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVolumeProtectLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, protectionlevel: *mut VSS_VOLUME_PROTECTION_INFO) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVolumeProtectLevel: usize,
    pub ClearVolumeProtectFault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16) -> ::windows_core::HRESULT,
    pub DeleteUnusedDiffAreas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszdiffareavolumename: *const u16) -> ::windows_core::HRESULT,
    pub QuerySnapshotDeltaBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idsnapshotolder: ::windows_core::GUID, idsnapshotyounger: ::windows_core::GUID, pcblocksizeperbit: *mut u32, pcbitmaplength: *mut u32, ppbbitmap: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssEnumMgmtObject(::windows_core::IUnknown);
impl IVssEnumMgmtObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_MGMT_OBJECT_PROP], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut ::core::option::Option<IVssEnumMgmtObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppenum)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssEnumMgmtObject, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssEnumMgmtObject {
    type Vtable = IVssEnumMgmtObject_Vtbl;
}
impl ::core::clone::Clone for IVssEnumMgmtObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssEnumMgmtObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x01954e6b_9254_4e6e_808c_c9e05d007696);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumMgmtObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_MGMT_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssEnumObject(::windows_core::IUnknown);
impl IVssEnumObject {
    pub unsafe fn Next(&self, rgelt: &mut [VSS_OBJECT_PROP], pceltfetched: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Next)(::windows_core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Skip)(::windows_core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Reset)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self, ppenum: *mut ::core::option::Option<IVssEnumObject>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(ppenum)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssEnumObject, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssEnumObject {
    type Vtable = IVssEnumObject_Vtbl;
}
impl ::core::clone::Clone for IVssEnumObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssEnumObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae1c7110_2f60_11d3_8a39_00c04f72d8e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssEnumObject_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut VSS_OBJECT_PROP, pceltfetched: *mut u32) -> ::windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssExpressWriter(::windows_core::IUnknown);
impl IVssExpressWriter {
    pub unsafe fn CreateMetadata<P0>(&self, writerid: ::windows_core::GUID, writername: P0, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> ::windows_core::Result<IVssCreateExpressWriterMetadata>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateMetadata)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(writerid), writername.into_param().abi(), usagetype, versionmajor, versionminor, reserved, &mut result__).from_abi(result__)
    }
    pub unsafe fn LoadMetadata<P0>(&self, metadata: P0, reserved: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LoadMetadata)(::windows_core::Interface::as_raw(self), metadata.into_param().abi(), reserved).ok()
    }
    pub unsafe fn Register(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Register)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Unregister(&self, writerid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Unregister)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(writerid)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssExpressWriter, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssExpressWriter {
    type Vtable = IVssExpressWriter_Vtbl;
}
impl ::core::clone::Clone for IVssExpressWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssExpressWriter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe33affdc_59c7_47b1_97d5_4266598f6235);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExpressWriter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreateMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writerid: ::windows_core::GUID, writername: ::windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub LoadMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, metadata: ::windows_core::PCWSTR, reserved: u32) -> ::windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, writerid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssFileShareSnapshotProvider(::windows_core::IUnknown);
impl IVssFileShareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContext)(::windows_core::Interface::as_raw(self), lcontext).ok()
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: ::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSnapshotProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), pprop).ok()
    }
    pub unsafe fn Query(&self, queriedobjectid: ::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows_core::Result<IVssEnumObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<P0>(&self, sourceobjectid: ::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: P0, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).DeleteSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into_param().abi(), pldeletedsnapshots, pnondeletedsnapshotid).ok()
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginPrepareSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), pwszsharepath, lnewcontext, ::core::mem::transmute(providerid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSupported(&self, pwszsharepath: *const u16) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsPathSupported)(::windows_core::Interface::as_raw(self), pwszsharepath, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPathSnapshotted(&self, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsPathSnapshotted)(::windows_core::Interface::as_raw(self), pwszsharepath, pbsnapshotspresent, plsnapshotcompatibility).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSnapshotProperty(&self, snapshotid: ::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSnapshotProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), esnapshotpropertyid, ::core::mem::transmute(vproperty)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssFileShareSnapshotProvider, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssFileShareSnapshotProvider {
    type Vtable = IVssFileShareSnapshotProvider_Vtbl;
}
impl ::core::clone::Clone for IVssFileShareSnapshotProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssFileShareSnapshotProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8636060_7c2e_11df_8c4a_0800200c9a66);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssFileShareSnapshotProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSnapshots: usize,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, pwszsharepath: *const u16, lnewcontext: i32, providerid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPathSnapshotted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszsharepath: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPathSnapshotted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSnapshotProperty: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssHardwareSnapshotProvider(::windows_core::IUnknown);
impl IVssHardwareSnapshotProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AreLunsSupported)(::windows_core::Interface::as_raw(self), lluncount, lcontext, rgwszdevices, pluninformation, pbissupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).FillInLunInfo)(::windows_core::Interface::as_raw(self), wszdevicename, pluninfo, pbissupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginPrepareSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), lcontext, lluncount, rgdevicenames, rgluninformation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetTargetLuns)(::windows_core::Interface::as_raw(self), lluncount, rgdevicenames, rgsourceluns, rgdestinationluns).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).LocateLuns)(::windows_core::Interface::as_raw(self), rgsourceluns.len() as _, ::core::mem::transmute(rgsourceluns.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLunEmpty)(::windows_core::Interface::as_raw(self), wszdevicename, pinformation).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssHardwareSnapshotProvider, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssHardwareSnapshotProvider {
    type Vtable = IVssHardwareSnapshotProvider_Vtbl;
}
impl ::core::clone::Clone for IVssHardwareSnapshotProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssHardwareSnapshotProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9593a157_44e9_4344_bbeb_44fbf9b06b10);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub AreLunsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    AreLunsSupported: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub FillInLunInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    FillInLunInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    BeginPrepareSnapshot: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub GetTargetLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    GetTargetLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub LocateLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lluncount: i32, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    LocateLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnLunEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnLunEmpty: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssHardwareSnapshotProviderEx(::windows_core::IUnknown);
impl IVssHardwareSnapshotProviderEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AreLunsSupported)(::windows_core::Interface::as_raw(self), lluncount, lcontext, rgwszdevices, pluninformation, pbissupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.FillInLunInfo)(::windows_core::Interface::as_raw(self), wszdevicename, pluninfo, pbissupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.BeginPrepareSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), lcontext, lluncount, rgdevicenames, rgluninformation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.GetTargetLuns)(::windows_core::Interface::as_raw(self), lluncount, rgdevicenames, rgsourceluns, rgdestinationluns).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.LocateLuns)(::windows_core::Interface::as_raw(self), rgsourceluns.len() as _, ::core::mem::transmute(rgsourceluns.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.OnLunEmpty)(::windows_core::Interface::as_raw(self), wszdevicename, pinformation).ok()
    }
    pub unsafe fn GetProviderCapabilities(&self) -> ::windows_core::Result<u64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProviderCapabilities)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunStateChange(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnLunStateChange)(::windows_core::Interface::as_raw(self), psnapshotluns, poriginalluns, dwcount, dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn ResyncLuns(&self, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_core::Result<IVssAsync> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).ResyncLuns)(::windows_core::Interface::as_raw(self), psourceluns, ptargetluns, dwcount, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnReuseLuns(&self, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).OnReuseLuns)(::windows_core::Interface::as_raw(self), psnapshotluns, poriginalluns, dwcount).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssHardwareSnapshotProviderEx, ::windows_core::IUnknown, IVssHardwareSnapshotProvider);
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
unsafe impl ::windows_core::Interface for IVssHardwareSnapshotProviderEx {
    type Vtable = IVssHardwareSnapshotProviderEx_Vtbl;
}
impl ::core::clone::Clone for IVssHardwareSnapshotProviderEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssHardwareSnapshotProviderEx {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7f5ba925_cdb1_4d11_a71f_339eb7e709fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssHardwareSnapshotProviderEx_Vtbl {
    pub base__: IVssHardwareSnapshotProvider_Vtbl,
    pub GetProviderCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plloriginalcapabilitymask: *mut u64) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnLunStateChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, dwflags: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnLunStateChange: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub ResyncLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, ptargetluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    ResyncLuns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub OnReuseLuns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psnapshotluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, poriginalluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, dwcount: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService")))]
    OnReuseLuns: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssProviderCreateSnapshotSet(::windows_core::IUnknown);
impl IVssProviderCreateSnapshotSet {
    pub unsafe fn EndPrepareSnapshots(&self, snapshotsetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EndPrepareSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PreCommitSnapshots(&self, snapshotsetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreCommitSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn CommitSnapshots(&self, snapshotsetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CommitSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PostCommitSnapshots(&self, snapshotsetid: ::windows_core::GUID, lsnapshotscount: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PostCommitSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), lsnapshotscount).ok()
    }
    pub unsafe fn PreFinalCommitSnapshots(&self, snapshotsetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PreFinalCommitSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn PostFinalCommitSnapshots(&self, snapshotsetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).PostFinalCommitSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
    pub unsafe fn AbortSnapshots(&self, snapshotsetid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AbortSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssProviderCreateSnapshotSet, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssProviderCreateSnapshotSet {
    type Vtable = IVssProviderCreateSnapshotSet_Vtbl;
}
impl ::core::clone::Clone for IVssProviderCreateSnapshotSet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssProviderCreateSnapshotSet {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5f894e5b_1e39_4778_8e23_9abad9f0e08c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderCreateSnapshotSet_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub EndPrepareSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PreCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub CommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PostCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, lsnapshotscount: i32) -> ::windows_core::HRESULT,
    pub PreFinalCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PostFinalCommitSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub AbortSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssProviderNotifications(::windows_core::IUnknown);
impl IVssProviderNotifications {
    pub unsafe fn OnLoad<P0>(&self, pcallback: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).OnLoad)(::windows_core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<P0>(&self, bforceunload: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).OnUnload)(::windows_core::Interface::as_raw(self), bforceunload.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssProviderNotifications, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssProviderNotifications {
    type Vtable = IVssProviderNotifications_Vtbl;
}
impl ::core::clone::Clone for IVssProviderNotifications {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssProviderNotifications {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe561901f_03a5_4afe_86d0_72baeece7004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssProviderNotifications_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub OnLoad: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OnUnload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnUnload: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssSnapshotMgmt(::windows_core::IUnknown);
impl IVssSnapshotMgmt {
    pub unsafe fn GetProviderMgmtInterface(&self, providerid: ::windows_core::GUID, interfaceid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProviderMgmtInterface)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(providerid), interfaceid, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryVolumesSupportedForSnapshots(&self, providerid: ::windows_core::GUID, lcontext: i32) -> ::windows_core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryVolumesSupportedForSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(providerid), lcontext, &mut result__).from_abi(result__)
    }
    pub unsafe fn QuerySnapshotsByVolume(&self, pwszvolumename: *const u16, providerid: ::windows_core::GUID) -> ::windows_core::Result<IVssEnumObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QuerySnapshotsByVolume)(::windows_core::Interface::as_raw(self), pwszvolumename, ::core::mem::transmute(providerid), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssSnapshotMgmt, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssSnapshotMgmt {
    type Vtable = IVssSnapshotMgmt_Vtbl;
}
impl ::core::clone::Clone for IVssSnapshotMgmt {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssSnapshotMgmt {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfa7df749_66e7_4986_a27f_e2f04ae53772);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetProviderMgmtInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, interfaceid: *const ::windows_core::GUID, ppitf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QueryVolumesSupportedForSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, providerid: ::windows_core::GUID, lcontext: i32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub QuerySnapshotsByVolume: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, providerid: ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssSnapshotMgmt2(::windows_core::IUnknown);
impl IVssSnapshotMgmt2 {
    pub unsafe fn GetMinDiffAreaSize(&self) -> ::windows_core::Result<i64> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMinDiffAreaSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssSnapshotMgmt2, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssSnapshotMgmt2 {
    type Vtable = IVssSnapshotMgmt2_Vtbl;
}
impl ::core::clone::Clone for IVssSnapshotMgmt2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssSnapshotMgmt2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0f61ec39_fe82_45f2_a3f0_768b5d427102);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSnapshotMgmt2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetMinDiffAreaSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pllmindiffareasize: *mut i64) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssSoftwareSnapshotProvider(::windows_core::IUnknown);
impl IVssSoftwareSnapshotProvider {
    pub unsafe fn SetContext(&self, lcontext: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContext)(::windows_core::Interface::as_raw(self), lcontext).ok()
    }
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: ::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSnapshotProperties)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), pprop).ok()
    }
    pub unsafe fn Query(&self, queriedobjectid: ::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE) -> ::windows_core::Result<IVssEnumObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Query)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSnapshots<P0>(&self, sourceobjectid: ::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: P0, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).DeleteSnapshots)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into_param().abi(), pldeletedsnapshots, pnondeletedsnapshotid).ok()
    }
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).BeginPrepareSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), pwszvolumename, lnewcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSupported(&self, pwszvolumename: *const u16) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsVolumeSupported)(::windows_core::Interface::as_raw(self), pwszvolumename, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsVolumeSnapshotted(&self, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).IsVolumeSnapshotted)(::windows_core::Interface::as_raw(self), pwszvolumename, pbsnapshotspresent, plsnapshotcompatibility).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetSnapshotProperty(&self, snapshotid: ::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSnapshotProperty)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid), esnapshotpropertyid, ::core::mem::transmute(vproperty)).ok()
    }
    pub unsafe fn RevertToSnapshot(&self, snapshotid: ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RevertToSnapshot)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(snapshotid)).ok()
    }
    pub unsafe fn QueryRevertStatus(&self, pwszvolume: *const u16) -> ::windows_core::Result<IVssAsync> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).QueryRevertStatus)(::windows_core::Interface::as_raw(self), pwszvolume, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssSoftwareSnapshotProvider, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssSoftwareSnapshotProvider {
    type Vtable = IVssSoftwareSnapshotProvider_Vtbl;
}
impl ::core::clone::Clone for IVssSoftwareSnapshotProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssSoftwareSnapshotProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x609e123e_2c5a_44d3_8f01_0b1d9a47d1ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssSoftwareSnapshotProvider_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcontext: i32) -> ::windows_core::HRESULT,
    pub GetSnapshotProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, pprop: *mut VSS_SNAPSHOT_PROP) -> ::windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queriedobjectid: ::windows_core::GUID, equeriedobjecttype: VSS_OBJECT_TYPE, ereturnedobjectstype: VSS_OBJECT_TYPE, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DeleteSnapshots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceobjectid: ::windows_core::GUID, esourceobjecttype: VSS_OBJECT_TYPE, bforcedelete: super::super::Foundation::BOOL, pldeletedsnapshots: *mut i32, pnondeletedsnapshotid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DeleteSnapshots: usize,
    pub BeginPrepareSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotsetid: ::windows_core::GUID, snapshotid: ::windows_core::GUID, pwszvolumename: *const u16, lnewcontext: i32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolumeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsupportedbythisprovider: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolumeSupported: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsVolumeSnapshotted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolumename: *const u16, pbsnapshotspresent: *mut super::super::Foundation::BOOL, plsnapshotcompatibility: *mut i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsVolumeSnapshotted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetSnapshotProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID, esnapshotpropertyid: VSS_SNAPSHOT_PROPERTY_ID, vproperty: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetSnapshotProperty: usize,
    pub RevertToSnapshot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapshotid: ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub QueryRevertStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszvolume: *const u16, ppasync: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWMDependency(::windows_core::IUnknown);
impl IVssWMDependency {
    pub unsafe fn GetWriterId(&self, pwriterid: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWriterId)(::windows_core::Interface::as_raw(self), pwriterid).ok()
    }
    pub unsafe fn GetLogicalPath(&self, pbstrlogicalpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetLogicalPath)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrlogicalpath)).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrcomponentname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComponentName)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(pbstrcomponentname)).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IVssWMDependency, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssWMDependency {
    type Vtable = IVssWMDependency_Vtbl;
}
impl ::core::clone::Clone for IVssWMDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssWMDependency {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMDependency_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetWriterId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwriterid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetLogicalPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrlogicalpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcomponentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWMFiledesc(::windows_core::IUnknown);
impl IVssWMFiledesc {
    pub unsafe fn GetPath(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFilespec(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFilespec)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRecursive(&self) -> ::windows_core::Result<bool> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRecursive)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAlternateLocation(&self) -> ::windows_core::Result<::windows_core::BSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAlternateLocation)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBackupTypeMask(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBackupTypeMask)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IVssWMFiledesc, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IVssWMFiledesc {
    type Vtable = IVssWMFiledesc_Vtbl;
}
impl ::core::clone::Clone for IVssWMFiledesc {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVssWMFiledesc {
    const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMFiledesc_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetFilespec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfilespec: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetRecursive: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrecursive: *mut bool) -> ::windows_core::HRESULT,
    pub GetAlternateLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstralternatelocation: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT,
    pub GetBackupTypeMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwtypemask: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
pub struct IVssWriterComponents(::std::ptr::NonNull<::std::ffi::c_void>);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(&self, pccomponents: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetComponentCount)(::windows_core::Interface::as_raw(self), pccomponents).ok()
    }
    pub unsafe fn GetWriterInfo(&self, pidinstance: *mut ::windows_core::GUID, pidwriter: *mut ::windows_core::GUID) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetWriterInfo)(::windows_core::Interface::as_raw(self), pidinstance, pidwriter).ok()
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> ::windows_core::Result<IVssComponent> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetComponent)(::windows_core::Interface::as_raw(self), icomponent, &mut result__).from_abi(result__)
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
unsafe impl ::windows_core::Interface for IVssWriterComponents {
    type Vtable = IVssWriterComponents_Vtbl;
}
impl ::core::clone::Clone for IVssWriterComponents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterComponents_Vtbl {
    pub GetComponentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccomponents: *mut u32) -> ::windows_core::HRESULT,
    pub GetWriterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidinstance: *mut ::windows_core::GUID, pidwriter: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSSCoordinator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe579ab5f_1cc4_44b4_bed9_de0991ff0623);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_AUTO: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(-1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_BACK_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_FRONT_END: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_SYSTEM_RM: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_APP_UNKNOWN: VSS_APPLICATION_LEVEL = VSS_APPLICATION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_NO_MAX_SPACE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ASSOC_REMOVE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = VSS_ALTERNATE_WRITER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MAKE_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_ALL: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BREAKEX_FLAG_REVERT_IDENTITY_NONE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_AUTHORITATIVE_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16384i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_COPY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_EXCLUSIVE_INCREMENTAL_DIFFERENTIAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INCREMENTAL: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_INDEPENDENT_SYSTEM_STATE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LAST_MODIFY: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(64i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LOG: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_LSN: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(128i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_RESTORE_RENAME: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(8192i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_ROLLFORWARD_RESTORE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(4096i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_TIMESTAMPED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_UNDEFINED: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_NEW_TARGET: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_PARALLEL_RESTORES: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(32768i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BS_WRITER_SUPPORTS_RESTORE_WITH_MOVE: VSS_BACKUP_SCHEMA = VSS_BACKUP_SCHEMA(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_COPY: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_DIFFERENTIAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_FULL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_INCREMENTAL: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_LOG: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_OTHER: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_BT_UNDEFINED: VSS_BACKUP_TYPE = VSS_BACKUP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = VSS_COMPONENT_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_ALL: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(-1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_APP_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(29i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_CLIENT_ACCESSIBLE_WRITERS: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_FILE_SHARE_BACKUP: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CTX_NAS_ROLLBACK: VSS_SNAPSHOT_CONTEXT = VSS_SNAPSHOT_CONTEXT(25i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = VSS_COMPONENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISKS_TOO_SMALL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212280i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_CRITICAL_DISK_CANNOT_BE_EXCLUDED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212267i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DATADISK_RDISK0: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212282i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_ASSIGNMENT_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212287i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DISK_RECREATION_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212286i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_DYNAMIC_VHD_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212278i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_FIXED_PHYSICAL_DISK_AVAILABLE_AFTER_DISK_EXCLUSION: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212268i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_MISSING_DYNDISK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212284i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_ARCPATH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212285i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_NO_PHYSICAL_DISK_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212269i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK0_TOOSMALL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212281i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_RDISK_FOR_SYSTEM_DISK_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212270i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SHARED_CRIDISK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212283i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_ASRERROR_SYSTEM_PARTITION_HIDDEN: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212266i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_AUTORECOVERY_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212293i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BAD_STATE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212543i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_BREAK_REVERT_ID_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212298i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CANNOT_REVERT_DISKID: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212290i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212288i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CLUSTER_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212498i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CORRUPT_XML_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212528i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_CRITICAL_VOLUME_ON_INVALID_DISK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212271i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_DYNAMIC_DISK_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212292i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FLUSH_WRITES_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212525i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_FSS_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212265i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_HOLD_WRITES_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212524i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INSUFFICIENT_STORAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212513i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_INVALID_XML_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212527i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_LEGACY_PROVIDER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212297i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_DIFFAREA_ASSOCIATIONS_REACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212514i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_REMOTE_MACHINES_REACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212510i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_SNAPSHOTS_REACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212521i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MAXIMUM_NUMBER_OF_VOLUMES_REACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212526i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_DISK: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212296i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_HIDDEN_VOLUME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212295i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_MISSING_VOLUME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212294i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NESTED_VOLUME_LIMIT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212500i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NONTRANSPORTABLE_BCD: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212291i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212497i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_NO_SNAPSHOTS_IMPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_ALREADY_EXISTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212531i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_OBJECT_NOT_FOUND: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212536i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_ALREADY_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212541i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212537i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_NOT_REGISTERED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212540i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_PROVIDER_VETO: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212538i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REBOOT_REQUIRED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212505i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNAVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212509i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REMOTE_SERVER_UNSUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212508i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_RESYNC_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212289i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212507i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_REVERT_VOLUME_LOST: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212506i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_NOT_IN_SET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212501i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SNAPSHOT_SET_IN_PROGRESS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212522i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212511i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_FREEZE_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212504i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_TRANSACTION_THAW_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212503i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212542i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_PROVIDER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212529i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNEXPECTED_WRITER_ERROR: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212523i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSELECTED_VOLUME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212502i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_UNSUPPORTED_CONTEXT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212517i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_IN_USE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212515i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_LOCAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212499i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212532i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_VOLUME_NOT_SUPPORTED_BY_PROVIDER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212530i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_INCONSISTENTSNAPSHOT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212304i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_NONRETRYABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212300i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_OUTOFRESOURCES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212303i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_PARTIAL_FAILURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212490i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RECOVERY_FAILED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212299i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_RETRYABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212301i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITERERROR_TIMEOUT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212302i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_ALREADY_SUBSCRIBED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212518i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_INFRASTRUCTURE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212520i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_NOT_RESPONDING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212519i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_E_WRITER_STATUS_NOT_AVAILABLE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2147212279i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_ALL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(3840i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_CREATED_DURING_BACKUP: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(65536i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_DIFFERENTIAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_FULL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_INCREMENTAL_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_BACKUP_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_FSBT_LOG_SNAPSHOT_REQUIRED: VSS_FILE_SPEC_BACKUP_TYPE = VSS_FILE_SPEC_BACKUP_TYPE(2048i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_AREA: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_DIFF_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_UNKNOWN: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_MGMT_OBJECT_VOLUME: VSS_MGMT_OBJECT_TYPE = VSS_MGMT_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_NONE: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_PROVIDER: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_SNAPSHOT_SET: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_TYPE_COUNT: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_OBJECT_UNKNOWN: VSS_OBJECT_TYPE = VSS_OBJECT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_DO_MASK_LUNS: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(2048i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_POST_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(1024i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_LUN_PRE_RECOVERY: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ONLUNSTATECHANGE_NOTIFY_READ_WRITE: VSS_HARDWARE_OPTIONS = VSS_HARDWARE_OPTIONS(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_READ_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_COW_WRITE_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DESTROY_ALL_SNAPSHOTS: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_FULL: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_MISSING: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_DIFF_AREA_REMOVED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(14i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_EXTERNAL_WRITER_TO_DIFF_AREA: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_FILE_SYSTEM_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_FAILED: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_GROW_TOO_SLOW: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_IO_FAILURE_DURING_ONLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MAPPED_MEMORY_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MEMORY_ALLOCATION_FAILURE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_META_DATA_CORRUPTION: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_MOUNT_DURING_CLUSTER_OFFLINE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_FAULT_NONE: VSS_PROTECTION_FAULT = VSS_PROTECTION_FAULT(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_ORIGINAL_VOLUME: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROTECTION_LEVEL_SNAPSHOT: VSS_PROTECTION_LEVEL = VSS_PROTECTION_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_FILESHARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_HARDWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SOFTWARE: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_SYSTEM: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PROV_UNKNOWN: VSS_PROVIDER_TYPE = VSS_PROVIDER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_CLUSTERED: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_COMPLIANT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_DIFFERENTIAL: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LEGACY: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_REPOINT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_LUN_RESYNC: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_MULTIPLE_IMPORT: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(32i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_OFFLINE_CREATION: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_PLEX: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(128i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_PRV_CAPABILITY_RECYCLING: VSS_PROVIDER_CAPABILITIES = VSS_PROVIDER_CAPABILITIES(64i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_NO_VOLUME_CHECK: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(512i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RECOVERY_REVERT_IDENTITY_ALL: VSS_RECOVERY_OPTIONS = VSS_RECOVERY_OPTIONS(256i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_ALL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_NONE: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_PARTIAL: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RF_UNDEFINED: VSS_ROLLFORWARD_TYPE = VSS_ROLLFORWARD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = VSS_RESTOREMETHOD_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = VSS_FILE_RESTORE_STATUS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_BY_COPY: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_IMPORT: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_OTHER: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RTYPE_UNDEFINED: VSS_RESTORE_TYPE = VSS_RESTORE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = VSS_RESTORE_TARGET(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_CONTENTINDEX: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SC_DISABLE_DEFRAG: VSS_SNAPSHOT_COMPATIBILITY = VSS_SNAPSHOT_COMPATIBILITY(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(-1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = VSS_SUBSCRIBE_MASK(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_CREATION_TIMESTAMP: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_NAME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_EXPOSED_PATH: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINAL_VOLUME: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_ORIGINATING_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_PROVIDER_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SERVICE_MACHINE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOTS_COUNT: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ATTRIBUTES: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_DEVICE: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_SNAPSHOT_SET_ID: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_STATUS: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SPROPID_UNKNOWN: VSS_SNAPSHOT_PROPERTY_ID = VSS_SNAPSHOT_PROPERTY_ID(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_ABORTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_COUNT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_CREATED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_DELETED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(14i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_POSTCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PRECOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREFINALCOMMITTED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARED: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PREPARING: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_COMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_POSTFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PRECOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREFINALCOMMIT: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_PROCESSING_PREPARE: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_SS_UNKNOWN: VSS_SNAPSHOT_STATE = VSS_SNAPSHOT_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = VSS_SOURCE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_CANCELLED: ::windows_core::HRESULT = ::windows_core::HRESULT(271115i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_FINISHED: ::windows_core::HRESULT = ::windows_core::HRESULT(271114i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_ASYNC_PENDING: ::windows_core::HRESULT = ::windows_core::HRESULT(271113i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_S_SOME_SNAPSHOTS_NOT_IMPORTED: ::windows_core::HRESULT = ::windows_core::HRESULT(271137i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = VSS_USAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = VSS_USAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = VSS_USAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = VSS_USAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_AUTORECOVER: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4194304i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_CLIENT_ACCESSIBLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DELAYED_POSTSNAPSHOT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16777216i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_DIFFERENTIAL: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(131072i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_LOCALLY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1048576i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_EXPOSED_REMOTELY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2097152i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_FILE_SHARE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(67108864i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_HARDWARE_ASSISTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(65536i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_IMPORTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(524288i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_SURFACED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(64i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NOT_TRANSACTED: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(128i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTORECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_AUTO_RELEASE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_NO_WRITERS: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PERSISTENT: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_PLEX: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(262144i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_ROLLBACK_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(8388608i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TRANSPORTABLE: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(32i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_VOLSNAP_ATTR_TXF_RECOVERY: VSS_VOLUME_SNAPSHOT_ATTRIBUTES = VSS_VOLUME_SNAPSHOT_ATTRIBUTES(33554432i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = VSS_WRITERRESTORE_ENUM(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_COUNT: VSS_WRITER_STATE = VSS_WRITER_STATE(16i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUPSHUTDOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(15i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(12i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_IDENTIFY: VSS_WRITER_STATE = VSS_WRITER_STATE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(14i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(11i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_BACKUP: VSS_WRITER_STATE = VSS_WRITER_STATE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PREPARE_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_PRE_RESTORE: VSS_WRITER_STATE = VSS_WRITER_STATE(13i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_FAILED_AT_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(10i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_STABLE: VSS_WRITER_STATE = VSS_WRITER_STATE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_UNKNOWN: VSS_WRITER_STATE = VSS_WRITER_STATE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_BACKUP_COMPLETE: VSS_WRITER_STATE = VSS_WRITER_STATE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_FREEZE: VSS_WRITER_STATE = VSS_WRITER_STATE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_POST_SNAPSHOT: VSS_WRITER_STATE = VSS_WRITER_STATE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VSS_WS_WAITING_FOR_THAW: VSS_WRITER_STATE = VSS_WRITER_STATE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub const VssSnapshotMgmt: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0b5a2c52_3eb9_470a_96e2_6c6d4570e40f);
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_ALTERNATE_WRITER_STATE(pub i32);
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
impl ::windows_core::TypeKind for VSS_ALTERNATE_WRITER_STATE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_APPLICATION_LEVEL {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_APPLICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_APPLICATION_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_BACKUP_SCHEMA(pub i32);
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
impl ::windows_core::TypeKind for VSS_BACKUP_SCHEMA {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_BACKUP_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_COMPONENT_FLAGS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_COMPONENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_COMPONENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_FILE_RESTORE_STATUS(pub i32);
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
impl ::windows_core::TypeKind for VSS_FILE_RESTORE_STATUS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_FILE_SPEC_BACKUP_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_HARDWARE_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_HARDWARE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_HARDWARE_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_MGMT_OBJECT_TYPE(pub i32);
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
impl ::windows_core::TypeKind for VSS_MGMT_OBJECT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_MGMT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_MGMT_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_OBJECT_TYPE(pub i32);
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
impl ::windows_core::TypeKind for VSS_OBJECT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_PROTECTION_FAULT(pub i32);
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
impl ::windows_core::TypeKind for VSS_PROTECTION_FAULT {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_PROTECTION_LEVEL {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_PROVIDER_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_PROVIDER_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_CAPABILITIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_PROVIDER_TYPE(pub i32);
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
impl ::windows_core::TypeKind for VSS_PROVIDER_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_RECOVERY_OPTIONS {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_RESTOREMETHOD_ENUM {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_RESTORE_TARGET {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_RESTORE_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_ROLLFORWARD_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_SNAPSHOT_COMPATIBILITY {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_SNAPSHOT_CONTEXT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_CONTEXT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_SNAPSHOT_PROPERTY_ID(pub i32);
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
impl ::windows_core::TypeKind for VSS_SNAPSHOT_PROPERTY_ID {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_SNAPSHOT_STATE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_SOURCE_TYPE {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_SUBSCRIBE_MASK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_SUBSCRIBE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SUBSCRIBE_MASK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_USAGE_TYPE(pub i32);
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
impl ::windows_core::TypeKind for VSS_USAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_USAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_USAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VSS_VOLUME_SNAPSHOT_ATTRIBUTES(pub i32);
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
impl ::windows_core::TypeKind for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_WRITERRESTORE_ENUM {
    type TypeKind = ::windows_core::CopyType;
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
impl ::windows_core::TypeKind for VSS_WRITER_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for VSS_WRITER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_WRITER_STATE").field(&self.0).finish()
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
impl ::windows_core::TypeKind for VSS_DIFF_AREA_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VSS_DIFF_AREA_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszDiffAreaVolumeName == other.m_pwszDiffAreaVolumeName && self.m_llMaximumDiffSpace == other.m_llMaximumDiffSpace && self.m_llAllocatedDiffSpace == other.m_llAllocatedDiffSpace && self.m_llUsedDiffSpace == other.m_llUsedDiffSpace
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
impl ::windows_core::TypeKind for VSS_DIFF_VOLUME_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VSS_DIFF_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName && self.m_llVolumeFreeSpace == other.m_llVolumeFreeSpace && self.m_llVolumeTotalSpace == other.m_llVolumeTotalSpace
    }
}
impl ::core::cmp::Eq for VSS_DIFF_VOLUME_PROP {}
impl ::core::default::Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows_core::TypeKind for VSS_MGMT_OBJECT_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows_core::TypeKind for VSS_MGMT_OBJECT_UNION {
    type TypeKind = ::windows_core::CopyType;
}
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
impl ::windows_core::TypeKind for VSS_OBJECT_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows_core::TypeKind for VSS_OBJECT_UNION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_PROVIDER_PROP {
    pub m_ProviderId: ::windows_core::GUID,
    pub m_pwszProviderName: *mut u16,
    pub m_eProviderType: VSS_PROVIDER_TYPE,
    pub m_pwszProviderVersion: *mut u16,
    pub m_ProviderVersionId: ::windows_core::GUID,
    pub m_ClassId: ::windows_core::GUID,
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
impl ::windows_core::TypeKind for VSS_PROVIDER_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VSS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_ProviderId == other.m_ProviderId && self.m_pwszProviderName == other.m_pwszProviderName && self.m_eProviderType == other.m_eProviderType && self.m_pwszProviderVersion == other.m_pwszProviderVersion && self.m_ProviderVersionId == other.m_ProviderVersionId && self.m_ClassId == other.m_ClassId
    }
}
impl ::core::cmp::Eq for VSS_PROVIDER_PROP {}
impl ::core::default::Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Vss\"`*"]
pub struct VSS_SNAPSHOT_PROP {
    pub m_SnapshotId: ::windows_core::GUID,
    pub m_SnapshotSetId: ::windows_core::GUID,
    pub m_lSnapshotsCount: i32,
    pub m_pwszSnapshotDeviceObject: *mut u16,
    pub m_pwszOriginalVolumeName: *mut u16,
    pub m_pwszOriginatingMachine: *mut u16,
    pub m_pwszServiceMachine: *mut u16,
    pub m_pwszExposedName: *mut u16,
    pub m_pwszExposedPath: *mut u16,
    pub m_ProviderId: ::windows_core::GUID,
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
impl ::windows_core::TypeKind for VSS_SNAPSHOT_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VSS_SNAPSHOT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_SnapshotId == other.m_SnapshotId && self.m_SnapshotSetId == other.m_SnapshotSetId && self.m_lSnapshotsCount == other.m_lSnapshotsCount && self.m_pwszSnapshotDeviceObject == other.m_pwszSnapshotDeviceObject && self.m_pwszOriginalVolumeName == other.m_pwszOriginalVolumeName && self.m_pwszOriginatingMachine == other.m_pwszOriginatingMachine && self.m_pwszServiceMachine == other.m_pwszServiceMachine && self.m_pwszExposedName == other.m_pwszExposedName && self.m_pwszExposedPath == other.m_pwszExposedPath && self.m_ProviderId == other.m_ProviderId && self.m_lSnapshotAttributes == other.m_lSnapshotAttributes && self.m_tsCreationTimestamp == other.m_tsCreationTimestamp && self.m_eStatus == other.m_eStatus
    }
}
impl ::core::cmp::Eq for VSS_SNAPSHOT_PROP {}
impl ::core::default::Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows_core::TypeKind for VSS_VOLUME_PROP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VSS_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName
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
impl ::windows_core::TypeKind for VSS_VOLUME_PROTECTION_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VSS_VOLUME_PROTECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.m_protectionLevel == other.m_protectionLevel && self.m_volumeIsOfflineForProtection == other.m_volumeIsOfflineForProtection && self.m_protectionFault == other.m_protectionFault && self.m_failureStatus == other.m_failureStatus && self.m_volumeHasUnusedDiffArea == other.m_volumeHasUnusedDiffArea && self.m_reserved == other.m_reserved
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
#[cfg(feature = "implement")]
::core::include!("impl.rs");
