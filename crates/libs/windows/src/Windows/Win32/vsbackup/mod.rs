#[inline]
pub unsafe fn CreateVssBackupComponentsInternal() -> windows_core::Result<IVssBackupComponents> {
    windows_core::link!("vssapi.dll" "system" fn CreateVssBackupComponentsInternal(ppbackup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateVssBackupComponentsInternal(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateVssExamineWriterMetadataInternal(bstrxml: &windows_core::BSTR) -> windows_core::Result<IVssExamineWriterMetadata> {
    windows_core::link!("vssapi.dll" "system" fn CreateVssExamineWriterMetadataInternal(bstrxml : *mut core::ffi::c_void, ppmetadata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateVssExamineWriterMetadataInternal(core::mem::transmute_copy(bstrxml), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_vss")]
#[inline]
pub unsafe fn GetProviderMgmtInterfaceInternal(providerid: super::vss::VSS_ID, interfaceid: windows_core::GUID) -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("vssapi.dll" "system" fn GetProviderMgmtInterfaceInternal(providerid : super::vss::VSS_ID, interfaceid : windows_core::GUID, ppitf : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetProviderMgmtInterfaceInternal(core::mem::transmute(providerid), core::mem::transmute(interfaceid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn IsVolumeSnapshottedInternal<P0>(pwszvolumename: P0, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcapability: *mut i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("vssapi.dll" "system" fn IsVolumeSnapshottedInternal(pwszvolumename : windows_core::PCWSTR, pbsnapshotspresent : *mut windows_core::BOOL, plsnapshotcapability : *mut i32) -> windows_core::HRESULT);
    unsafe { IsVolumeSnapshottedInternal(pwszvolumename.param().abi(), pbsnapshotspresent as _, plsnapshotcapability as _) }
}
#[inline]
pub unsafe fn ShouldBlockRevertInternal<P0>(wszvolumename: P0) -> windows_core::Result<bool>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("vssapi.dll" "system" fn ShouldBlockRevertInternal(wszvolumename : windows_core::PCWSTR, pbblock : *mut bool) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ShouldBlockRevertInternal(wszvolumename.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_vss")]
#[inline]
pub unsafe fn VssFreeSnapshotPropertiesInternal(pprop: *const super::vss::VSS_SNAPSHOT_PROP) {
    windows_core::link!("vssapi.dll" "system" fn VssFreeSnapshotPropertiesInternal(pprop : *const super::vss::VSS_SNAPSHOT_PROP));
    unsafe { VssFreeSnapshotPropertiesInternal(pprop) }
}
windows_core::imp::define_interface!(IVssBackupComponents, IVssBackupComponents_Vtbl, 0x665c1d5f_c218_414d_a05d_7fef5f9d5c86);
windows_core::imp::interface_hierarchy!(IVssBackupComponents, windows_core::IUnknown);
impl IVssBackupComponents {
    pub unsafe fn GetWriterComponentsCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterComponentsCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetWriterComponents(&self, iwriter: u32) -> windows_core::Result<IVssWriterComponentsExt> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterComponents)(windows_core::Interface::as_raw(self), iwriter, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn InitializeForBackup(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeForBackup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn SetBackupState(&self, bselectcomponents: bool, bbackupbootablesystemstate: bool, backuptype: super::vss::VSS_BACKUP_TYPE, bpartialfilesupport: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackupState)(windows_core::Interface::as_raw(self), bselectcomponents, bbackupbootablesystemstate, backuptype, bpartialfilesupport) }
    }
    pub unsafe fn InitializeForRestore(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeForRestore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn SetRestoreState(&self, restoretype: super::vss::VSS_RESTORE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreState)(windows_core::Interface::as_raw(self), restoretype) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GatherWriterMetadata(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GatherWriterMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriterMetadataCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterMetadataCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetWriterMetadata(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, ppmetadata: *mut Option<IVssExamineWriterMetadata>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterMetadata)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, core::mem::transmute(ppmetadata)) }
    }
    pub unsafe fn FreeWriterMetadata(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeWriterMetadata)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn AddComponent<P3, P4>(&self, instanceid: super::vss::VSS_ID, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P3, wszcomponentname: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), core::mem::transmute(instanceid), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi()) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn PrepareForBackup(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrepareForBackup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AbortBackup(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbortBackup)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GatherWriterStatus(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GatherWriterStatus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWriterStatusCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterStatusCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeWriterStatus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeWriterStatus)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetWriterStatus(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::vss::VSS_WRITER_STATE, phresultfailure: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterStatus)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwriter), pnstatus as _, phresultfailure as _) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetBackupSucceeded<P3, P4>(&self, instanceid: super::vss::VSS_ID, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P3, wszcomponentname: P4, bsucceded: bool) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupSucceeded)(windows_core::Interface::as_raw(self), core::mem::transmute(instanceid), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bsucceded) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetBackupOptions<P2, P3, P4>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszbackupoptions: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszbackupoptions.param().abi()) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetSelectedForRestore<P2, P3>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, bselectedforrestore: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedForRestore)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bselectedforrestore) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetRestoreOptions<P2, P3, P4>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszrestoreoptions: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreOptions)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszrestoreoptions.param().abi()) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetAdditionalRestores<P2, P3>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, badditionalrestores: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAdditionalRestores)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), badditionalrestores) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetPreviousBackupStamp<P2, P3, P4>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszpreviousbackupstamp: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreviousBackupStamp)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszpreviousbackupstamp.param().abi()) }
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrxml)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn BackupComplete(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BackupComplete)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn AddAlternativeLocationMapping<P2, P3, P4, P5, P7>(&self, writerid: super::vss::VSS_ID, componenttype: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszpath: P4, wszfilespec: P5, brecursive: bool, wszdestination: P7) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAlternativeLocationMapping)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), componenttype, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszdestination.param().abi()) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn AddRestoreSubcomponent<P2, P3, P4, P5>(&self, writerid: super::vss::VSS_ID, componenttype: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszsubcomponentlogicalpath: P4, wszsubcomponentname: P5, brepair: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRestoreSubcomponent)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), componenttype, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszsubcomponentlogicalpath.param().abi(), wszsubcomponentname.param().abi(), brepair) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetFileRestoreStatus<P2, P3>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, status: super::vswriter::VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFileRestoreStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), status) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn AddNewTarget<P2, P3, P4, P5, P7>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszpath: P4, wszfilename: P5, brecursive: bool, wszalternatepath: P7) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNewTarget)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszpath.param().abi(), wszfilename.param().abi(), brecursive, wszalternatepath.param().abi()) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetRangesFilePath<P2, P3, P5>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, ipartialfile: u32, wszrangesfile: P5) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRangesFilePath)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), ipartialfile, wszrangesfile.param().abi()) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn PreRestore(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreRestore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn PostRestore(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostRestore)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetContext(&self, lcontext: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), lcontext) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn StartSnapshotSet(&self) -> windows_core::Result<super::vss::VSS_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartSnapshotSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn AddToSnapshotSet<P0>(&self, pwszvolumename: P0, providerid: super::vss::VSS_ID) -> windows_core::Result<super::vss::VSS_ID>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddToSnapshotSet)(windows_core::Interface::as_raw(self), pwszvolumename.param().abi(), core::mem::transmute(providerid), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn DoSnapshotSet(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoSnapshotSet)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn DeleteSnapshots(&self, sourceobjectid: super::vss::VSS_ID, esourceobjecttype: super::vss::VSS_OBJECT_TYPE, bforcedelete: bool, pldeletedsnapshots: *const i32, pnondeletedsnapshotid: *const super::vss::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(sourceobjectid), esourceobjecttype, bforcedelete.into(), pldeletedsnapshots, pnondeletedsnapshotid) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn ImportSnapshots(&self) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportSnapshots)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn BreakSnapshotSet(&self, snapshotsetid: super::vss::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BreakSnapshotSet)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: super::vss::VSS_ID, pprop: *mut super::vss::VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSnapshotProperties)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), pprop as _) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn Query(&self, queriedobjectid: super::vss::VSS_ID, equeriedobjecttype: super::vss::VSS_OBJECT_TYPE, ereturnedobjectstype: super::vss::VSS_OBJECT_TYPE, ppenum: *const Option<super::vss::IVssEnumObject>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), core::mem::transmute(queriedobjectid), equeriedobjecttype, ereturnedobjectstype, core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn IsVolumeSupported<P1>(&self, providerid: super::vss::VSS_ID, pwszvolumename: P1, pbsupportedbythisprovider: *const windows_core::BOOL) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsVolumeSupported)(windows_core::Interface::as_raw(self), core::mem::transmute(providerid), pwszvolumename.param().abi(), pbsupportedbythisprovider) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn DisableWriterClasses(&self, rgwriterclassid: *const super::vss::VSS_ID, cclassid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableWriterClasses)(windows_core::Interface::as_raw(self), rgwriterclassid, cclassid) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn EnableWriterClasses(&self, rgwriterclassid: *const super::vss::VSS_ID, cclassid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableWriterClasses)(windows_core::Interface::as_raw(self), rgwriterclassid, cclassid) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn DisableWriterInstances(&self, rgwriterinstanceid: *const super::vss::VSS_ID, cinstanceid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableWriterInstances)(windows_core::Interface::as_raw(self), rgwriterinstanceid, cinstanceid) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn ExposeSnapshot<P1, P3>(&self, snapshotid: super::vss::VSS_ID, wszpathfromroot: P1, lattributes: i32, wszexpose: P3) -> windows_core::Result<super::vss::VSS_PWSZ>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExposeSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), wszpathfromroot.param().abi(), lattributes, wszexpose.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn RevertToSnapshot(&self, snapshotid: super::vss::VSS_ID, bforcedismount: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevertToSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), bforcedismount.into()) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn QueryRevertStatus<P0>(&self, pwszvolume: P0) -> windows_core::Result<super::vss::IVssAsync>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryRevertStatus)(windows_core::Interface::as_raw(self), pwszvolume.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetWriterComponentsCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vswriter")]
    pub GetWriterComponents: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetWriterComponents: usize,
    pub InitializeForBackup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub SetBackupState: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, super::vss::VSS_BACKUP_TYPE, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    SetBackupState: usize,
    pub InitializeForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub SetRestoreState: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_RESTORE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    SetRestoreState: usize,
    #[cfg(feature = "Win32_vss")]
    pub GatherWriterMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GatherWriterMetadata: usize,
    pub GetWriterMetadataCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GetWriterMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::vss::VSS_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetWriterMetadata: usize,
    pub FreeWriterMetadata: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    AddComponent: usize,
    #[cfg(feature = "Win32_vss")]
    pub PrepareForBackup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    PrepareForBackup: usize,
    pub AbortBackup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GatherWriterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GatherWriterStatus: usize,
    pub GetWriterStatusCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FreeWriterStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GetWriterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::vss::VSS_ID, *mut super::vss::VSS_ID, *mut *mut core::ffi::c_void, *mut super::vss::VSS_WRITER_STATE, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetWriterStatus: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetBackupSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetBackupSucceeded: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetBackupOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetBackupOptions: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetSelectedForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetSelectedForRestore: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetRestoreOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetRestoreOptions: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetAdditionalRestores: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetAdditionalRestores: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetPreviousBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetPreviousBackupStamp: usize,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub BackupComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    BackupComplete: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub AddAlternativeLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    AddAlternativeLocationMapping: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub AddRestoreSubcomponent: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    AddRestoreSubcomponent: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetFileRestoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, super::vswriter::VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetFileRestoreStatus: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub AddNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    AddNewTarget: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetRangesFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetRangesFilePath: usize,
    #[cfg(feature = "Win32_vss")]
    pub PreRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    PreRestore: usize,
    #[cfg(feature = "Win32_vss")]
    pub PostRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    PostRestore: usize,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub StartSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    StartSnapshotSet: usize,
    #[cfg(feature = "Win32_vss")]
    pub AddToSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::vss::VSS_ID, *mut super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    AddToSnapshotSet: usize,
    #[cfg(feature = "Win32_vss")]
    pub DoSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    DoSnapshotSet: usize,
    #[cfg(feature = "Win32_vss")]
    pub DeleteSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vss::VSS_OBJECT_TYPE, windows_core::BOOL, *const i32, *const super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    DeleteSnapshots: usize,
    #[cfg(feature = "Win32_vss")]
    pub ImportSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    ImportSnapshots: usize,
    #[cfg(feature = "Win32_vss")]
    pub BreakSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    BreakSnapshotSet: usize,
    #[cfg(feature = "Win32_vss")]
    pub GetSnapshotProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, *mut super::vss::VSS_SNAPSHOT_PROP) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetSnapshotProperties: usize,
    #[cfg(feature = "Win32_vss")]
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vss::VSS_OBJECT_TYPE, super::vss::VSS_OBJECT_TYPE, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    Query: usize,
    #[cfg(feature = "Win32_vss")]
    pub IsVolumeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, windows_core::PCWSTR, *const windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    IsVolumeSupported: usize,
    #[cfg(feature = "Win32_vss")]
    pub DisableWriterClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::vss::VSS_ID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    DisableWriterClasses: usize,
    #[cfg(feature = "Win32_vss")]
    pub EnableWriterClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::vss::VSS_ID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    EnableWriterClasses: usize,
    #[cfg(feature = "Win32_vss")]
    pub DisableWriterInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::vss::VSS_ID, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    DisableWriterInstances: usize,
    #[cfg(feature = "Win32_vss")]
    pub ExposeSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, windows_core::PCWSTR, i32, windows_core::PCWSTR, *mut super::vss::VSS_PWSZ) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    ExposeSnapshot: usize,
    #[cfg(feature = "Win32_vss")]
    pub RevertToSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    RevertToSnapshot: usize,
    #[cfg(feature = "Win32_vss")]
    pub QueryRevertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    QueryRevertStatus: usize,
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssBackupComponents_Impl: windows_core::IUnknownImpl {
    fn GetWriterComponentsCount(&self) -> windows_core::Result<u32>;
    fn GetWriterComponents(&self, iwriter: u32) -> windows_core::Result<IVssWriterComponentsExt>;
    fn InitializeForBackup(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBackupState(&self, bselectcomponents: bool, bbackupbootablesystemstate: bool, backuptype: super::vss::VSS_BACKUP_TYPE, bpartialfilesupport: bool) -> windows_core::Result<()>;
    fn InitializeForRestore(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreState(&self, restoretype: super::vss::VSS_RESTORE_TYPE) -> windows_core::Result<()>;
    fn GatherWriterMetadata(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn GetWriterMetadataCount(&self) -> windows_core::Result<u32>;
    fn GetWriterMetadata(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, ppmetadata: windows_core::OutRef<IVssExamineWriterMetadata>) -> windows_core::Result<()>;
    fn FreeWriterMetadata(&self) -> windows_core::Result<()>;
    fn AddComponent(&self, instanceid: &super::vss::VSS_ID, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PrepareForBackup(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn AbortBackup(&self) -> windows_core::Result<()>;
    fn GatherWriterStatus(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn GetWriterStatusCount(&self) -> windows_core::Result<u32>;
    fn FreeWriterStatus(&self) -> windows_core::Result<()>;
    fn GetWriterStatus(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::vss::VSS_WRITER_STATE, phresultfailure: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn SetBackupSucceeded(&self, instanceid: &super::vss::VSS_ID, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bsucceded: bool) -> windows_core::Result<()>;
    fn SetBackupOptions(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszbackupoptions: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSelectedForRestore(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bselectedforrestore: bool) -> windows_core::Result<()>;
    fn SetRestoreOptions(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszrestoreoptions: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAdditionalRestores(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, badditionalrestores: bool) -> windows_core::Result<()>;
    fn SetPreviousBackupStamp(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszpreviousbackupstamp: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::Result<()>;
    fn BackupComplete(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn AddAlternativeLocationMapping(&self, writerid: &super::vss::VSS_ID, componenttype: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool, wszdestination: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddRestoreSubcomponent(&self, writerid: &super::vss::VSS_ID, componenttype: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszsubcomponentlogicalpath: &windows_core::PCWSTR, wszsubcomponentname: &windows_core::PCWSTR, brepair: bool) -> windows_core::Result<()>;
    fn SetFileRestoreStatus(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, status: super::vswriter::VSS_FILE_RESTORE_STATUS) -> windows_core::Result<()>;
    fn AddNewTarget(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilename: &windows_core::PCWSTR, brecursive: bool, wszalternatepath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRangesFilePath(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, ipartialfile: u32, wszrangesfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PreRestore(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn PostRestore(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn SetContext(&self, lcontext: i32) -> windows_core::Result<()>;
    fn StartSnapshotSet(&self) -> windows_core::Result<super::vss::VSS_ID>;
    fn AddToSnapshotSet(&self, pwszvolumename: &windows_core::PCWSTR, providerid: &super::vss::VSS_ID) -> windows_core::Result<super::vss::VSS_ID>;
    fn DoSnapshotSet(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn DeleteSnapshots(&self, sourceobjectid: &super::vss::VSS_ID, esourceobjecttype: super::vss::VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *const i32, pnondeletedsnapshotid: *const super::vss::VSS_ID) -> windows_core::Result<()>;
    fn ImportSnapshots(&self) -> windows_core::Result<super::vss::IVssAsync>;
    fn BreakSnapshotSet(&self, snapshotsetid: &super::vss::VSS_ID) -> windows_core::Result<()>;
    fn GetSnapshotProperties(&self, snapshotid: &super::vss::VSS_ID, pprop: *mut super::vss::VSS_SNAPSHOT_PROP) -> windows_core::Result<()>;
    fn Query(&self, queriedobjectid: &super::vss::VSS_ID, equeriedobjecttype: super::vss::VSS_OBJECT_TYPE, ereturnedobjectstype: super::vss::VSS_OBJECT_TYPE, ppenum: *const Option<super::vss::IVssEnumObject>) -> windows_core::Result<()>;
    fn IsVolumeSupported(&self, providerid: &super::vss::VSS_ID, pwszvolumename: &windows_core::PCWSTR, pbsupportedbythisprovider: *const windows_core::BOOL) -> windows_core::Result<()>;
    fn DisableWriterClasses(&self, rgwriterclassid: *const super::vss::VSS_ID, cclassid: u32) -> windows_core::Result<()>;
    fn EnableWriterClasses(&self, rgwriterclassid: *const super::vss::VSS_ID, cclassid: u32) -> windows_core::Result<()>;
    fn DisableWriterInstances(&self, rgwriterinstanceid: *const super::vss::VSS_ID, cinstanceid: u32) -> windows_core::Result<()>;
    fn ExposeSnapshot(&self, snapshotid: &super::vss::VSS_ID, wszpathfromroot: &windows_core::PCWSTR, lattributes: i32, wszexpose: &windows_core::PCWSTR) -> windows_core::Result<super::vss::VSS_PWSZ>;
    fn RevertToSnapshot(&self, snapshotid: &super::vss::VSS_ID, bforcedismount: windows_core::BOOL) -> windows_core::Result<()>;
    fn QueryRevertStatus(&self, pwszvolume: &windows_core::PCWSTR) -> windows_core::Result<super::vss::IVssAsync>;
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssBackupComponents_Vtbl {
    pub const fn new<Identity: IVssBackupComponents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterComponentsCount<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccomponents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterComponentsCount(this) {
                    Ok(ok__) => {
                        pccomponents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterComponents<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: u32, ppwriter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterComponents(this, core::mem::transmute_copy(&iwriter)) {
                    Ok(ok__) => {
                        ppwriter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeForBackup<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::InitializeForBackup(this, core::mem::transmute(&bstrxml)).into()
            }
        }
        unsafe extern "system" fn SetBackupState<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bselectcomponents: bool, bbackupbootablesystemstate: bool, backuptype: super::vss::VSS_BACKUP_TYPE, bpartialfilesupport: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupState(this, core::mem::transmute_copy(&bselectcomponents), core::mem::transmute_copy(&bbackupbootablesystemstate), core::mem::transmute_copy(&backuptype), core::mem::transmute_copy(&bpartialfilesupport)).into()
            }
        }
        unsafe extern "system" fn InitializeForRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::InitializeForRestore(this, core::mem::transmute(&bstrxml)).into()
            }
        }
        unsafe extern "system" fn SetRestoreState<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restoretype: super::vss::VSS_RESTORE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRestoreState(this, core::mem::transmute_copy(&restoretype)).into()
            }
        }
        unsafe extern "system" fn GatherWriterMetadata<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GatherWriterMetadata(this) {
                    Ok(ok__) => {
                        pasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterMetadataCount<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwriters: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterMetadataCount(this) {
                    Ok(ok__) => {
                        pcwriters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterMetadata<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterMetadata(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&ppmetadata)).into()
            }
        }
        unsafe extern "system" fn FreeWriterMetadata<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::FreeWriterMetadata(this).into()
            }
        }
        unsafe extern "system" fn AddComponent<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instanceid: super::vss::VSS_ID, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddComponent(this, core::mem::transmute(&instanceid), core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname)).into()
            }
        }
        unsafe extern "system" fn PrepareForBackup<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::PrepareForBackup(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AbortBackup<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AbortBackup(this).into()
            }
        }
        unsafe extern "system" fn GatherWriterStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GatherWriterStatus(this) {
                    Ok(ok__) => {
                        pasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterStatusCount<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwriters: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::GetWriterStatusCount(this) {
                    Ok(ok__) => {
                        pcwriters.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeWriterStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::FreeWriterStatus(this).into()
            }
        }
        unsafe extern "system" fn GetWriterStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwriter: *mut *mut core::ffi::c_void, pnstatus: *mut super::vss::VSS_WRITER_STATE, phresultfailure: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterStatus(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwriter), core::mem::transmute_copy(&pnstatus), core::mem::transmute_copy(&phresultfailure)).into()
            }
        }
        unsafe extern "system" fn SetBackupSucceeded<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instanceid: super::vss::VSS_ID, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bsucceded: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupSucceeded(this, core::mem::transmute(&instanceid), core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bsucceded)).into()
            }
        }
        unsafe extern "system" fn SetBackupOptions<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszbackupoptions: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupOptions(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszbackupoptions)).into()
            }
        }
        unsafe extern "system" fn SetSelectedForRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bselectedforrestore: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetSelectedForRestore(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bselectedforrestore)).into()
            }
        }
        unsafe extern "system" fn SetRestoreOptions<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszrestoreoptions: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRestoreOptions(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszrestoreoptions)).into()
            }
        }
        unsafe extern "system" fn SetAdditionalRestores<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, badditionalrestores: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetAdditionalRestores(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&badditionalrestores)).into()
            }
        }
        unsafe extern "system" fn SetPreviousBackupStamp<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszpreviousbackupstamp: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetPreviousBackupStamp(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszpreviousbackupstamp)).into()
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SaveAsXML(this, core::mem::transmute_copy(&pbstrxml)).into()
            }
        }
        unsafe extern "system" fn BackupComplete<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::BackupComplete(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddAlternativeLocationMapping<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, componenttype: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool, wszdestination: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddAlternativeLocationMapping(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&componenttype), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszdestination)).into()
            }
        }
        unsafe extern "system" fn AddRestoreSubcomponent<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, componenttype: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszsubcomponentlogicalpath: windows_core::PCWSTR, wszsubcomponentname: windows_core::PCWSTR, brepair: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddRestoreSubcomponent(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&componenttype), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszsubcomponentlogicalpath), core::mem::transmute(&wszsubcomponentname), core::mem::transmute_copy(&brepair)).into()
            }
        }
        unsafe extern "system" fn SetFileRestoreStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, status: super::vswriter::VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetFileRestoreStatus(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn AddNewTarget<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilename: windows_core::PCWSTR, brecursive: bool, wszalternatepath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddNewTarget(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilename), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatepath)).into()
            }
        }
        unsafe extern "system" fn SetRangesFilePath<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, ipartialfile: u32, wszrangesfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRangesFilePath(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&ipartialfile), core::mem::transmute(&wszrangesfile)).into()
            }
        }
        unsafe extern "system" fn PreRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::PreRestore(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PostRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::PostRestore(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContext<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
            }
        }
        unsafe extern "system" fn StartSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotsetid: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::StartSnapshotSet(this) {
                    Ok(ok__) => {
                        psnapshotsetid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddToSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: windows_core::PCWSTR, providerid: super::vss::VSS_ID, pidsnapshot: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::AddToSnapshotSet(this, core::mem::transmute(&pwszvolumename), core::mem::transmute(&providerid)) {
                    Ok(ok__) => {
                        pidsnapshot.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DoSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::DoSnapshotSet(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: super::vss::VSS_ID, esourceobjecttype: super::vss::VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *const i32, pnondeletedsnapshotid: *const super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
            }
        }
        unsafe extern "system" fn ImportSnapshots<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::ImportSnapshots(this) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BreakSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::BreakSnapshotSet(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::vss::VSS_ID, pprop: *mut super::vss::VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: super::vss::VSS_ID, equeriedobjecttype: super::vss::VSS_OBJECT_TYPE, ereturnedobjectstype: super::vss::VSS_OBJECT_TYPE, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype), core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: super::vss::VSS_ID, pwszvolumename: windows_core::PCWSTR, pbsupportedbythisprovider: *const windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::IsVolumeSupported(this, core::mem::transmute(&providerid), core::mem::transmute(&pwszvolumename), core::mem::transmute_copy(&pbsupportedbythisprovider)).into()
            }
        }
        unsafe extern "system" fn DisableWriterClasses<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgwriterclassid: *const super::vss::VSS_ID, cclassid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DisableWriterClasses(this, core::mem::transmute_copy(&rgwriterclassid), core::mem::transmute_copy(&cclassid)).into()
            }
        }
        unsafe extern "system" fn EnableWriterClasses<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgwriterclassid: *const super::vss::VSS_ID, cclassid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::EnableWriterClasses(this, core::mem::transmute_copy(&rgwriterclassid), core::mem::transmute_copy(&cclassid)).into()
            }
        }
        unsafe extern "system" fn DisableWriterInstances<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgwriterinstanceid: *const super::vss::VSS_ID, cinstanceid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DisableWriterInstances(this, core::mem::transmute_copy(&rgwriterinstanceid), core::mem::transmute_copy(&cinstanceid)).into()
            }
        }
        unsafe extern "system" fn ExposeSnapshot<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::vss::VSS_ID, wszpathfromroot: windows_core::PCWSTR, lattributes: i32, wszexpose: windows_core::PCWSTR, pwszexposed: *mut super::vss::VSS_PWSZ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::ExposeSnapshot(this, core::mem::transmute(&snapshotid), core::mem::transmute(&wszpathfromroot), core::mem::transmute_copy(&lattributes), core::mem::transmute(&wszexpose)) {
                    Ok(ok__) => {
                        pwszexposed.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::vss::VSS_ID, bforcedismount: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::RevertToSnapshot(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&bforcedismount)).into()
            }
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolume: windows_core::PCWSTR, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponents_Impl::QueryRevertStatus(this, core::mem::transmute(&pwszvolume)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWriterComponentsCount: GetWriterComponentsCount::<Identity, OFFSET>,
            GetWriterComponents: GetWriterComponents::<Identity, OFFSET>,
            InitializeForBackup: InitializeForBackup::<Identity, OFFSET>,
            SetBackupState: SetBackupState::<Identity, OFFSET>,
            InitializeForRestore: InitializeForRestore::<Identity, OFFSET>,
            SetRestoreState: SetRestoreState::<Identity, OFFSET>,
            GatherWriterMetadata: GatherWriterMetadata::<Identity, OFFSET>,
            GetWriterMetadataCount: GetWriterMetadataCount::<Identity, OFFSET>,
            GetWriterMetadata: GetWriterMetadata::<Identity, OFFSET>,
            FreeWriterMetadata: FreeWriterMetadata::<Identity, OFFSET>,
            AddComponent: AddComponent::<Identity, OFFSET>,
            PrepareForBackup: PrepareForBackup::<Identity, OFFSET>,
            AbortBackup: AbortBackup::<Identity, OFFSET>,
            GatherWriterStatus: GatherWriterStatus::<Identity, OFFSET>,
            GetWriterStatusCount: GetWriterStatusCount::<Identity, OFFSET>,
            FreeWriterStatus: FreeWriterStatus::<Identity, OFFSET>,
            GetWriterStatus: GetWriterStatus::<Identity, OFFSET>,
            SetBackupSucceeded: SetBackupSucceeded::<Identity, OFFSET>,
            SetBackupOptions: SetBackupOptions::<Identity, OFFSET>,
            SetSelectedForRestore: SetSelectedForRestore::<Identity, OFFSET>,
            SetRestoreOptions: SetRestoreOptions::<Identity, OFFSET>,
            SetAdditionalRestores: SetAdditionalRestores::<Identity, OFFSET>,
            SetPreviousBackupStamp: SetPreviousBackupStamp::<Identity, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, OFFSET>,
            BackupComplete: BackupComplete::<Identity, OFFSET>,
            AddAlternativeLocationMapping: AddAlternativeLocationMapping::<Identity, OFFSET>,
            AddRestoreSubcomponent: AddRestoreSubcomponent::<Identity, OFFSET>,
            SetFileRestoreStatus: SetFileRestoreStatus::<Identity, OFFSET>,
            AddNewTarget: AddNewTarget::<Identity, OFFSET>,
            SetRangesFilePath: SetRangesFilePath::<Identity, OFFSET>,
            PreRestore: PreRestore::<Identity, OFFSET>,
            PostRestore: PostRestore::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            StartSnapshotSet: StartSnapshotSet::<Identity, OFFSET>,
            AddToSnapshotSet: AddToSnapshotSet::<Identity, OFFSET>,
            DoSnapshotSet: DoSnapshotSet::<Identity, OFFSET>,
            DeleteSnapshots: DeleteSnapshots::<Identity, OFFSET>,
            ImportSnapshots: ImportSnapshots::<Identity, OFFSET>,
            BreakSnapshotSet: BreakSnapshotSet::<Identity, OFFSET>,
            GetSnapshotProperties: GetSnapshotProperties::<Identity, OFFSET>,
            Query: Query::<Identity, OFFSET>,
            IsVolumeSupported: IsVolumeSupported::<Identity, OFFSET>,
            DisableWriterClasses: DisableWriterClasses::<Identity, OFFSET>,
            EnableWriterClasses: EnableWriterClasses::<Identity, OFFSET>,
            DisableWriterInstances: DisableWriterInstances::<Identity, OFFSET>,
            ExposeSnapshot: ExposeSnapshot::<Identity, OFFSET>,
            RevertToSnapshot: RevertToSnapshot::<Identity, OFFSET>,
            QueryRevertStatus: QueryRevertStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssBackupComponents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssBackupComponents {}
windows_core::imp::define_interface!(IVssBackupComponentsEx, IVssBackupComponentsEx_Vtbl, 0x963f03ad_9e4c_4a34_ac15_e4b6174e5036);
impl core::ops::Deref for IVssBackupComponentsEx {
    type Target = IVssBackupComponents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssBackupComponentsEx, windows_core::IUnknown, IVssBackupComponents);
impl IVssBackupComponentsEx {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetWriterMetadataEx(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, ppmetadata: *mut Option<IVssExamineWriterMetadataEx>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterMetadataEx)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, core::mem::transmute(ppmetadata)) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetSelectedForRestoreEx<P2, P3>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, bselectedforrestore: bool, instanceid: super::vss::VSS_ID) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedForRestoreEx)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bselectedforrestore, core::mem::transmute(instanceid)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx_Vtbl {
    pub base__: IVssBackupComponents_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub GetWriterMetadataEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::vss::VSS_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetWriterMetadataEx: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetSelectedForRestoreEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool, super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetSelectedForRestoreEx: usize,
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssBackupComponentsEx_Impl: IVssBackupComponents_Impl {
    fn GetWriterMetadataEx(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, ppmetadata: windows_core::OutRef<IVssExamineWriterMetadataEx>) -> windows_core::Result<()>;
    fn SetSelectedForRestoreEx(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bselectedforrestore: bool, instanceid: &super::vss::VSS_ID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssBackupComponentsEx_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterMetadataEx<Identity: IVssBackupComponentsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx_Impl::GetWriterMetadataEx(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&ppmetadata)).into()
            }
        }
        unsafe extern "system" fn SetSelectedForRestoreEx<Identity: IVssBackupComponentsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bselectedforrestore: bool, instanceid: super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx_Impl::SetSelectedForRestoreEx(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bselectedforrestore), core::mem::transmute(&instanceid)).into()
            }
        }
        Self {
            base__: IVssBackupComponents_Vtbl::new::<Identity, OFFSET>(),
            GetWriterMetadataEx: GetWriterMetadataEx::<Identity, OFFSET>,
            SetSelectedForRestoreEx: SetSelectedForRestoreEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssBackupComponentsEx as windows_core::Interface>::IID || iid == &<IVssBackupComponents as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssBackupComponentsEx {}
windows_core::imp::define_interface!(IVssBackupComponentsEx2, IVssBackupComponentsEx2_Vtbl, 0xacfe2b3a_22c9_4ef8_bd03_2f9ca230084e);
impl core::ops::Deref for IVssBackupComponentsEx2 {
    type Target = IVssBackupComponentsEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssBackupComponentsEx2, windows_core::IUnknown, IVssBackupComponents, IVssBackupComponentsEx);
impl IVssBackupComponentsEx2 {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn UnexposeSnapshot(&self, snapshotid: super::vss::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnexposeSnapshot)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid)) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetAuthoritativeRestore<P2, P3>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, bauth: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAuthoritativeRestore)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bauth) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetRollForward<P2, P3, P5>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, rolltype: super::vss::VSS_ROLLFORWARD_TYPE, wszrollforwardpoint: P5) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRollForward)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), rolltype, wszrollforwardpoint.param().abi()) }
    }
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn SetRestoreName<P2, P3, P4>(&self, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszrestorename: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreName)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszrestorename.param().abi()) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn BreakSnapshotSetEx(&self, snapshotsetid: super::vss::VSS_ID, dwbreakflags: u32) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BreakSnapshotSetEx)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), dwbreakflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn PreFastRecovery(&self, snapshotsetid: super::vss::VSS_ID, dwprefastrecoveryflags: u32) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreFastRecovery)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), dwprefastrecoveryflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn FastRecovery(&self, snapshotsetid: super::vss::VSS_ID, dwfastrecoveryflags: u32) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FastRecovery)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotsetid), dwfastrecoveryflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx2_Vtbl {
    pub base__: IVssBackupComponentsEx_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub UnexposeSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    UnexposeSnapshot: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetAuthoritativeRestore: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetAuthoritativeRestore: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetRollForward: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, super::vss::VSS_ROLLFORWARD_TYPE, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetRollForward: usize,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub SetRestoreName: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, super::vswriter::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    SetRestoreName: usize,
    #[cfg(feature = "Win32_vss")]
    pub BreakSnapshotSetEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    BreakSnapshotSetEx: usize,
    #[cfg(feature = "Win32_vss")]
    pub PreFastRecovery: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    PreFastRecovery: usize,
    #[cfg(feature = "Win32_vss")]
    pub FastRecovery: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    FastRecovery: usize,
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssBackupComponentsEx2_Impl: IVssBackupComponentsEx_Impl {
    fn UnexposeSnapshot(&self, snapshotid: &super::vss::VSS_ID) -> windows_core::Result<()>;
    fn SetAuthoritativeRestore(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bauth: bool) -> windows_core::Result<()>;
    fn SetRollForward(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, rolltype: super::vss::VSS_ROLLFORWARD_TYPE, wszrollforwardpoint: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRestoreName(&self, writerid: &super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszrestorename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BreakSnapshotSetEx(&self, snapshotsetid: &super::vss::VSS_ID, dwbreakflags: u32) -> windows_core::Result<super::vss::IVssAsync>;
    fn PreFastRecovery(&self, snapshotsetid: &super::vss::VSS_ID, dwprefastrecoveryflags: u32) -> windows_core::Result<super::vss::IVssAsync>;
    fn FastRecovery(&self, snapshotsetid: &super::vss::VSS_ID, dwfastrecoveryflags: u32) -> windows_core::Result<super::vss::IVssAsync>;
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssBackupComponentsEx2_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnexposeSnapshot<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::UnexposeSnapshot(this, core::mem::transmute(&snapshotid)).into()
            }
        }
        unsafe extern "system" fn SetAuthoritativeRestore<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bauth: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::SetAuthoritativeRestore(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bauth)).into()
            }
        }
        unsafe extern "system" fn SetRollForward<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, rolltype: super::vss::VSS_ROLLFORWARD_TYPE, wszrollforwardpoint: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::SetRollForward(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&rolltype), core::mem::transmute(&wszrollforwardpoint)).into()
            }
        }
        unsafe extern "system" fn SetRestoreName<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, ct: super::vswriter::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszrestorename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::SetRestoreName(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszrestorename)).into()
            }
        }
        unsafe extern "system" fn BreakSnapshotSetEx<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::vss::VSS_ID, dwbreakflags: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponentsEx2_Impl::BreakSnapshotSetEx(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&dwbreakflags)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PreFastRecovery<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::vss::VSS_ID, dwprefastrecoveryflags: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponentsEx2_Impl::PreFastRecovery(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&dwprefastrecoveryflags)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FastRecovery<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::vss::VSS_ID, dwfastrecoveryflags: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponentsEx2_Impl::FastRecovery(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&dwfastrecoveryflags)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IVssBackupComponentsEx_Vtbl::new::<Identity, OFFSET>(),
            UnexposeSnapshot: UnexposeSnapshot::<Identity, OFFSET>,
            SetAuthoritativeRestore: SetAuthoritativeRestore::<Identity, OFFSET>,
            SetRollForward: SetRollForward::<Identity, OFFSET>,
            SetRestoreName: SetRestoreName::<Identity, OFFSET>,
            BreakSnapshotSetEx: BreakSnapshotSetEx::<Identity, OFFSET>,
            PreFastRecovery: PreFastRecovery::<Identity, OFFSET>,
            FastRecovery: FastRecovery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssBackupComponentsEx2 as windows_core::Interface>::IID || iid == &<IVssBackupComponents as windows_core::Interface>::IID || iid == &<IVssBackupComponentsEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssBackupComponentsEx2 {}
windows_core::imp::define_interface!(IVssBackupComponentsEx3, IVssBackupComponentsEx3_Vtbl, 0xc191bfbc_b602_4675_8bd1_67d642f529d5);
impl core::ops::Deref for IVssBackupComponentsEx3 {
    type Target = IVssBackupComponentsEx2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssBackupComponentsEx3, windows_core::IUnknown, IVssBackupComponents, IVssBackupComponentsEx, IVssBackupComponentsEx2);
impl IVssBackupComponentsEx3 {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetWriterStatusEx(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::vss::VSS_WRITER_STATE, phrfailurewriter: *mut windows_core::HRESULT, phrapplication: Option<*mut windows_core::HRESULT>, pbstrapplicationmessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterStatusEx)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwriter), pnstatus as _, phrfailurewriter as _, phrapplication.unwrap_or(core::mem::zeroed()) as _, pbstrapplicationmessage.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn AddSnapshotToRecoverySet(&self, snapshotid: super::vss::VSS_ID, dwflags: u32, pwszdestinationvolume: Option<*const u16>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSnapshotToRecoverySet)(windows_core::Interface::as_raw(self), core::mem::transmute(snapshotid), dwflags, pwszdestinationvolume.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn RecoverSet(&self, dwflags: u32) -> windows_core::Result<super::vss::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecoverSet)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetSessionId(&self) -> windows_core::Result<super::vss::VSS_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx3_Vtbl {
    pub base__: IVssBackupComponentsEx2_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub GetWriterStatusEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::vss::VSS_ID, *mut super::vss::VSS_ID, *mut *mut core::ffi::c_void, *mut super::vss::VSS_WRITER_STATE, *mut windows_core::HRESULT, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetWriterStatusEx: usize,
    #[cfg(feature = "Win32_vss")]
    pub AddSnapshotToRecoverySet: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, u32, *const u16) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    AddSnapshotToRecoverySet: usize,
    #[cfg(feature = "Win32_vss")]
    pub RecoverSet: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    RecoverSet: usize,
    #[cfg(feature = "Win32_vss")]
    pub GetSessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetSessionId: usize,
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssBackupComponentsEx3_Impl: IVssBackupComponentsEx2_Impl {
    fn GetWriterStatusEx(&self, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::vss::VSS_WRITER_STATE, phrfailurewriter: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSnapshotToRecoverySet(&self, snapshotid: &super::vss::VSS_ID, dwflags: u32, pwszdestinationvolume: *const u16) -> windows_core::Result<()>;
    fn RecoverSet(&self, dwflags: u32) -> windows_core::Result<super::vss::IVssAsync>;
    fn GetSessionId(&self) -> windows_core::Result<super::vss::VSS_ID>;
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssBackupComponentsEx3_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterStatusEx<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: u32, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwriter: *mut *mut core::ffi::c_void, pnstatus: *mut super::vss::VSS_WRITER_STATE, phrfailurewriter: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx3_Impl::GetWriterStatusEx(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwriter), core::mem::transmute_copy(&pnstatus), core::mem::transmute_copy(&phrfailurewriter), core::mem::transmute_copy(&phrapplication), core::mem::transmute_copy(&pbstrapplicationmessage)).into()
            }
        }
        unsafe extern "system" fn AddSnapshotToRecoverySet<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::vss::VSS_ID, dwflags: u32, pwszdestinationvolume: *const u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx3_Impl::AddSnapshotToRecoverySet(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwszdestinationvolume)).into()
            }
        }
        unsafe extern "system" fn RecoverSet<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponentsEx3_Impl::RecoverSet(this, core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppasync.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSessionId<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsession: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssBackupComponentsEx3_Impl::GetSessionId(this) {
                    Ok(ok__) => {
                        idsession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IVssBackupComponentsEx2_Vtbl::new::<Identity, OFFSET>(),
            GetWriterStatusEx: GetWriterStatusEx::<Identity, OFFSET>,
            AddSnapshotToRecoverySet: AddSnapshotToRecoverySet::<Identity, OFFSET>,
            RecoverSet: RecoverSet::<Identity, OFFSET>,
            GetSessionId: GetSessionId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssBackupComponentsEx3 as windows_core::Interface>::IID || iid == &<IVssBackupComponents as windows_core::Interface>::IID || iid == &<IVssBackupComponentsEx as windows_core::Interface>::IID || iid == &<IVssBackupComponentsEx2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssBackupComponentsEx3 {}
windows_core::imp::define_interface!(IVssBackupComponentsEx4, IVssBackupComponentsEx4_Vtbl, 0xf434c2fd_b553_4961_a9f9_a8e90b673e53);
impl core::ops::Deref for IVssBackupComponentsEx4 {
    type Target = IVssBackupComponentsEx3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssBackupComponentsEx4, windows_core::IUnknown, IVssBackupComponents, IVssBackupComponentsEx, IVssBackupComponentsEx2, IVssBackupComponentsEx3);
impl IVssBackupComponentsEx4 {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetRootAndLogicalPrefixPaths(&self, pwszfilepath: *const u16, ppwszrootpath: *mut super::vss::VSS_PWSZ, ppwszlogicalprefix: *mut super::vss::VSS_PWSZ, bnormalizefqdnforrootpath: Option<windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRootAndLogicalPrefixPaths)(windows_core::Interface::as_raw(self), pwszfilepath, ppwszrootpath as _, ppwszlogicalprefix as _, bnormalizefqdnforrootpath.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx4_Vtbl {
    pub base__: IVssBackupComponentsEx3_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub GetRootAndLogicalPrefixPaths: unsafe extern "system" fn(*mut core::ffi::c_void, *const u16, *mut super::vss::VSS_PWSZ, *mut super::vss::VSS_PWSZ, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetRootAndLogicalPrefixPaths: usize,
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssBackupComponentsEx4_Impl: IVssBackupComponentsEx3_Impl {
    fn GetRootAndLogicalPrefixPaths(&self, pwszfilepath: *const u16, ppwszrootpath: *mut super::vss::VSS_PWSZ, ppwszlogicalprefix: *mut super::vss::VSS_PWSZ, bnormalizefqdnforrootpath: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssBackupComponentsEx4_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRootAndLogicalPrefixPaths<Identity: IVssBackupComponentsEx4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilepath: *const u16, ppwszrootpath: *mut super::vss::VSS_PWSZ, ppwszlogicalprefix: *mut super::vss::VSS_PWSZ, bnormalizefqdnforrootpath: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx4_Impl::GetRootAndLogicalPrefixPaths(this, core::mem::transmute_copy(&pwszfilepath), core::mem::transmute_copy(&ppwszrootpath), core::mem::transmute_copy(&ppwszlogicalprefix), core::mem::transmute_copy(&bnormalizefqdnforrootpath)).into()
            }
        }
        Self { base__: IVssBackupComponentsEx3_Vtbl::new::<Identity, OFFSET>(), GetRootAndLogicalPrefixPaths: GetRootAndLogicalPrefixPaths::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssBackupComponentsEx4 as windows_core::Interface>::IID || iid == &<IVssBackupComponents as windows_core::Interface>::IID || iid == &<IVssBackupComponentsEx as windows_core::Interface>::IID || iid == &<IVssBackupComponentsEx2 as windows_core::Interface>::IID || iid == &<IVssBackupComponentsEx3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssBackupComponentsEx4 {}
windows_core::imp::define_interface!(IVssExamineWriterMetadata, IVssExamineWriterMetadata_Vtbl, 0x902fcf7f_b7fd_42f8_81f1_b2e400b1e5bd);
windows_core::imp::interface_hierarchy!(IVssExamineWriterMetadata, windows_core::IUnknown);
impl IVssExamineWriterMetadata {
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn GetIdentity(&self, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pusage: *mut super::vswriter::VSS_USAGE_TYPE, psource: *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentity)(windows_core::Interface::as_raw(self), pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwritername), pusage as _, psource as _) }
    }
    pub unsafe fn GetFileCounts(&self, pcincludefiles: *mut u32, pcexcludefiles: *mut u32, pccomponents: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileCounts)(windows_core::Interface::as_raw(self), pcincludefiles as _, pcexcludefiles as _, pccomponents as _) }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetIncludeFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIncludeFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetExcludeFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludeFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssWMComponent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponent)(windows_core::Interface::as_raw(self), icomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetRestoreMethod(&self, pmethod: *mut super::vswriter::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut windows_core::BSTR, pbstruserprocedure: *mut windows_core::BSTR, pwriterrestore: *mut super::vswriter::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreMethod)(windows_core::Interface::as_raw(self), pmethod as _, core::mem::transmute(pbstrservice), core::mem::transmute(pbstruserprocedure), pwriterrestore as _, pbrebootrequired as _, pcmappings as _) }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocationMapping)(windows_core::Interface::as_raw(self), imapping, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBackupSchema(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupSchema)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
    pub unsafe fn GetDocument(&self) -> windows_core::Result<super::msxml::IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrxml)) }
    }
    pub unsafe fn LoadFromXML(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LoadFromXML)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExamineWriterMetadata_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub GetIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID, *mut super::vss::VSS_ID, *mut *mut core::ffi::c_void, *mut super::vswriter::VSS_USAGE_TYPE, *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    GetIdentity: usize,
    pub GetFileCounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vswriter")]
    pub GetIncludeFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetIncludeFile: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub GetExcludeFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetExcludeFile: usize,
    pub GetComponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vswriter")]
    pub GetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vswriter::VSS_RESTOREMETHOD_ENUM, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::vswriter::VSS_WRITERRESTORE_ENUM, *mut bool, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetRestoreMethod: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub GetAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetAlternateLocationMapping: usize,
    pub GetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msxml", feature = "Win32_oaidl")))]
    GetDocument: usize,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadFromXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssExamineWriterMetadata_Impl: windows_core::IUnknownImpl {
    fn GetIdentity(&self, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pusage: *mut super::vswriter::VSS_USAGE_TYPE, psource: *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::Result<()>;
    fn GetFileCounts(&self, pcincludefiles: *mut u32, pcexcludefiles: *mut u32, pccomponents: *mut u32) -> windows_core::Result<()>;
    fn GetIncludeFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
    fn GetExcludeFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
    fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssWMComponent>;
    fn GetRestoreMethod(&self, pmethod: *mut super::vswriter::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut windows_core::BSTR, pbstruserprocedure: *mut windows_core::BSTR, pwriterrestore: *mut super::vswriter::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut u32) -> windows_core::Result<()>;
    fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
    fn GetBackupSchema(&self) -> windows_core::Result<u32>;
    fn GetDocument(&self) -> windows_core::Result<super::msxml::IXMLDOMDocument>;
    fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadFromXML(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssExamineWriterMetadata_Vtbl {
    pub const fn new<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentity<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwritername: *mut *mut core::ffi::c_void, pusage: *mut super::vswriter::VSS_USAGE_TYPE, psource: *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetIdentity(this, core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwritername), core::mem::transmute_copy(&pusage), core::mem::transmute_copy(&psource)).into()
            }
        }
        unsafe extern "system" fn GetFileCounts<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcincludefiles: *mut u32, pcexcludefiles: *mut u32, pccomponents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetFileCounts(this, core::mem::transmute_copy(&pcincludefiles), core::mem::transmute_copy(&pcexcludefiles), core::mem::transmute_copy(&pccomponents)).into()
            }
        }
        unsafe extern "system" fn GetIncludeFile<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadata_Impl::GetIncludeFile(this, core::mem::transmute_copy(&ifile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExcludeFile<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadata_Impl::GetExcludeFile(this, core::mem::transmute_copy(&ifile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComponent<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadata_Impl::GetComponent(this, core::mem::transmute_copy(&icomponent)) {
                    Ok(ok__) => {
                        ppcomponent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestoreMethod<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmethod: *mut super::vswriter::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut *mut core::ffi::c_void, pbstruserprocedure: *mut *mut core::ffi::c_void, pwriterrestore: *mut super::vswriter::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetRestoreMethod(this, core::mem::transmute_copy(&pmethod), core::mem::transmute_copy(&pbstrservice), core::mem::transmute_copy(&pbstruserprocedure), core::mem::transmute_copy(&pwriterrestore), core::mem::transmute_copy(&pbrebootrequired), core::mem::transmute_copy(&pcmappings)).into()
            }
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadata_Impl::GetAlternateLocationMapping(this, core::mem::transmute_copy(&imapping)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackupSchema<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwschemamask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadata_Impl::GetBackupSchema(this) {
                    Ok(ok__) => {
                        pdwschemamask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDocument<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadata_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        pdoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::SaveAsXML(this, core::mem::transmute_copy(&pbstrxml)).into()
            }
        }
        unsafe extern "system" fn LoadFromXML<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::LoadFromXML(this, core::mem::transmute(&bstrxml)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIdentity: GetIdentity::<Identity, OFFSET>,
            GetFileCounts: GetFileCounts::<Identity, OFFSET>,
            GetIncludeFile: GetIncludeFile::<Identity, OFFSET>,
            GetExcludeFile: GetExcludeFile::<Identity, OFFSET>,
            GetComponent: GetComponent::<Identity, OFFSET>,
            GetRestoreMethod: GetRestoreMethod::<Identity, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, OFFSET>,
            GetBackupSchema: GetBackupSchema::<Identity, OFFSET>,
            GetDocument: GetDocument::<Identity, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, OFFSET>,
            LoadFromXML: LoadFromXML::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssExamineWriterMetadata as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssExamineWriterMetadata {}
windows_core::imp::define_interface!(IVssExamineWriterMetadataEx, IVssExamineWriterMetadataEx_Vtbl, 0x0c0e5ec0_ca44_472b_b702_e652db1c0451);
impl core::ops::Deref for IVssExamineWriterMetadataEx {
    type Target = IVssExamineWriterMetadata;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssExamineWriterMetadataEx, windows_core::IUnknown, IVssExamineWriterMetadata);
impl IVssExamineWriterMetadataEx {
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub unsafe fn GetIdentityEx(&self, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pbstrinstancename: *mut windows_core::BSTR, pusage: *mut super::vswriter::VSS_USAGE_TYPE, psource: *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentityEx)(windows_core::Interface::as_raw(self), pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwritername), core::mem::transmute(pbstrinstancename), pusage as _, psource as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExamineWriterMetadataEx_Vtbl {
    pub base__: IVssExamineWriterMetadata_Vtbl,
    #[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
    pub GetIdentityEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID, *mut super::vss::VSS_ID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::vswriter::VSS_USAGE_TYPE, *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_vss", feature = "Win32_vswriter")))]
    GetIdentityEx: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssExamineWriterMetadataEx_Impl: IVssExamineWriterMetadata_Impl {
    fn GetIdentityEx(&self, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pbstrinstancename: *mut windows_core::BSTR, pusage: *mut super::vswriter::VSS_USAGE_TYPE, psource: *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssExamineWriterMetadataEx_Vtbl {
    pub const fn new<Identity: IVssExamineWriterMetadataEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentityEx<Identity: IVssExamineWriterMetadataEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID, pbstrwritername: *mut *mut core::ffi::c_void, pbstrinstancename: *mut *mut core::ffi::c_void, pusage: *mut super::vswriter::VSS_USAGE_TYPE, psource: *mut super::vswriter::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadataEx_Impl::GetIdentityEx(this, core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwritername), core::mem::transmute_copy(&pbstrinstancename), core::mem::transmute_copy(&pusage), core::mem::transmute_copy(&psource)).into()
            }
        }
        Self { base__: IVssExamineWriterMetadata_Vtbl::new::<Identity, OFFSET>(), GetIdentityEx: GetIdentityEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssExamineWriterMetadataEx as windows_core::Interface>::IID || iid == &<IVssExamineWriterMetadata as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssExamineWriterMetadataEx {}
windows_core::imp::define_interface!(IVssExamineWriterMetadataEx2, IVssExamineWriterMetadataEx2_Vtbl, 0xce115780_a611_431b_b57f_c38303ab6aee);
impl core::ops::Deref for IVssExamineWriterMetadataEx2 {
    type Target = IVssExamineWriterMetadataEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssExamineWriterMetadataEx2, windows_core::IUnknown, IVssExamineWriterMetadata, IVssExamineWriterMetadataEx);
impl IVssExamineWriterMetadataEx2 {
    pub unsafe fn GetVersion(&self, pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), pdwmajorversion as _, pdwminorversion as _) }
    }
    pub unsafe fn GetExcludeFromSnapshotCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludeFromSnapshotCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetExcludeFromSnapshotFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludeFromSnapshotFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExamineWriterMetadataEx2_Vtbl {
    pub base__: IVssExamineWriterMetadataEx_Vtbl,
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetExcludeFromSnapshotCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vswriter")]
    pub GetExcludeFromSnapshotFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetExcludeFromSnapshotFile: usize,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssExamineWriterMetadataEx2_Impl: IVssExamineWriterMetadataEx_Impl {
    fn GetVersion(&self, pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> windows_core::Result<()>;
    fn GetExcludeFromSnapshotCount(&self) -> windows_core::Result<u32>;
    fn GetExcludeFromSnapshotFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssExamineWriterMetadataEx2_Vtbl {
    pub const fn new<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersion<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadataEx2_Impl::GetVersion(this, core::mem::transmute_copy(&pdwmajorversion), core::mem::transmute_copy(&pdwminorversion)).into()
            }
        }
        unsafe extern "system" fn GetExcludeFromSnapshotCount<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcexcludedfromsnapshot: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadataEx2_Impl::GetExcludeFromSnapshotCount(this) {
                    Ok(ok__) => {
                        pcexcludedfromsnapshot.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExcludeFromSnapshotFile<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExamineWriterMetadataEx2_Impl::GetExcludeFromSnapshotFile(this, core::mem::transmute_copy(&ifile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IVssExamineWriterMetadataEx_Vtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, OFFSET>,
            GetExcludeFromSnapshotCount: GetExcludeFromSnapshotCount::<Identity, OFFSET>,
            GetExcludeFromSnapshotFile: GetExcludeFromSnapshotFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssExamineWriterMetadataEx2 as windows_core::Interface>::IID || iid == &<IVssExamineWriterMetadata as windows_core::Interface>::IID || iid == &<IVssExamineWriterMetadataEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss", feature = "Win32_vswriter"))]
impl windows_core::RuntimeName for IVssExamineWriterMetadataEx2 {}
windows_core::imp::define_interface!(IVssWMComponent, IVssWMComponent_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMComponent, windows_core::IUnknown);
impl IVssWMComponent {
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetComponentInfo(&self) -> windows_core::Result<PVSSCOMPONENTINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn FreeComponentInfo(&self, pinfo: *const VSS_COMPONENTINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeComponentInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pinfo)) }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetDatabaseFile(&self, idbfile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDatabaseFile)(windows_core::Interface::as_raw(self), idbfile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetDatabaseLogFile(&self, idblogfile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDatabaseLogFile)(windows_core::Interface::as_raw(self), idblogfile, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_vswriter")]
    pub unsafe fn GetDependency(&self, idependency: u32) -> windows_core::Result<super::vswriter::IVssWMDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDependency)(windows_core::Interface::as_raw(self), idependency, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_vswriter")]
    pub GetComponentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PVSSCOMPONENTINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetComponentInfo: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub FreeComponentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *const VSS_COMPONENTINFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    FreeComponentInfo: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetFile: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub GetDatabaseFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetDatabaseFile: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub GetDatabaseLogFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetDatabaseLogFile: usize,
    #[cfg(feature = "Win32_vswriter")]
    pub GetDependency: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vswriter"))]
    GetDependency: usize,
}
#[cfg(feature = "Win32_vswriter")]
pub trait IVssWMComponent_Impl: windows_core::IUnknownImpl {
    fn GetComponentInfo(&self) -> windows_core::Result<PVSSCOMPONENTINFO>;
    fn FreeComponentInfo(&self, pinfo: *const VSS_COMPONENTINFO) -> windows_core::Result<()>;
    fn GetFile(&self, ifile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
    fn GetDatabaseFile(&self, idbfile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
    fn GetDatabaseLogFile(&self, idblogfile: u32) -> windows_core::Result<super::vswriter::IVssWMFiledesc>;
    fn GetDependency(&self, idependency: u32) -> windows_core::Result<super::vswriter::IVssWMDependency>;
}
#[cfg(feature = "Win32_vswriter")]
impl IVssWMComponent_Vtbl {
    pub const fn new<Identity: IVssWMComponent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetComponentInfo<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut PVSSCOMPONENTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMComponent_Impl::GetComponentInfo(this) {
                    Ok(ok__) => {
                        ppinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeComponentInfo<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: *const VSS_COMPONENTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::FreeComponentInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn GetFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMComponent_Impl::GetFile(this, core::mem::transmute_copy(&ifile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDatabaseFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idbfile: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMComponent_Impl::GetDatabaseFile(this, core::mem::transmute_copy(&idbfile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDatabaseLogFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idblogfile: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMComponent_Impl::GetDatabaseLogFile(this, core::mem::transmute_copy(&idblogfile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDependency<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idependency: u32, ppdependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMComponent_Impl::GetDependency(this, core::mem::transmute_copy(&idependency)) {
                    Ok(ok__) => {
                        ppdependency.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetComponentInfo: GetComponentInfo::<Identity, OFFSET>,
            FreeComponentInfo: FreeComponentInfo::<Identity, OFFSET>,
            GetFile: GetFile::<Identity, OFFSET>,
            GetDatabaseFile: GetDatabaseFile::<Identity, OFFSET>,
            GetDatabaseLogFile: GetDatabaseLogFile::<Identity, OFFSET>,
            GetDependency: GetDependency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMComponent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_vswriter")]
impl windows_core::RuntimeName for IVssWMComponent {}
#[cfg(feature = "Win32_vswriter")]
windows_core::imp::define_interface!(IVssWriterComponentsExt, IVssWriterComponentsExt_Vtbl);
#[cfg(feature = "Win32_vswriter")]
impl core::ops::Deref for IVssWriterComponentsExt {
    type Target = super::vswriter::IVssWriterComponents;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_vswriter")]
windows_core::imp::interface_hierarchy!(IVssWriterComponentsExt, super::vswriter::IVssWriterComponents);
#[cfg(feature = "Win32_vswriter")]
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterComponentsExt_Vtbl {
    pub base__: super::vswriter::IVssWriterComponents_Vtbl,
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
pub trait IVssWriterComponentsExt_Impl: super::vswriter::IVssWriterComponents_Impl {}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssWriterComponentsExt_Vtbl {
    pub const fn new<Identity: IVssWriterComponentsExt_Impl>() -> Self {
        Self { base__: super::vswriter::IVssWriterComponents_Vtbl::new::<Identity>() }
    }
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
struct IVssWriterComponentsExt_ImplVtbl<T: IVssWriterComponentsExt_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl<T: IVssWriterComponentsExt_Impl> IVssWriterComponentsExt_ImplVtbl<T> {
    const VTABLE: IVssWriterComponentsExt_Vtbl = IVssWriterComponentsExt_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_vss", feature = "Win32_vswriter"))]
impl IVssWriterComponentsExt {
    pub fn new<'a, T: IVssWriterComponentsExt_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssWriterComponentsExt_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
#[cfg(feature = "Win32_vswriter")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVSSCOMPONENTINFO(pub *const VSS_COMPONENTINFO);
#[cfg(feature = "Win32_vswriter")]
impl PVSSCOMPONENTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_vswriter")]
impl Default for PVSSCOMPONENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_vswriter")]
#[derive(Clone, Debug, PartialEq)]
pub struct VSS_COMPONENTINFO {
    pub r#type: super::vswriter::VSS_COMPONENT_TYPE,
    pub bstrLogicalPath: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrComponentName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrCaption: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub pbIcon: *mut u8,
    pub cbIcon: u32,
    pub bRestoreMetadata: bool,
    pub bNotifyOnBackupComplete: bool,
    pub bSelectable: bool,
    pub bSelectableForRestore: bool,
    pub dwComponentFlags: u32,
    pub cFileCount: u32,
    pub cDatabases: u32,
    pub cLogFiles: u32,
    pub cDependencies: u32,
}
#[cfg(feature = "Win32_vswriter")]
impl Default for VSS_COMPONENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_SW_BOOTABLE_STATE: u32 = 1;
