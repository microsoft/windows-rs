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
impl IVssAdminEx {
    pub unsafe fn RegisterProvider(&self, pproviderid: ::windows::core::GUID, classid: ::windows::core::GUID, pwszprovidername: *const u16, eprovidertype: VSS_PROVIDER_TYPE, pwszproviderversion: *const u16, providerversionid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RegisterProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pproviderid), ::core::mem::transmute(classid), pwszprovidername, eprovidertype, pwszproviderversion, ::core::mem::transmute(providerversionid)).ok()
    }
    pub unsafe fn UnregisterProvider(&self, providerid: ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterProvider)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(providerid)).ok()
    }
    pub unsafe fn QueryProviders(&self) -> ::windows::core::Result<IVssEnumObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryProviders)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AbortAllSnapshotsInProgress(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AbortAllSnapshotsInProgress)(::windows::core::Vtable::as_raw(self)).ok()
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
impl IVssComponentEx {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLogicalPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetComponentType)(::windows::core::Vtable::as_raw(self), pct).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetComponentName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBackupSucceeded)(::windows::core::Vtable::as_raw(self), pbsucceeded).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAlternateLocationMappingCount)(::windows::core::Vtable::as_raw(self), pcmappings).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAlternateLocationMapping)(::windows::core::Vtable::as_raw(self), imapping, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBackupMetadata)(::windows::core::Vtable::as_raw(self), wszdata.into().abi()).ok()
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBackupMetadata)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPartialFile)(::windows::core::Vtable::as_raw(self), wszpath.into().abi(), wszfilename.into().abi(), wszranges.into().abi(), wszmetadata.into().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPartialFileCount)(::windows::core::Vtable::as_raw(self), pcpartialfiles).ok()
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut ::windows::core::BSTR, pbstrfilename: *mut ::windows::core::BSTR, pbstrrange: *mut ::windows::core::BSTR, pbstrmetadata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPartialFile)(::windows::core::Vtable::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsSelectedForRestore)(::windows::core::Vtable::as_raw(self), pbselectedforrestore).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAdditionalRestores)(::windows::core::Vtable::as_raw(self), pbadditionalrestores).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNewTargetCount)(::windows::core::Vtable::as_raw(self), pcnewtarget).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNewTarget)(::windows::core::Vtable::as_raw(self), inewtarget, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddDirectedTarget)(::windows::core::Vtable::as_raw(self), wszsourcepath.into().abi(), wszsourcefilename.into().abi(), wszsourcerangelist.into().abi(), wszdestinationpath.into().abi(), wszdestinationfilename.into().abi(), wszdestinationrangelist.into().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDirectedTargetCount)(::windows::core::Vtable::as_raw(self), pcdirectedtarget).ok()
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut ::windows::core::BSTR, pbstrsourcefilename: *mut ::windows::core::BSTR, pbstrsourcerangelist: *mut ::windows::core::BSTR, pbstrdestinationpath: *mut ::windows::core::BSTR, pbstrdestinationfilename: *mut ::windows::core::BSTR, pbstrdestinationrangelist: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDirectedTarget)(::windows::core::Vtable::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRestoreMetadata)(::windows::core::Vtable::as_raw(self), wszrestoremetadata.into().abi()).ok()
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRestoreMetadata)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRestoreTarget)(::windows::core::Vtable::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRestoreTarget)(::windows::core::Vtable::as_raw(self), ptarget).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPreRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), wszprerestorefailuremsg.into().abi()).ok()
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPreRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPostRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), wszpostrestorefailuremsg.into().abi()).ok()
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPostRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBackupStamp)(::windows::core::Vtable::as_raw(self), wszbackupstamp.into().abi()).ok()
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBackupStamp)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPreviousBackupStamp)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetBackupOptions)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRestoreOptions)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRestoreSubcomponentCount)(::windows::core::Vtable::as_raw(self), pcrestoresubcomponent).ok()
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut ::windows::core::BSTR, pbstrcomponentname: *mut ::windows::core::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRestoreSubcomponent)(::windows::core::Vtable::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), pbrepair).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFileRestoreStatus)(::windows::core::Vtable::as_raw(self), pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddDifferencedFilesByLastModifyTime)(::windows::core::Vtable::as_raw(self), wszpath.into().abi(), wszfilespec.into().abi(), brecursive.into(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddDifferencedFilesByLastModifyLSN)(::windows::core::Vtable::as_raw(self), wszpath.into().abi(), wszfilespec.into().abi(), brecursive.into(), ::core::mem::transmute_copy(bstrlsnstring)).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDifferencedFilesCount)(::windows::core::Vtable::as_raw(self), pcdifferencedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut ::windows::core::BSTR, pbstrfilespec: *mut ::windows::core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::windows::core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDifferencedFile)(::windows::core::Vtable::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), pbrecursive, ::core::mem::transmute(pbstrlsnstring), pftlastmodifytime).ok()
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
impl IVssComponentEx2 {
    pub unsafe fn GetLogicalPath(&self, pbstrpath: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetLogicalPath)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrpath)).ok()
    }
    pub unsafe fn GetComponentType(&self, pct: *mut VSS_COMPONENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentType)(::windows::core::Vtable::as_raw(self), pct).ok()
    }
    pub unsafe fn GetComponentName(&self, pbstrname: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrname)).ok()
    }
    pub unsafe fn GetBackupSucceeded(&self, pbsucceeded: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBackupSucceeded)(::windows::core::Vtable::as_raw(self), pbsucceeded).ok()
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self, pcmappings: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAlternateLocationMappingCount)(::windows::core::Vtable::as_raw(self), pcmappings).ok()
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAlternateLocationMapping)(::windows::core::Vtable::as_raw(self), imapping, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBackupMetadata)(::windows::core::Vtable::as_raw(self), wszdata.into().abi()).ok()
    }
    pub unsafe fn GetBackupMetadata(&self, pbstrdata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBackupMetadata)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrdata)).ok()
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddPartialFile)(::windows::core::Vtable::as_raw(self), wszpath.into().abi(), wszfilename.into().abi(), wszranges.into().abi(), wszmetadata.into().abi()).ok()
    }
    pub unsafe fn GetPartialFileCount(&self, pcpartialfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartialFileCount)(::windows::core::Vtable::as_raw(self), pcpartialfiles).ok()
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut ::windows::core::BSTR, pbstrfilename: *mut ::windows::core::BSTR, pbstrrange: *mut ::windows::core::BSTR, pbstrmetadata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartialFile)(::windows::core::Vtable::as_raw(self), ipartialfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilename), ::core::mem::transmute(pbstrrange), ::core::mem::transmute(pbstrmetadata)).ok()
    }
    pub unsafe fn IsSelectedForRestore(&self, pbselectedforrestore: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IsSelectedForRestore)(::windows::core::Vtable::as_raw(self), pbselectedforrestore).ok()
    }
    pub unsafe fn GetAdditionalRestores(&self, pbadditionalrestores: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAdditionalRestores)(::windows::core::Vtable::as_raw(self), pbadditionalrestores).ok()
    }
    pub unsafe fn GetNewTargetCount(&self, pcnewtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetNewTargetCount)(::windows::core::Vtable::as_raw(self), pcnewtarget).ok()
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> ::windows::core::Result<IVssWMFiledesc> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetNewTarget)(::windows::core::Vtable::as_raw(self), inewtarget, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddDirectedTarget)(::windows::core::Vtable::as_raw(self), wszsourcepath.into().abi(), wszsourcefilename.into().abi(), wszsourcerangelist.into().abi(), wszdestinationpath.into().abi(), wszdestinationfilename.into().abi(), wszdestinationrangelist.into().abi()).ok()
    }
    pub unsafe fn GetDirectedTargetCount(&self, pcdirectedtarget: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDirectedTargetCount)(::windows::core::Vtable::as_raw(self), pcdirectedtarget).ok()
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut ::windows::core::BSTR, pbstrsourcefilename: *mut ::windows::core::BSTR, pbstrsourcerangelist: *mut ::windows::core::BSTR, pbstrdestinationpath: *mut ::windows::core::BSTR, pbstrdestinationfilename: *mut ::windows::core::BSTR, pbstrdestinationrangelist: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDirectedTarget)(::windows::core::Vtable::as_raw(self), idirectedtarget, ::core::mem::transmute(pbstrsourcepath), ::core::mem::transmute(pbstrsourcefilename), ::core::mem::transmute(pbstrsourcerangelist), ::core::mem::transmute(pbstrdestinationpath), ::core::mem::transmute(pbstrdestinationfilename), ::core::mem::transmute(pbstrdestinationrangelist)).ok()
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRestoreMetadata)(::windows::core::Vtable::as_raw(self), wszrestoremetadata.into().abi()).ok()
    }
    pub unsafe fn GetRestoreMetadata(&self, pbstrrestoremetadata: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRestoreMetadata)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrrestoremetadata)).ok()
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRestoreTarget)(::windows::core::Vtable::as_raw(self), target).ok()
    }
    pub unsafe fn GetRestoreTarget(&self, ptarget: *mut VSS_RESTORE_TARGET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRestoreTarget)(::windows::core::Vtable::as_raw(self), ptarget).ok()
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPreRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), wszprerestorefailuremsg.into().abi()).ok()
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self, pbstrprerestorefailuremsg: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPreRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrprerestorefailuremsg)).ok()
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPostRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), wszpostrestorefailuremsg.into().abi()).ok()
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self, pbstrpostrestorefailuremsg: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPostRestoreFailureMsg)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrpostrestorefailuremsg)).ok()
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBackupStamp)(::windows::core::Vtable::as_raw(self), wszbackupstamp.into().abi()).ok()
    }
    pub unsafe fn GetBackupStamp(&self, pbstrbackupstamp: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBackupStamp)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetPreviousBackupStamp(&self, pbstrbackupstamp: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPreviousBackupStamp)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbackupstamp)).ok()
    }
    pub unsafe fn GetBackupOptions(&self, pbstrbackupoptions: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetBackupOptions)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrbackupoptions)).ok()
    }
    pub unsafe fn GetRestoreOptions(&self, pbstrrestoreoptions: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRestoreOptions)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pbstrrestoreoptions)).ok()
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self, pcrestoresubcomponent: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRestoreSubcomponentCount)(::windows::core::Vtable::as_raw(self), pcrestoresubcomponent).ok()
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut ::windows::core::BSTR, pbstrcomponentname: *mut ::windows::core::BSTR, pbrepair: *mut bool) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRestoreSubcomponent)(::windows::core::Vtable::as_raw(self), icomponent, ::core::mem::transmute(pbstrlogicalpath), ::core::mem::transmute(pbstrcomponentname), pbrepair).ok()
    }
    pub unsafe fn GetFileRestoreStatus(&self, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFileRestoreStatus)(::windows::core::Vtable::as_raw(self), pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, ftlastmodifytime: super::super::Foundation::FILETIME) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddDifferencedFilesByLastModifyTime)(::windows::core::Vtable::as_raw(self), wszpath.into().abi(), wszfilespec.into().abi(), brecursive.into(), ::core::mem::transmute(ftlastmodifytime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1, P2>(&self, wszpath: P0, wszfilespec: P1, brecursive: P2, bstrlsnstring: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddDifferencedFilesByLastModifyLSN)(::windows::core::Vtable::as_raw(self), wszpath.into().abi(), wszfilespec.into().abi(), brecursive.into(), ::core::mem::transmute_copy(bstrlsnstring)).ok()
    }
    pub unsafe fn GetDifferencedFilesCount(&self, pcdifferencedfiles: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDifferencedFilesCount)(::windows::core::Vtable::as_raw(self), pcdifferencedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut ::windows::core::BSTR, pbstrfilespec: *mut ::windows::core::BSTR, pbrecursive: *mut super::super::Foundation::BOOL, pbstrlsnstring: *mut ::windows::core::BSTR, pftlastmodifytime: *mut super::super::Foundation::FILETIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDifferencedFile)(::windows::core::Vtable::as_raw(self), idifferencedfile, ::core::mem::transmute(pbstrpath), ::core::mem::transmute(pbstrfilespec), pbrecursive, ::core::mem::transmute(pbstrlsnstring), pftlastmodifytime).ok()
    }
    pub unsafe fn SetPrepareForBackupFailureMsg<P0>(&self, wszfailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrepareForBackupFailureMsg)(::windows::core::Vtable::as_raw(self), wszfailuremsg.into().abi()).ok()
    }
    pub unsafe fn SetPostSnapshotFailureMsg<P0>(&self, wszfailuremsg: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPostSnapshotFailureMsg)(::windows::core::Vtable::as_raw(self), wszfailuremsg.into().abi()).ok()
    }
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPrepareForBackupFailureMsg)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPostSnapshotFailureMsg)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> ::windows::core::Result<bool> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAuthoritativeRestore)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRollForward(&self, prolltype: *mut VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut ::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRollForward)(::windows::core::Vtable::as_raw(self), prolltype, ::core::mem::transmute(pbstrpoint)).ok()
    }
    pub unsafe fn GetRestoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRestoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
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
impl IVssDifferentialSoftwareSnapshotMgmt2 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddDiffArea)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ChangeDiffAreaMaximumSize)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryVolumesSupportedForDiffAreas)(::windows::core::Vtable::as_raw(self), pwszoriginalvolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryDiffAreasForVolume)(::windows::core::Vtable::as_raw(self), pwszvolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryDiffAreasOnVolume)(::windows::core::Vtable::as_raw(self), pwszvolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryDiffAreasForSnapshot)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(snapshotid), result__.as_mut_ptr()).from_abi(result__)
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
impl IVssDifferentialSoftwareSnapshotMgmt3 {
    pub unsafe fn AddDiffArea(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddDiffArea)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn ChangeDiffAreaMaximumSize(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ChangeDiffAreaMaximumSize)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace).ok()
    }
    pub unsafe fn QueryVolumesSupportedForDiffAreas(&self, pwszoriginalvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QueryVolumesSupportedForDiffAreas)(::windows::core::Vtable::as_raw(self), pwszoriginalvolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QueryDiffAreasForVolume)(::windows::core::Vtable::as_raw(self), pwszvolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasOnVolume(&self, pwszvolumename: *const u16) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QueryDiffAreasOnVolume)(::windows::core::Vtable::as_raw(self), pwszvolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn QueryDiffAreasForSnapshot(&self, snapshotid: ::windows::core::GUID) -> ::windows::core::Result<IVssEnumMgmtObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.QueryDiffAreasForSnapshot)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(snapshotid), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangeDiffAreaMaximumSizeEx<P0>(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, llmaximumdiffspace: i64, bvolatile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.ChangeDiffAreaMaximumSizeEx)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, llmaximumdiffspace, bvolatile.into()).ok()
    }
    pub unsafe fn MigrateDiffAreas(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16, pwsznewdiffareavolumename: *const u16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MigrateDiffAreas)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, pwsznewdiffareavolumename).ok()
    }
    pub unsafe fn QueryMigrationStatus(&self, pwszvolumename: *const u16, pwszdiffareavolumename: *const u16) -> ::windows::core::Result<IVssAsync> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.QueryMigrationStatus)(::windows::core::Vtable::as_raw(self), pwszvolumename, pwszdiffareavolumename, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSnapshotPriority(&self, idsnapshot: ::windows::core::GUID, priority: u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSnapshotPriority)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(idsnapshot), priority).ok()
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
impl IVssHardwareSnapshotProviderEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn AreLunsSupported(&self, lluncount: i32, lcontext: i32, rgwszdevices: *const *const u16, pluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AreLunsSupported)(::windows::core::Vtable::as_raw(self), lluncount, lcontext, rgwszdevices, pluninformation, pbissupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn FillInLunInfo(&self, wszdevicename: *const u16, pluninfo: *mut super::VirtualDiskService::VDS_LUN_INFORMATION, pbissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.FillInLunInfo)(::windows::core::Vtable::as_raw(self), wszdevicename, pluninfo, pbissupported).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn BeginPrepareSnapshot(&self, snapshotsetid: ::windows::core::GUID, snapshotid: ::windows::core::GUID, lcontext: i32, lluncount: i32, rgdevicenames: *const *const u16, rgluninformation: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginPrepareSnapshot)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(snapshotsetid), ::core::mem::transmute(snapshotid), lcontext, lluncount, rgdevicenames, rgluninformation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn GetTargetLuns(&self, lluncount: i32, rgdevicenames: *const *const u16, rgsourceluns: *const super::VirtualDiskService::VDS_LUN_INFORMATION, rgdestinationluns: *mut super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTargetLuns)(::windows::core::Vtable::as_raw(self), lluncount, rgdevicenames, rgsourceluns, rgdestinationluns).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn LocateLuns(&self, rgsourceluns: &[super::VirtualDiskService::VDS_LUN_INFORMATION]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LocateLuns)(::windows::core::Vtable::as_raw(self), rgsourceluns.len() as _, ::core::mem::transmute(rgsourceluns.as_ptr())).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_VirtualDiskService\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_VirtualDiskService"))]
    pub unsafe fn OnLunEmpty(&self, wszdevicename: *const u16, pinformation: *const super::VirtualDiskService::VDS_LUN_INFORMATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OnLunEmpty)(::windows::core::Vtable::as_raw(self), wszdevicename, pinformation).ok()
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
impl ::core::default::Default for VSS_ALTERNATE_WRITER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_ALTERNATE_WRITER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_ALTERNATE_WRITER_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_APPLICATION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_APPLICATION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_APPLICATION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_BACKUP_SCHEMA {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_BACKUP_SCHEMA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_BACKUP_SCHEMA").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_BACKUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_BACKUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_BACKUP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_COMPONENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_COMPONENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_COMPONENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_COMPONENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_COMPONENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_DIFF_AREA_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_DIFF_AREA_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszDiffAreaVolumeName == other.m_pwszDiffAreaVolumeName && self.m_llMaximumDiffSpace == other.m_llMaximumDiffSpace && self.m_llAllocatedDiffSpace == other.m_llAllocatedDiffSpace && self.m_llUsedDiffSpace == other.m_llUsedDiffSpace
    }
}
impl ::core::cmp::Eq for VSS_DIFF_AREA_PROP {}
impl ::core::fmt::Debug for VSS_DIFF_AREA_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_DIFF_AREA_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszDiffAreaVolumeName", &self.m_pwszDiffAreaVolumeName).field("m_llMaximumDiffSpace", &self.m_llMaximumDiffSpace).field("m_llAllocatedDiffSpace", &self.m_llAllocatedDiffSpace).field("m_llUsedDiffSpace", &self.m_llUsedDiffSpace).finish()
    }
}
impl ::core::default::Default for VSS_DIFF_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_DIFF_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName && self.m_llVolumeFreeSpace == other.m_llVolumeFreeSpace && self.m_llVolumeTotalSpace == other.m_llVolumeTotalSpace
    }
}
impl ::core::cmp::Eq for VSS_DIFF_VOLUME_PROP {}
impl ::core::fmt::Debug for VSS_DIFF_VOLUME_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_DIFF_VOLUME_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName).field("m_llVolumeFreeSpace", &self.m_llVolumeFreeSpace).field("m_llVolumeTotalSpace", &self.m_llVolumeTotalSpace).finish()
    }
}
impl ::core::default::Default for VSS_FILE_RESTORE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_FILE_RESTORE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_FILE_RESTORE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_FILE_SPEC_BACKUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_FILE_SPEC_BACKUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_FILE_SPEC_BACKUP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_HARDWARE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_HARDWARE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_HARDWARE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_MGMT_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VSS_MGMT_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_MGMT_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_MGMT_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_MGMT_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VSS_OBJECT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VSS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_OBJECT_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VSS_PROTECTION_FAULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_PROTECTION_FAULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROTECTION_FAULT").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_PROVIDER_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_PROVIDER_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_ProviderId == other.m_ProviderId && self.m_pwszProviderName == other.m_pwszProviderName && self.m_eProviderType == other.m_eProviderType && self.m_pwszProviderVersion == other.m_pwszProviderVersion && self.m_ProviderVersionId == other.m_ProviderVersionId && self.m_ClassId == other.m_ClassId
    }
}
impl ::core::cmp::Eq for VSS_PROVIDER_PROP {}
impl ::core::fmt::Debug for VSS_PROVIDER_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_PROVIDER_PROP").field("m_ProviderId", &self.m_ProviderId).field("m_pwszProviderName", &self.m_pwszProviderName).field("m_eProviderType", &self.m_eProviderType).field("m_pwszProviderVersion", &self.m_pwszProviderVersion).field("m_ProviderVersionId", &self.m_ProviderVersionId).field("m_ClassId", &self.m_ClassId).finish()
    }
}
impl ::core::default::Default for VSS_PROVIDER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_PROVIDER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_PROVIDER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_RECOVERY_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_RECOVERY_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RECOVERY_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_RESTOREMETHOD_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_RESTOREMETHOD_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RESTOREMETHOD_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_RESTORE_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_RESTORE_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RESTORE_TARGET").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_RESTORE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_RESTORE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_RESTORE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_ROLLFORWARD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_ROLLFORWARD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_ROLLFORWARD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_COMPATIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_COMPATIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_COMPATIBILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_CONTEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_CONTEXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_SNAPSHOT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_SnapshotId == other.m_SnapshotId && self.m_SnapshotSetId == other.m_SnapshotSetId && self.m_lSnapshotsCount == other.m_lSnapshotsCount && self.m_pwszSnapshotDeviceObject == other.m_pwszSnapshotDeviceObject && self.m_pwszOriginalVolumeName == other.m_pwszOriginalVolumeName && self.m_pwszOriginatingMachine == other.m_pwszOriginatingMachine && self.m_pwszServiceMachine == other.m_pwszServiceMachine && self.m_pwszExposedName == other.m_pwszExposedName && self.m_pwszExposedPath == other.m_pwszExposedPath && self.m_ProviderId == other.m_ProviderId && self.m_lSnapshotAttributes == other.m_lSnapshotAttributes && self.m_tsCreationTimestamp == other.m_tsCreationTimestamp && self.m_eStatus == other.m_eStatus
    }
}
impl ::core::cmp::Eq for VSS_SNAPSHOT_PROP {}
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
impl ::core::default::Default for VSS_SNAPSHOT_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_SNAPSHOT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_SNAPSHOT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SNAPSHOT_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_SOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_SOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_SUBSCRIBE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_SUBSCRIBE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_SUBSCRIBE_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_USAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_USAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_USAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_VOLUME_PROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VSS_VOLUME_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.m_pwszVolumeName == other.m_pwszVolumeName && self.m_pwszVolumeDisplayName == other.m_pwszVolumeDisplayName
    }
}
impl ::core::cmp::Eq for VSS_VOLUME_PROP {}
impl ::core::fmt::Debug for VSS_VOLUME_PROP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_VOLUME_PROP").field("m_pwszVolumeName", &self.m_pwszVolumeName).field("m_pwszVolumeDisplayName", &self.m_pwszVolumeDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VSS_VOLUME_PROTECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::core::fmt::Debug for VSS_VOLUME_PROTECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VSS_VOLUME_PROTECTION_INFO").field("m_protectionLevel", &self.m_protectionLevel).field("m_volumeIsOfflineForProtection", &self.m_volumeIsOfflineForProtection).field("m_protectionFault", &self.m_protectionFault).field("m_failureStatus", &self.m_failureStatus).field("m_volumeHasUnusedDiffArea", &self.m_volumeHasUnusedDiffArea).field("m_reserved", &self.m_reserved).finish()
    }
}
impl ::core::default::Default for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_VOLUME_SNAPSHOT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_VOLUME_SNAPSHOT_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_WRITERRESTORE_ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_WRITERRESTORE_ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_WRITERRESTORE_ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for VSS_WRITER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VSS_WRITER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VSS_WRITER_STATE").field(&self.0).finish()
    }
}
