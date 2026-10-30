#[inline]
pub unsafe fn CreateVssBackupComponentsInternal(ppbackup: *mut Option<IVssBackupComponents>) -> windows_core::HRESULT {
    windows_core::link!("vssapi.dll" "system" fn CreateVssBackupComponentsInternal(ppbackup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateVssBackupComponentsInternal(core::mem::transmute(ppbackup)) }
}
#[inline]
pub unsafe fn CreateVssExamineWriterMetadataInternal(bstrxml: &windows_core::BSTR, ppmetadata: *mut Option<IVssExamineWriterMetadata>) -> windows_core::HRESULT {
    windows_core::link!("vssapi.dll" "system" fn CreateVssExamineWriterMetadataInternal(bstrxml : *mut core::ffi::c_void, ppmetadata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { CreateVssExamineWriterMetadataInternal(core::mem::transmute_copy(bstrxml), core::mem::transmute(ppmetadata)) }
}
#[cfg(all(feature = "guiddef", feature = "vss"))]
#[inline]
pub unsafe fn GetProviderMgmtInterfaceInternal(providerid: super::VSS_ID, interfaceid: super::IID, ppitf: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
    windows_core::link!("vssapi.dll" "system" fn GetProviderMgmtInterfaceInternal(providerid : super::VSS_ID, interfaceid : super::IID, ppitf : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetProviderMgmtInterfaceInternal(providerid, interfaceid, core::mem::transmute(ppitf)) }
}
#[cfg(all(feature = "shared", feature = "vss"))]
#[inline]
pub unsafe fn IsVolumeSnapshottedInternal(pwszvolumename: super::VSS_PWSZ, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcapability: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("vssapi.dll" "system" fn IsVolumeSnapshottedInternal(pwszvolumename : super::VSS_PWSZ, pbsnapshotspresent : *mut windows_core::BOOL, plsnapshotcapability : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { IsVolumeSnapshottedInternal(pwszvolumename, pbsnapshotspresent as _, plsnapshotcapability as _) }
}
#[inline]
pub unsafe fn ShouldBlockRevertInternal<P0>(wszvolumename: P0, pbblock: *mut bool) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("vssapi.dll" "system" fn ShouldBlockRevertInternal(wszvolumename : windows_core::PCWSTR, pbblock : *mut bool) -> windows_core::HRESULT);
    unsafe { ShouldBlockRevertInternal(wszvolumename.param().abi(), pbblock as _) }
}
#[cfg(all(feature = "shared", feature = "vss"))]
#[inline]
pub unsafe fn VssFreeSnapshotPropertiesInternal(pprop: *const super::VSS_SNAPSHOT_PROP) {
    windows_core::link!("vssapi.dll" "system" fn VssFreeSnapshotPropertiesInternal(pprop : *const super::VSS_SNAPSHOT_PROP));
    unsafe { VssFreeSnapshotPropertiesInternal(pprop) }
}
pub const IID_IVssBackupComponents: windows_core::GUID = windows_core::GUID::from_u128(0x665c1d5f_c218_414d_a05d_7fef5f9d5c86);
pub const IID_IVssBackupComponentsEx: windows_core::GUID = windows_core::GUID::from_u128(0x963f03ad_9e4c_4a34_ac15_e4b6174e5036);
pub const IID_IVssBackupComponentsEx2: windows_core::GUID = windows_core::GUID::from_u128(0xacfe2b3a_22c9_4ef8_bd03_2f9ca230084e);
pub const IID_IVssBackupComponentsEx3: windows_core::GUID = windows_core::GUID::from_u128(0xc191bfbc_b602_4675_8bd1_67d642f529d5);
pub const IID_IVssBackupComponentsEx4: windows_core::GUID = windows_core::GUID::from_u128(0xf434c2fd_b553_4961_a9f9_a8e90b673e53);
pub const IID_IVssExamineWriterMetadata: windows_core::GUID = windows_core::GUID::from_u128(0x902fcf7f_b7fd_42f8_81f1_b2e400b1e5bd);
pub const IID_IVssExamineWriterMetadataEx: windows_core::GUID = windows_core::GUID::from_u128(0x0c0e5ec0_ca44_472b_b702_e652db1c0451);
pub const IID_IVssExamineWriterMetadataEx2: windows_core::GUID = windows_core::GUID::from_u128(0xce115780_a611_431b_b57f_c38303ab6aee);
windows_core::imp::define_interface!(IVssBackupComponents, IVssBackupComponents_Vtbl, 0x665c1d5f_c218_414d_a05d_7fef5f9d5c86);
windows_core::imp::interface_hierarchy!(IVssBackupComponents, windows_core::IUnknown);
impl IVssBackupComponents {
    #[cfg(feature = "shared")]
    pub unsafe fn GetWriterComponentsCount(&self, pccomponents: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterComponentsCount)(windows_core::Interface::as_raw(self), pccomponents as _) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetWriterComponents(&self, iwriter: super::UINT, ppwriter: *mut *mut IVssWriterComponentsExt) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterComponents)(windows_core::Interface::as_raw(self), iwriter, ppwriter as _) }
    }
    pub unsafe fn InitializeForBackup(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeForBackup)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn SetBackupState(&self, bselectcomponents: bool, bbackupbootablesystemstate: bool, backuptype: super::VSS_BACKUP_TYPE, bpartialfilesupport: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackupState)(windows_core::Interface::as_raw(self), bselectcomponents, bbackupbootablesystemstate, backuptype, bpartialfilesupport) }
    }
    pub unsafe fn InitializeForRestore(&self, bstrxml: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeForRestore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrxml)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn SetRestoreState(&self, restoretype: super::VSS_RESTORE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreState)(windows_core::Interface::as_raw(self), restoretype) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn GatherWriterMetadata(&self, pasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GatherWriterMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(pasync)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetWriterMetadataCount(&self, pcwriters: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterMetadataCount)(windows_core::Interface::as_raw(self), pcwriters as _) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn GetWriterMetadata(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, ppmetadata: *mut Option<IVssExamineWriterMetadata>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterMetadata)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, core::mem::transmute(ppmetadata)) }
    }
    pub unsafe fn FreeWriterMetadata(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeWriterMetadata)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn AddComponent<P3, P4>(&self, instanceid: super::VSS_ID, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P3, wszcomponentname: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), instanceid, writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi()) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn PrepareForBackup(&self, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareForBackup)(windows_core::Interface::as_raw(self), core::mem::transmute(ppasync)) }
    }
    pub unsafe fn AbortBackup(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AbortBackup)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn GatherWriterStatus(&self, pasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GatherWriterStatus)(windows_core::Interface::as_raw(self), core::mem::transmute(pasync)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetWriterStatusCount(&self, pcwriters: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterStatusCount)(windows_core::Interface::as_raw(self), pcwriters as _) }
    }
    pub unsafe fn FreeWriterStatus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeWriterStatus)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn GetWriterStatus(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::VSS_WRITER_STATE, phresultfailure: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterStatus)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwriter), pnstatus as _, phresultfailure as _) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetBackupSucceeded<P3, P4>(&self, instanceid: super::VSS_ID, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P3, wszcomponentname: P4, bsucceded: bool) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupSucceeded)(windows_core::Interface::as_raw(self), instanceid, writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bsucceded) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetBackupOptions<P2, P3, P4>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszbackupoptions: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupOptions)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszbackupoptions.param().abi()) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetSelectedForRestore<P2, P3>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, bselectedforrestore: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedForRestore)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bselectedforrestore) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetRestoreOptions<P2, P3, P4>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszrestoreoptions: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreOptions)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszrestoreoptions.param().abi()) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetAdditionalRestores<P2, P3>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, badditionalrestores: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAdditionalRestores)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), badditionalrestores) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetPreviousBackupStamp<P2, P3, P4>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszpreviousbackupstamp: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreviousBackupStamp)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszpreviousbackupstamp.param().abi()) }
    }
    pub unsafe fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrxml)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn BackupComplete(&self, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BackupComplete)(windows_core::Interface::as_raw(self), core::mem::transmute(ppasync)) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn AddAlternativeLocationMapping<P2, P3, P4, P5, P7>(&self, writerid: super::VSS_ID, componenttype: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszpath: P4, wszfilespec: P5, brecursive: bool, wszdestination: P7) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAlternativeLocationMapping)(windows_core::Interface::as_raw(self), writerid, componenttype, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszdestination.param().abi()) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn AddRestoreSubcomponent<P2, P3, P4, P5>(&self, writerid: super::VSS_ID, componenttype: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszsubcomponentlogicalpath: P4, wszsubcomponentname: P5, brepair: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRestoreSubcomponent)(windows_core::Interface::as_raw(self), writerid, componenttype, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszsubcomponentlogicalpath.param().abi(), wszsubcomponentname.param().abi(), brepair) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetFileRestoreStatus<P2, P3>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, status: super::VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFileRestoreStatus)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), status) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn AddNewTarget<P2, P3, P4, P5, P7>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszpath: P4, wszfilename: P5, brecursive: bool, wszalternatepath: P7) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P7: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNewTarget)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszpath.param().abi(), wszfilename.param().abi(), brecursive, wszalternatepath.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetRangesFilePath<P2, P3, P5>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, ipartialfile: super::UINT, wszrangesfile: P5) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRangesFilePath)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), ipartialfile, wszrangesfile.param().abi()) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn PreRestore(&self, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreRestore)(windows_core::Interface::as_raw(self), core::mem::transmute(ppasync)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn PostRestore(&self, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PostRestore)(windows_core::Interface::as_raw(self), core::mem::transmute(ppasync)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetContext(&self, lcontext: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), lcontext) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn StartSnapshotSet(&self, psnapshotsetid: *mut super::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartSnapshotSet)(windows_core::Interface::as_raw(self), psnapshotsetid as _) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn AddToSnapshotSet(&self, pwszvolumename: super::VSS_PWSZ, providerid: super::VSS_ID, pidsnapshot: *mut super::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddToSnapshotSet)(windows_core::Interface::as_raw(self), pwszvolumename, providerid, pidsnapshot as _) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn DoSnapshotSet(&self, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoSnapshotSet)(windows_core::Interface::as_raw(self), core::mem::transmute(ppasync)) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn DeleteSnapshots(&self, sourceobjectid: super::VSS_ID, esourceobjecttype: super::VSS_OBJECT_TYPE, bforcedelete: bool, pldeletedsnapshots: *const super::LONG, pnondeletedsnapshotid: *const super::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteSnapshots)(windows_core::Interface::as_raw(self), sourceobjectid, esourceobjecttype, bforcedelete.into(), pldeletedsnapshots, pnondeletedsnapshotid) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn ImportSnapshots(&self, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ImportSnapshots)(windows_core::Interface::as_raw(self), core::mem::transmute(ppasync)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn BreakSnapshotSet(&self, snapshotsetid: super::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BreakSnapshotSet)(windows_core::Interface::as_raw(self), snapshotsetid) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn GetSnapshotProperties(&self, snapshotid: super::VSS_ID, pprop: *mut super::VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSnapshotProperties)(windows_core::Interface::as_raw(self), snapshotid, pprop as _) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn Query(&self, queriedobjectid: super::VSS_ID, equeriedobjecttype: super::VSS_OBJECT_TYPE, ereturnedobjectstype: super::VSS_OBJECT_TYPE, ppenum: *const Option<super::IVssEnumObject>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), queriedobjectid, equeriedobjecttype, ereturnedobjectstype, core::mem::transmute(ppenum)) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn IsVolumeSupported(&self, providerid: super::VSS_ID, pwszvolumename: super::VSS_PWSZ, pbsupportedbythisprovider: *const windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsVolumeSupported)(windows_core::Interface::as_raw(self), providerid, pwszvolumename, pbsupportedbythisprovider) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn DisableWriterClasses(&self, rgwriterclassid: *const super::VSS_ID, cclassid: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableWriterClasses)(windows_core::Interface::as_raw(self), rgwriterclassid, cclassid) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn EnableWriterClasses(&self, rgwriterclassid: *const super::VSS_ID, cclassid: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableWriterClasses)(windows_core::Interface::as_raw(self), rgwriterclassid, cclassid) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn DisableWriterInstances(&self, rgwriterinstanceid: *const super::VSS_ID, cinstanceid: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableWriterInstances)(windows_core::Interface::as_raw(self), rgwriterinstanceid, cinstanceid) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn ExposeSnapshot(&self, snapshotid: super::VSS_ID, wszpathfromroot: super::VSS_PWSZ, lattributes: super::LONG, wszexpose: super::VSS_PWSZ, pwszexposed: *mut super::VSS_PWSZ) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExposeSnapshot)(windows_core::Interface::as_raw(self), snapshotid, wszpathfromroot, lattributes, wszexpose, pwszexposed as _) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn RevertToSnapshot(&self, snapshotid: super::VSS_ID, bforcedismount: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RevertToSnapshot)(windows_core::Interface::as_raw(self), snapshotid, bforcedismount.into()) }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn QueryRevertStatus(&self, pwszvolume: super::VSS_PWSZ, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryRevertStatus)(windows_core::Interface::as_raw(self), pwszvolume, core::mem::transmute(ppasync)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetWriterComponentsCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetWriterComponentsCount: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetWriterComponents: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut IVssWriterComponentsExt) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetWriterComponents: usize,
    pub InitializeForBackup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "vss")]
    pub SetBackupState: unsafe extern "system" fn(*mut core::ffi::c_void, bool, bool, super::VSS_BACKUP_TYPE, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    SetBackupState: usize,
    pub InitializeForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "vss")]
    pub SetRestoreState: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_RESTORE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    SetRestoreState: usize,
    #[cfg(feature = "vss")]
    pub GatherWriterMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    GatherWriterMetadata: usize,
    #[cfg(feature = "shared")]
    pub GetWriterMetadataCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetWriterMetadataCount: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub GetWriterMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::VSS_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    GetWriterMetadata: usize,
    pub FreeWriterMetadata: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    AddComponent: usize,
    #[cfg(feature = "vss")]
    pub PrepareForBackup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    PrepareForBackup: usize,
    pub AbortBackup: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "vss")]
    pub GatherWriterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    GatherWriterStatus: usize,
    #[cfg(feature = "shared")]
    pub GetWriterStatusCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetWriterStatusCount: usize,
    pub FreeWriterStatus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub GetWriterStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::VSS_ID, *mut super::VSS_ID, *mut *mut core::ffi::c_void, *mut super::VSS_WRITER_STATE, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    GetWriterStatus: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetBackupSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetBackupSucceeded: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetBackupOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetBackupOptions: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetSelectedForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetSelectedForRestore: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetRestoreOptions: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetRestoreOptions: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetAdditionalRestores: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetAdditionalRestores: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetPreviousBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetPreviousBackupStamp: usize,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "vss")]
    pub BackupComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    BackupComplete: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub AddAlternativeLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    AddAlternativeLocationMapping: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub AddRestoreSubcomponent: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    AddRestoreSubcomponent: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetFileRestoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, super::VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetFileRestoreStatus: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub AddNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    AddNewTarget: usize,
    #[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
    pub SetRangesFilePath: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, super::UINT, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss", feature = "vswriter")))]
    SetRangesFilePath: usize,
    #[cfg(feature = "vss")]
    pub PreRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    PreRestore: usize,
    #[cfg(feature = "vss")]
    pub PostRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    PostRestore: usize,
    #[cfg(feature = "shared")]
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetContext: usize,
    #[cfg(feature = "vss")]
    pub StartSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    StartSnapshotSet: usize,
    #[cfg(feature = "vss")]
    pub AddToSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_PWSZ, super::VSS_ID, *mut super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    AddToSnapshotSet: usize,
    #[cfg(feature = "vss")]
    pub DoSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    DoSnapshotSet: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub DeleteSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_OBJECT_TYPE, windows_core::BOOL, *const super::LONG, *const super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    DeleteSnapshots: usize,
    #[cfg(feature = "vss")]
    pub ImportSnapshots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    ImportSnapshots: usize,
    #[cfg(feature = "vss")]
    pub BreakSnapshotSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    BreakSnapshotSet: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub GetSnapshotProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, *mut super::VSS_SNAPSHOT_PROP) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    GetSnapshotProperties: usize,
    #[cfg(feature = "vss")]
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_OBJECT_TYPE, super::VSS_OBJECT_TYPE, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    Query: usize,
    #[cfg(feature = "vss")]
    pub IsVolumeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_PWSZ, *const windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    IsVolumeSupported: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub DisableWriterClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VSS_ID, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    DisableWriterClasses: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub EnableWriterClasses: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VSS_ID, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    EnableWriterClasses: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub DisableWriterInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::VSS_ID, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    DisableWriterInstances: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub ExposeSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_PWSZ, super::LONG, super::VSS_PWSZ, *mut super::VSS_PWSZ) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    ExposeSnapshot: usize,
    #[cfg(feature = "vss")]
    pub RevertToSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    RevertToSnapshot: usize,
    #[cfg(feature = "vss")]
    pub QueryRevertStatus: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_PWSZ, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    QueryRevertStatus: usize,
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssBackupComponents_Impl: windows_core::IUnknownImpl {
    fn GetWriterComponentsCount(&self, pccomponents: *mut super::UINT) -> windows_core::Result<()>;
    fn GetWriterComponents(&self, iwriter: super::UINT, ppwriter: *mut *mut IVssWriterComponentsExt) -> windows_core::Result<()>;
    fn InitializeForBackup(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetBackupState(&self, bselectcomponents: bool, bbackupbootablesystemstate: bool, backuptype: super::VSS_BACKUP_TYPE, bpartialfilesupport: bool) -> windows_core::Result<()>;
    fn InitializeForRestore(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreState(&self, restoretype: super::VSS_RESTORE_TYPE) -> windows_core::Result<()>;
    fn GatherWriterMetadata(&self, pasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn GetWriterMetadataCount(&self, pcwriters: *mut super::UINT) -> windows_core::Result<()>;
    fn GetWriterMetadata(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, ppmetadata: windows_core::OutRef<IVssExamineWriterMetadata>) -> windows_core::Result<()>;
    fn FreeWriterMetadata(&self) -> windows_core::Result<()>;
    fn AddComponent(&self, instanceid: &super::VSS_ID, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PrepareForBackup(&self, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn AbortBackup(&self) -> windows_core::Result<()>;
    fn GatherWriterStatus(&self, pasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn GetWriterStatusCount(&self, pcwriters: *mut super::UINT) -> windows_core::Result<()>;
    fn FreeWriterStatus(&self) -> windows_core::Result<()>;
    fn GetWriterStatus(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::VSS_WRITER_STATE, phresultfailure: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn SetBackupSucceeded(&self, instanceid: &super::VSS_ID, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bsucceded: bool) -> windows_core::Result<()>;
    fn SetBackupOptions(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszbackupoptions: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetSelectedForRestore(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bselectedforrestore: bool) -> windows_core::Result<()>;
    fn SetRestoreOptions(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszrestoreoptions: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAdditionalRestores(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, badditionalrestores: bool) -> windows_core::Result<()>;
    fn SetPreviousBackupStamp(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszpreviousbackupstamp: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::Result<()>;
    fn BackupComplete(&self, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn AddAlternativeLocationMapping(&self, writerid: &super::VSS_ID, componenttype: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool, wszdestination: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddRestoreSubcomponent(&self, writerid: &super::VSS_ID, componenttype: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszsubcomponentlogicalpath: &windows_core::PCWSTR, wszsubcomponentname: &windows_core::PCWSTR, brepair: bool) -> windows_core::Result<()>;
    fn SetFileRestoreStatus(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, status: super::VSS_FILE_RESTORE_STATUS) -> windows_core::Result<()>;
    fn AddNewTarget(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilename: &windows_core::PCWSTR, brecursive: bool, wszalternatepath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRangesFilePath(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, ipartialfile: super::UINT, wszrangesfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PreRestore(&self, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn PostRestore(&self, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn SetContext(&self, lcontext: super::LONG) -> windows_core::Result<()>;
    fn StartSnapshotSet(&self, psnapshotsetid: *mut super::VSS_ID) -> windows_core::Result<()>;
    fn AddToSnapshotSet(&self, pwszvolumename: super::VSS_PWSZ, providerid: &super::VSS_ID, pidsnapshot: *mut super::VSS_ID) -> windows_core::Result<()>;
    fn DoSnapshotSet(&self, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn DeleteSnapshots(&self, sourceobjectid: &super::VSS_ID, esourceobjecttype: super::VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *const super::LONG, pnondeletedsnapshotid: *const super::VSS_ID) -> windows_core::Result<()>;
    fn ImportSnapshots(&self, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn BreakSnapshotSet(&self, snapshotsetid: &super::VSS_ID) -> windows_core::Result<()>;
    fn GetSnapshotProperties(&self, snapshotid: &super::VSS_ID, pprop: *mut super::VSS_SNAPSHOT_PROP) -> windows_core::Result<()>;
    fn Query(&self, queriedobjectid: &super::VSS_ID, equeriedobjecttype: super::VSS_OBJECT_TYPE, ereturnedobjectstype: super::VSS_OBJECT_TYPE, ppenum: *const Option<super::IVssEnumObject>) -> windows_core::Result<()>;
    fn IsVolumeSupported(&self, providerid: &super::VSS_ID, pwszvolumename: super::VSS_PWSZ, pbsupportedbythisprovider: *const windows_core::BOOL) -> windows_core::Result<()>;
    fn DisableWriterClasses(&self, rgwriterclassid: *const super::VSS_ID, cclassid: super::UINT) -> windows_core::Result<()>;
    fn EnableWriterClasses(&self, rgwriterclassid: *const super::VSS_ID, cclassid: super::UINT) -> windows_core::Result<()>;
    fn DisableWriterInstances(&self, rgwriterinstanceid: *const super::VSS_ID, cinstanceid: super::UINT) -> windows_core::Result<()>;
    fn ExposeSnapshot(&self, snapshotid: &super::VSS_ID, wszpathfromroot: super::VSS_PWSZ, lattributes: super::LONG, wszexpose: super::VSS_PWSZ, pwszexposed: *mut super::VSS_PWSZ) -> windows_core::Result<()>;
    fn RevertToSnapshot(&self, snapshotid: &super::VSS_ID, bforcedismount: windows_core::BOOL) -> windows_core::Result<()>;
    fn QueryRevertStatus(&self, pwszvolume: super::VSS_PWSZ, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssBackupComponents_Vtbl {
    pub const fn new<Identity: IVssBackupComponents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterComponentsCount<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccomponents: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterComponentsCount(this, core::mem::transmute_copy(&pccomponents)).into()
            }
        }
        unsafe extern "system" fn GetWriterComponents<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: super::UINT, ppwriter: *mut *mut IVssWriterComponentsExt) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterComponents(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&ppwriter)).into()
            }
        }
        unsafe extern "system" fn InitializeForBackup<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrxml: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::InitializeForBackup(this, core::mem::transmute(&bstrxml)).into()
            }
        }
        unsafe extern "system" fn SetBackupState<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bselectcomponents: bool, bbackupbootablesystemstate: bool, backuptype: super::VSS_BACKUP_TYPE, bpartialfilesupport: bool) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetRestoreState<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restoretype: super::VSS_RESTORE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRestoreState(this, core::mem::transmute_copy(&restoretype)).into()
            }
        }
        unsafe extern "system" fn GatherWriterMetadata<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GatherWriterMetadata(this, core::mem::transmute_copy(&pasync)).into()
            }
        }
        unsafe extern "system" fn GetWriterMetadataCount<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwriters: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterMetadataCount(this, core::mem::transmute_copy(&pcwriters)).into()
            }
        }
        unsafe extern "system" fn GetWriterMetadata<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddComponent<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instanceid: super::VSS_ID, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddComponent(this, core::mem::transmute(&instanceid), core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname)).into()
            }
        }
        unsafe extern "system" fn PrepareForBackup<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::PrepareForBackup(this, core::mem::transmute_copy(&ppasync)).into()
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
                IVssBackupComponents_Impl::GatherWriterStatus(this, core::mem::transmute_copy(&pasync)).into()
            }
        }
        unsafe extern "system" fn GetWriterStatusCount<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwriters: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterStatusCount(this, core::mem::transmute_copy(&pcwriters)).into()
            }
        }
        unsafe extern "system" fn FreeWriterStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::FreeWriterStatus(this).into()
            }
        }
        unsafe extern "system" fn GetWriterStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwriter: *mut *mut core::ffi::c_void, pnstatus: *mut super::VSS_WRITER_STATE, phresultfailure: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetWriterStatus(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwriter), core::mem::transmute_copy(&pnstatus), core::mem::transmute_copy(&phresultfailure)).into()
            }
        }
        unsafe extern "system" fn SetBackupSucceeded<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instanceid: super::VSS_ID, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bsucceded: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupSucceeded(this, core::mem::transmute(&instanceid), core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bsucceded)).into()
            }
        }
        unsafe extern "system" fn SetBackupOptions<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszbackupoptions: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetBackupOptions(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszbackupoptions)).into()
            }
        }
        unsafe extern "system" fn SetSelectedForRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bselectedforrestore: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetSelectedForRestore(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bselectedforrestore)).into()
            }
        }
        unsafe extern "system" fn SetRestoreOptions<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszrestoreoptions: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRestoreOptions(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszrestoreoptions)).into()
            }
        }
        unsafe extern "system" fn SetAdditionalRestores<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, badditionalrestores: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetAdditionalRestores(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&badditionalrestores)).into()
            }
        }
        unsafe extern "system" fn SetPreviousBackupStamp<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszpreviousbackupstamp: windows_core::PCWSTR) -> windows_core::HRESULT {
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
                IVssBackupComponents_Impl::BackupComplete(this, core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn AddAlternativeLocationMapping<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, componenttype: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool, wszdestination: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddAlternativeLocationMapping(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&componenttype), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszdestination)).into()
            }
        }
        unsafe extern "system" fn AddRestoreSubcomponent<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, componenttype: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszsubcomponentlogicalpath: windows_core::PCWSTR, wszsubcomponentname: windows_core::PCWSTR, brepair: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddRestoreSubcomponent(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&componenttype), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszsubcomponentlogicalpath), core::mem::transmute(&wszsubcomponentname), core::mem::transmute_copy(&brepair)).into()
            }
        }
        unsafe extern "system" fn SetFileRestoreStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, status: super::VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetFileRestoreStatus(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn AddNewTarget<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilename: windows_core::PCWSTR, brecursive: bool, wszalternatepath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddNewTarget(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilename), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatepath)).into()
            }
        }
        unsafe extern "system" fn SetRangesFilePath<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, ipartialfile: super::UINT, wszrangesfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetRangesFilePath(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&ipartialfile), core::mem::transmute(&wszrangesfile)).into()
            }
        }
        unsafe extern "system" fn PreRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::PreRestore(this, core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn PostRestore<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::PostRestore(this, core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn SetContext<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lcontext: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::SetContext(this, core::mem::transmute_copy(&lcontext)).into()
            }
        }
        unsafe extern "system" fn StartSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psnapshotsetid: *mut super::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::StartSnapshotSet(this, core::mem::transmute_copy(&psnapshotsetid)).into()
            }
        }
        unsafe extern "system" fn AddToSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolumename: super::VSS_PWSZ, providerid: super::VSS_ID, pidsnapshot: *mut super::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::AddToSnapshotSet(this, core::mem::transmute_copy(&pwszvolumename), core::mem::transmute(&providerid), core::mem::transmute_copy(&pidsnapshot)).into()
            }
        }
        unsafe extern "system" fn DoSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DoSnapshotSet(this, core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn DeleteSnapshots<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceobjectid: super::VSS_ID, esourceobjecttype: super::VSS_OBJECT_TYPE, bforcedelete: windows_core::BOOL, pldeletedsnapshots: *const super::LONG, pnondeletedsnapshotid: *const super::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DeleteSnapshots(this, core::mem::transmute(&sourceobjectid), core::mem::transmute_copy(&esourceobjecttype), core::mem::transmute_copy(&bforcedelete), core::mem::transmute_copy(&pldeletedsnapshots), core::mem::transmute_copy(&pnondeletedsnapshotid)).into()
            }
        }
        unsafe extern "system" fn ImportSnapshots<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::ImportSnapshots(this, core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn BreakSnapshotSet<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::BreakSnapshotSet(this, core::mem::transmute(&snapshotsetid)).into()
            }
        }
        unsafe extern "system" fn GetSnapshotProperties<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::VSS_ID, pprop: *mut super::VSS_SNAPSHOT_PROP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::GetSnapshotProperties(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&pprop)).into()
            }
        }
        unsafe extern "system" fn Query<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, queriedobjectid: super::VSS_ID, equeriedobjecttype: super::VSS_OBJECT_TYPE, ereturnedobjectstype: super::VSS_OBJECT_TYPE, ppenum: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::Query(this, core::mem::transmute(&queriedobjectid), core::mem::transmute_copy(&equeriedobjecttype), core::mem::transmute_copy(&ereturnedobjectstype), core::mem::transmute_copy(&ppenum)).into()
            }
        }
        unsafe extern "system" fn IsVolumeSupported<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, providerid: super::VSS_ID, pwszvolumename: super::VSS_PWSZ, pbsupportedbythisprovider: *const windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::IsVolumeSupported(this, core::mem::transmute(&providerid), core::mem::transmute_copy(&pwszvolumename), core::mem::transmute_copy(&pbsupportedbythisprovider)).into()
            }
        }
        unsafe extern "system" fn DisableWriterClasses<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgwriterclassid: *const super::VSS_ID, cclassid: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DisableWriterClasses(this, core::mem::transmute_copy(&rgwriterclassid), core::mem::transmute_copy(&cclassid)).into()
            }
        }
        unsafe extern "system" fn EnableWriterClasses<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgwriterclassid: *const super::VSS_ID, cclassid: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::EnableWriterClasses(this, core::mem::transmute_copy(&rgwriterclassid), core::mem::transmute_copy(&cclassid)).into()
            }
        }
        unsafe extern "system" fn DisableWriterInstances<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rgwriterinstanceid: *const super::VSS_ID, cinstanceid: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::DisableWriterInstances(this, core::mem::transmute_copy(&rgwriterinstanceid), core::mem::transmute_copy(&cinstanceid)).into()
            }
        }
        unsafe extern "system" fn ExposeSnapshot<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::VSS_ID, wszpathfromroot: super::VSS_PWSZ, lattributes: super::LONG, wszexpose: super::VSS_PWSZ, pwszexposed: *mut super::VSS_PWSZ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::ExposeSnapshot(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&wszpathfromroot), core::mem::transmute_copy(&lattributes), core::mem::transmute_copy(&wszexpose), core::mem::transmute_copy(&pwszexposed)).into()
            }
        }
        unsafe extern "system" fn RevertToSnapshot<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::VSS_ID, bforcedismount: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::RevertToSnapshot(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&bforcedismount)).into()
            }
        }
        unsafe extern "system" fn QueryRevertStatus<Identity: IVssBackupComponents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszvolume: super::VSS_PWSZ, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponents_Impl::QueryRevertStatus(this, core::mem::transmute_copy(&pwszvolume), core::mem::transmute_copy(&ppasync)).into()
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
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
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
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn GetWriterMetadataEx(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, ppmetadata: *mut Option<IVssExamineWriterMetadataEx>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterMetadataEx)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, core::mem::transmute(ppmetadata)) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetSelectedForRestoreEx<P2, P3>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, bselectedforrestore: bool, instanceid: super::VSS_ID) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedForRestoreEx)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bselectedforrestore, instanceid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx_Vtbl {
    pub base__: IVssBackupComponents_Vtbl,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub GetWriterMetadataEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::VSS_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    GetWriterMetadataEx: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetSelectedForRestoreEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool, super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetSelectedForRestoreEx: usize,
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssBackupComponentsEx_Impl: IVssBackupComponents_Impl {
    fn GetWriterMetadataEx(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, ppmetadata: windows_core::OutRef<IVssExamineWriterMetadataEx>) -> windows_core::Result<()>;
    fn SetSelectedForRestoreEx(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bselectedforrestore: bool, instanceid: &super::VSS_ID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssBackupComponentsEx_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterMetadataEx<Identity: IVssBackupComponentsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx_Impl::GetWriterMetadataEx(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&ppmetadata)).into()
            }
        }
        unsafe extern "system" fn SetSelectedForRestoreEx<Identity: IVssBackupComponentsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bselectedforrestore: bool, instanceid: super::VSS_ID) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
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
    #[cfg(feature = "vss")]
    pub unsafe fn UnexposeSnapshot(&self, snapshotid: super::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnexposeSnapshot)(windows_core::Interface::as_raw(self), snapshotid) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetAuthoritativeRestore<P2, P3>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, bauth: bool) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAuthoritativeRestore)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), bauth) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetRollForward<P2, P3, P5>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, rolltype: super::VSS_ROLLFORWARD_TYPE, wszrollforwardpoint: P5) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRollForward)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), rolltype, wszrollforwardpoint.param().abi()) }
    }
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn SetRestoreName<P2, P3, P4>(&self, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: P2, wszcomponentname: P3, wszrestorename: P4) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreName)(windows_core::Interface::as_raw(self), writerid, ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszrestorename.param().abi()) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn BreakSnapshotSetEx(&self, snapshotsetid: super::VSS_ID, dwbreakflags: super::DWORD, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BreakSnapshotSetEx)(windows_core::Interface::as_raw(self), snapshotsetid, dwbreakflags, core::mem::transmute(ppasync)) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn PreFastRecovery(&self, snapshotsetid: super::VSS_ID, dwprefastrecoveryflags: super::DWORD, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreFastRecovery)(windows_core::Interface::as_raw(self), snapshotsetid, dwprefastrecoveryflags, core::mem::transmute(ppasync)) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn FastRecovery(&self, snapshotsetid: super::VSS_ID, dwfastrecoveryflags: super::DWORD, ppasync: *mut Option<super::IVssAsync>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FastRecovery)(windows_core::Interface::as_raw(self), snapshotsetid, dwfastrecoveryflags, core::mem::transmute(ppasync)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx2_Vtbl {
    pub base__: IVssBackupComponentsEx_Vtbl,
    #[cfg(feature = "vss")]
    pub UnexposeSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    UnexposeSnapshot: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetAuthoritativeRestore: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetAuthoritativeRestore: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetRollForward: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, super::VSS_ROLLFORWARD_TYPE, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetRollForward: usize,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub SetRestoreName: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    SetRestoreName: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub BreakSnapshotSetEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    BreakSnapshotSetEx: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub PreFastRecovery: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    PreFastRecovery: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub FastRecovery: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    FastRecovery: usize,
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssBackupComponentsEx2_Impl: IVssBackupComponentsEx_Impl {
    fn UnexposeSnapshot(&self, snapshotid: &super::VSS_ID) -> windows_core::Result<()>;
    fn SetAuthoritativeRestore(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, bauth: bool) -> windows_core::Result<()>;
    fn SetRollForward(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, rolltype: super::VSS_ROLLFORWARD_TYPE, wszrollforwardpoint: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRestoreName(&self, writerid: &super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszrestorename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BreakSnapshotSetEx(&self, snapshotsetid: &super::VSS_ID, dwbreakflags: super::DWORD, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn PreFastRecovery(&self, snapshotsetid: &super::VSS_ID, dwprefastrecoveryflags: super::DWORD, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
    fn FastRecovery(&self, snapshotsetid: &super::VSS_ID, dwfastrecoveryflags: super::DWORD, ppasync: windows_core::OutRef<super::IVssAsync>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssBackupComponentsEx2_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnexposeSnapshot<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::UnexposeSnapshot(this, core::mem::transmute(&snapshotid)).into()
            }
        }
        unsafe extern "system" fn SetAuthoritativeRestore<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, bauth: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::SetAuthoritativeRestore(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&bauth)).into()
            }
        }
        unsafe extern "system" fn SetRollForward<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, rolltype: super::VSS_ROLLFORWARD_TYPE, wszrollforwardpoint: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::SetRollForward(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute_copy(&rolltype), core::mem::transmute(&wszrollforwardpoint)).into()
            }
        }
        unsafe extern "system" fn SetRestoreName<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::VSS_ID, ct: super::VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszrestorename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::SetRestoreName(this, core::mem::transmute(&writerid), core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszrestorename)).into()
            }
        }
        unsafe extern "system" fn BreakSnapshotSetEx<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::VSS_ID, dwbreakflags: super::DWORD, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::BreakSnapshotSetEx(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&dwbreakflags), core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn PreFastRecovery<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::VSS_ID, dwprefastrecoveryflags: super::DWORD, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::PreFastRecovery(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&dwprefastrecoveryflags), core::mem::transmute_copy(&ppasync)).into()
            }
        }
        unsafe extern "system" fn FastRecovery<Identity: IVssBackupComponentsEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotsetid: super::VSS_ID, dwfastrecoveryflags: super::DWORD, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx2_Impl::FastRecovery(this, core::mem::transmute(&snapshotsetid), core::mem::transmute_copy(&dwfastrecoveryflags), core::mem::transmute_copy(&ppasync)).into()
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
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
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
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn GetWriterStatusEx(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::VSS_WRITER_STATE, phrfailurewriter: *mut windows_core::HRESULT, phrapplication: Option<*mut windows_core::HRESULT>, pbstrapplicationmessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterStatusEx)(windows_core::Interface::as_raw(self), iwriter, pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwriter), pnstatus as _, phrfailurewriter as _, phrapplication.unwrap_or(core::mem::zeroed()) as _, pbstrapplicationmessage.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn AddSnapshotToRecoverySet(&self, snapshotid: super::VSS_ID, dwflags: super::DWORD, pwszdestinationvolume: Option<super::VSS_PWSZ>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddSnapshotToRecoverySet)(windows_core::Interface::as_raw(self), snapshotid, dwflags, pwszdestinationvolume.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub unsafe fn RecoverSet(&self, dwflags: super::DWORD) -> windows_core::Result<super::IVssAsync> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RecoverSet)(windows_core::Interface::as_raw(self), dwflags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "vss")]
    pub unsafe fn GetSessionId(&self, idsession: *mut super::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSessionId)(windows_core::Interface::as_raw(self), idsession as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx3_Vtbl {
    pub base__: IVssBackupComponentsEx2_Vtbl,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub GetWriterStatusEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::VSS_ID, *mut super::VSS_ID, *mut *mut core::ffi::c_void, *mut super::VSS_WRITER_STATE, *mut windows_core::HRESULT, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    GetWriterStatusEx: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub AddSnapshotToRecoverySet: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_ID, super::DWORD, super::VSS_PWSZ) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    AddSnapshotToRecoverySet: usize,
    #[cfg(all(feature = "shared", feature = "vss"))]
    pub RecoverSet: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vss")))]
    RecoverSet: usize,
    #[cfg(feature = "vss")]
    pub GetSessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    GetSessionId: usize,
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssBackupComponentsEx3_Impl: IVssBackupComponentsEx2_Impl {
    fn GetWriterStatusEx(&self, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwriter: *mut windows_core::BSTR, pnstatus: *mut super::VSS_WRITER_STATE, phrfailurewriter: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn AddSnapshotToRecoverySet(&self, snapshotid: &super::VSS_ID, dwflags: super::DWORD, pwszdestinationvolume: super::VSS_PWSZ) -> windows_core::Result<()>;
    fn RecoverSet(&self, dwflags: super::DWORD) -> windows_core::Result<super::IVssAsync>;
    fn GetSessionId(&self, idsession: *mut super::VSS_ID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssBackupComponentsEx3_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterStatusEx<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iwriter: super::UINT, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwriter: *mut *mut core::ffi::c_void, pnstatus: *mut super::VSS_WRITER_STATE, phrfailurewriter: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx3_Impl::GetWriterStatusEx(this, core::mem::transmute_copy(&iwriter), core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwriter), core::mem::transmute_copy(&pnstatus), core::mem::transmute_copy(&phrfailurewriter), core::mem::transmute_copy(&phrapplication), core::mem::transmute_copy(&pbstrapplicationmessage)).into()
            }
        }
        unsafe extern "system" fn AddSnapshotToRecoverySet<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, snapshotid: super::VSS_ID, dwflags: super::DWORD, pwszdestinationvolume: super::VSS_PWSZ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx3_Impl::AddSnapshotToRecoverySet(this, core::mem::transmute(&snapshotid), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pwszdestinationvolume)).into()
            }
        }
        unsafe extern "system" fn RecoverSet<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: super::DWORD, ppasync: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetSessionId<Identity: IVssBackupComponentsEx3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsession: *mut super::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssBackupComponentsEx3_Impl::GetSessionId(this, core::mem::transmute_copy(&idsession)).into()
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
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
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
    #[cfg(feature = "vss")]
    pub unsafe fn GetRootAndLogicalPrefixPaths(&self, pwszfilepath: super::VSS_PWSZ, ppwszrootpath: *mut super::VSS_PWSZ, ppwszlogicalprefix: *mut super::VSS_PWSZ, bnormalizefqdnforrootpath: Option<windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRootAndLogicalPrefixPaths)(windows_core::Interface::as_raw(self), pwszfilepath, ppwszrootpath as _, ppwszlogicalprefix as _, bnormalizefqdnforrootpath.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssBackupComponentsEx4_Vtbl {
    pub base__: IVssBackupComponentsEx3_Vtbl,
    #[cfg(feature = "vss")]
    pub GetRootAndLogicalPrefixPaths: unsafe extern "system" fn(*mut core::ffi::c_void, super::VSS_PWSZ, *mut super::VSS_PWSZ, *mut super::VSS_PWSZ, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "vss"))]
    GetRootAndLogicalPrefixPaths: usize,
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssBackupComponentsEx4_Impl: IVssBackupComponentsEx3_Impl {
    fn GetRootAndLogicalPrefixPaths(&self, pwszfilepath: super::VSS_PWSZ, ppwszrootpath: *mut super::VSS_PWSZ, ppwszlogicalprefix: *mut super::VSS_PWSZ, bnormalizefqdnforrootpath: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssBackupComponentsEx4_Vtbl {
    pub const fn new<Identity: IVssBackupComponentsEx4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRootAndLogicalPrefixPaths<Identity: IVssBackupComponentsEx4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilepath: super::VSS_PWSZ, ppwszrootpath: *mut super::VSS_PWSZ, ppwszlogicalprefix: *mut super::VSS_PWSZ, bnormalizefqdnforrootpath: windows_core::BOOL) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "vss", feature = "vswriter"))]
impl windows_core::RuntimeName for IVssBackupComponentsEx4 {}
windows_core::imp::define_interface!(IVssExamineWriterMetadata, IVssExamineWriterMetadata_Vtbl, 0x902fcf7f_b7fd_42f8_81f1_b2e400b1e5bd);
windows_core::imp::interface_hierarchy!(IVssExamineWriterMetadata, windows_core::IUnknown);
impl IVssExamineWriterMetadata {
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn GetIdentity(&self, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pusage: *mut super::VSS_USAGE_TYPE, psource: *mut super::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentity)(windows_core::Interface::as_raw(self), pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwritername), pusage as _, psource as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFileCounts(&self, pcincludefiles: *mut super::UINT, pcexcludefiles: *mut super::UINT, pccomponents: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFileCounts)(windows_core::Interface::as_raw(self), pcincludefiles as _, pcexcludefiles as _, pccomponents as _) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetIncludeFile(&self, ifile: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIncludeFile)(windows_core::Interface::as_raw(self), ifile, core::mem::transmute(ppfiledesc)) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetExcludeFile(&self, ifile: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExcludeFile)(windows_core::Interface::as_raw(self), ifile, core::mem::transmute(ppfiledesc)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetComponent(&self, icomponent: super::UINT, ppcomponent: *mut Option<IVssWMComponent>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetComponent)(windows_core::Interface::as_raw(self), icomponent, core::mem::transmute(ppcomponent)) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetRestoreMethod(&self, pmethod: *mut super::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut windows_core::BSTR, pbstruserprocedure: *mut windows_core::BSTR, pwriterrestore: *mut super::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreMethod)(windows_core::Interface::as_raw(self), pmethod as _, core::mem::transmute(pbstrservice), core::mem::transmute(pbstruserprocedure), pwriterrestore as _, pbrebootrequired as _, pcmappings as _) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAlternateLocationMapping)(windows_core::Interface::as_raw(self), imapping, core::mem::transmute(ppfiledesc)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBackupSchema(&self, pdwschemamask: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBackupSchema)(windows_core::Interface::as_raw(self), pdwschemamask as _) }
    }
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub unsafe fn GetDocument(&self, pdoc: *mut Option<super::IXMLDOMDocument>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), core::mem::transmute(pdoc)) }
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
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub GetIdentity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VSS_ID, *mut super::VSS_ID, *mut *mut core::ffi::c_void, *mut super::VSS_USAGE_TYPE, *mut super::VSS_SOURCE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    GetIdentity: usize,
    #[cfg(feature = "shared")]
    pub GetFileCounts: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT, *mut super::UINT, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFileCounts: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetIncludeFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetIncludeFile: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetExcludeFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetExcludeFile: usize,
    #[cfg(feature = "shared")]
    pub GetComponent: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetComponent: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VSS_RESTOREMETHOD_ENUM, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::VSS_WRITERRESTORE_ENUM, *mut bool, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetRestoreMethod: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetAlternateLocationMapping: usize,
    #[cfg(feature = "shared")]
    pub GetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetBackupSchema: usize,
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "msxml", feature = "oaidl")))]
    GetDocument: usize,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadFromXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssExamineWriterMetadata_Impl: windows_core::IUnknownImpl {
    fn GetIdentity(&self, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pusage: *mut super::VSS_USAGE_TYPE, psource: *mut super::VSS_SOURCE_TYPE) -> windows_core::Result<()>;
    fn GetFileCounts(&self, pcincludefiles: *mut super::UINT, pcexcludefiles: *mut super::UINT, pccomponents: *mut super::UINT) -> windows_core::Result<()>;
    fn GetIncludeFile(&self, ifile: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
    fn GetExcludeFile(&self, ifile: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
    fn GetComponent(&self, icomponent: super::UINT, ppcomponent: windows_core::OutRef<IVssWMComponent>) -> windows_core::Result<()>;
    fn GetRestoreMethod(&self, pmethod: *mut super::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut windows_core::BSTR, pbstruserprocedure: *mut windows_core::BSTR, pwriterrestore: *mut super::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut super::UINT) -> windows_core::Result<()>;
    fn GetAlternateLocationMapping(&self, imapping: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
    fn GetBackupSchema(&self, pdwschemamask: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetDocument(&self, pdoc: windows_core::OutRef<super::IXMLDOMDocument>) -> windows_core::Result<()>;
    fn SaveAsXML(&self, pbstrxml: *const windows_core::BSTR) -> windows_core::Result<()>;
    fn LoadFromXML(&self, bstrxml: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssExamineWriterMetadata_Vtbl {
    pub const fn new<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentity<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwritername: *mut *mut core::ffi::c_void, pusage: *mut super::VSS_USAGE_TYPE, psource: *mut super::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetIdentity(this, core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter), core::mem::transmute_copy(&pbstrwritername), core::mem::transmute_copy(&pusage), core::mem::transmute_copy(&psource)).into()
            }
        }
        unsafe extern "system" fn GetFileCounts<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcincludefiles: *mut super::UINT, pcexcludefiles: *mut super::UINT, pccomponents: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetFileCounts(this, core::mem::transmute_copy(&pcincludefiles), core::mem::transmute_copy(&pcexcludefiles), core::mem::transmute_copy(&pccomponents)).into()
            }
        }
        unsafe extern "system" fn GetIncludeFile<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetIncludeFile(this, core::mem::transmute_copy(&ifile), core::mem::transmute_copy(&ppfiledesc)).into()
            }
        }
        unsafe extern "system" fn GetExcludeFile<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetExcludeFile(this, core::mem::transmute_copy(&ifile), core::mem::transmute_copy(&ppfiledesc)).into()
            }
        }
        unsafe extern "system" fn GetComponent<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: super::UINT, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetComponent(this, core::mem::transmute_copy(&icomponent), core::mem::transmute_copy(&ppcomponent)).into()
            }
        }
        unsafe extern "system" fn GetRestoreMethod<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmethod: *mut super::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut *mut core::ffi::c_void, pbstruserprocedure: *mut *mut core::ffi::c_void, pwriterrestore: *mut super::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetRestoreMethod(this, core::mem::transmute_copy(&pmethod), core::mem::transmute_copy(&pbstrservice), core::mem::transmute_copy(&pbstruserprocedure), core::mem::transmute_copy(&pwriterrestore), core::mem::transmute_copy(&pbrebootrequired), core::mem::transmute_copy(&pcmappings)).into()
            }
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetAlternateLocationMapping(this, core::mem::transmute_copy(&imapping), core::mem::transmute_copy(&ppfiledesc)).into()
            }
        }
        unsafe extern "system" fn GetBackupSchema<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwschemamask: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetBackupSchema(this, core::mem::transmute_copy(&pdwschemamask)).into()
            }
        }
        unsafe extern "system" fn GetDocument<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetDocument(this, core::mem::transmute_copy(&pdoc)).into()
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
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
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
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub unsafe fn GetIdentityEx(&self, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pbstrinstancename: *mut windows_core::BSTR, pusage: *mut super::VSS_USAGE_TYPE, psource: *mut super::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdentityEx)(windows_core::Interface::as_raw(self), pidinstance as _, pidwriter as _, core::mem::transmute(pbstrwritername), core::mem::transmute(pbstrinstancename), pusage as _, psource as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExamineWriterMetadataEx_Vtbl {
    pub base__: IVssExamineWriterMetadata_Vtbl,
    #[cfg(all(feature = "vss", feature = "vswriter"))]
    pub GetIdentityEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VSS_ID, *mut super::VSS_ID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::VSS_USAGE_TYPE, *mut super::VSS_SOURCE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "vss", feature = "vswriter")))]
    GetIdentityEx: usize,
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssExamineWriterMetadataEx_Impl: IVssExamineWriterMetadata_Impl {
    fn GetIdentityEx(&self, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwritername: *mut windows_core::BSTR, pbstrinstancename: *mut windows_core::BSTR, pusage: *mut super::VSS_USAGE_TYPE, psource: *mut super::VSS_SOURCE_TYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssExamineWriterMetadataEx_Vtbl {
    pub const fn new<Identity: IVssExamineWriterMetadataEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetIdentityEx<Identity: IVssExamineWriterMetadataEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidinstance: *mut super::VSS_ID, pidwriter: *mut super::VSS_ID, pbstrwritername: *mut *mut core::ffi::c_void, pbstrinstancename: *mut *mut core::ffi::c_void, pusage: *mut super::VSS_USAGE_TYPE, psource: *mut super::VSS_SOURCE_TYPE) -> windows_core::HRESULT {
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
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
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
    #[cfg(feature = "shared")]
    pub unsafe fn GetVersion(&self, pdwmajorversion: *mut super::DWORD, pdwminorversion: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVersion)(windows_core::Interface::as_raw(self), pdwmajorversion as _, pdwminorversion as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetExcludeFromSnapshotCount(&self, pcexcludedfromsnapshot: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExcludeFromSnapshotCount)(windows_core::Interface::as_raw(self), pcexcludedfromsnapshot as _) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetExcludeFromSnapshotFile(&self, ifile: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExcludeFromSnapshotFile)(windows_core::Interface::as_raw(self), ifile, core::mem::transmute(ppfiledesc)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExamineWriterMetadataEx2_Vtbl {
    pub base__: IVssExamineWriterMetadataEx_Vtbl,
    #[cfg(feature = "shared")]
    pub GetVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetVersion: usize,
    #[cfg(feature = "shared")]
    pub GetExcludeFromSnapshotCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetExcludeFromSnapshotCount: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetExcludeFromSnapshotFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetExcludeFromSnapshotFile: usize,
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
pub trait IVssExamineWriterMetadataEx2_Impl: IVssExamineWriterMetadataEx_Impl {
    fn GetVersion(&self, pdwmajorversion: *mut super::DWORD, pdwminorversion: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetExcludeFromSnapshotCount(&self, pcexcludedfromsnapshot: *mut super::UINT) -> windows_core::Result<()>;
    fn GetExcludeFromSnapshotFile(&self, ifile: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
impl IVssExamineWriterMetadataEx2_Vtbl {
    pub const fn new<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersion<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmajorversion: *mut super::DWORD, pdwminorversion: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadataEx2_Impl::GetVersion(this, core::mem::transmute_copy(&pdwmajorversion), core::mem::transmute_copy(&pdwminorversion)).into()
            }
        }
        unsafe extern "system" fn GetExcludeFromSnapshotCount<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcexcludedfromsnapshot: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadataEx2_Impl::GetExcludeFromSnapshotCount(this, core::mem::transmute_copy(&pcexcludedfromsnapshot)).into()
            }
        }
        unsafe extern "system" fn GetExcludeFromSnapshotFile<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadataEx2_Impl::GetExcludeFromSnapshotFile(this, core::mem::transmute_copy(&ifile), core::mem::transmute_copy(&ppfiledesc)).into()
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
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
impl windows_core::RuntimeName for IVssExamineWriterMetadataEx2 {}
windows_core::imp::define_interface!(IVssWMComponent, IVssWMComponent_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMComponent, windows_core::IUnknown);
impl IVssWMComponent {
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetComponentInfo(&self, ppinfo: *mut PVSSCOMPONENTINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetComponentInfo)(windows_core::Interface::as_raw(self), ppinfo as _) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn FreeComponentInfo(&self, pinfo: PVSSCOMPONENTINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeComponentInfo)(windows_core::Interface::as_raw(self), pinfo) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetFile(&self, ifile: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), ifile, core::mem::transmute(ppfiledesc)) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetDatabaseFile(&self, idbfile: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDatabaseFile)(windows_core::Interface::as_raw(self), idbfile, core::mem::transmute(ppfiledesc)) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetDatabaseLogFile(&self, idblogfile: super::UINT, ppfiledesc: *mut Option<super::IVssWMFiledesc>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDatabaseLogFile)(windows_core::Interface::as_raw(self), idblogfile, core::mem::transmute(ppfiledesc)) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetDependency(&self, idependency: super::UINT, ppdependency: *mut Option<super::IVssWMDependency>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDependency)(windows_core::Interface::as_raw(self), idependency, core::mem::transmute(ppdependency)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetComponentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PVSSCOMPONENTINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetComponentInfo: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub FreeComponentInfo: unsafe extern "system" fn(*mut core::ffi::c_void, PVSSCOMPONENTINFO) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    FreeComponentInfo: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetFile: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetDatabaseFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetDatabaseFile: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetDatabaseLogFile: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetDatabaseLogFile: usize,
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub GetDependency: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "vswriter")))]
    GetDependency: usize,
}
#[cfg(all(feature = "shared", feature = "vswriter"))]
pub trait IVssWMComponent_Impl: windows_core::IUnknownImpl {
    fn GetComponentInfo(&self, ppinfo: *mut PVSSCOMPONENTINFO) -> windows_core::Result<()>;
    fn FreeComponentInfo(&self, pinfo: PVSSCOMPONENTINFO) -> windows_core::Result<()>;
    fn GetFile(&self, ifile: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
    fn GetDatabaseFile(&self, idbfile: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
    fn GetDatabaseLogFile(&self, idblogfile: super::UINT, ppfiledesc: windows_core::OutRef<super::IVssWMFiledesc>) -> windows_core::Result<()>;
    fn GetDependency(&self, idependency: super::UINT, ppdependency: windows_core::OutRef<super::IVssWMDependency>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "vswriter"))]
impl IVssWMComponent_Vtbl {
    pub const fn new<Identity: IVssWMComponent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetComponentInfo<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppinfo: *mut PVSSCOMPONENTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::GetComponentInfo(this, core::mem::transmute_copy(&ppinfo)).into()
            }
        }
        unsafe extern "system" fn FreeComponentInfo<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: PVSSCOMPONENTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::FreeComponentInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn GetFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::GetFile(this, core::mem::transmute_copy(&ifile), core::mem::transmute_copy(&ppfiledesc)).into()
            }
        }
        unsafe extern "system" fn GetDatabaseFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idbfile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::GetDatabaseFile(this, core::mem::transmute_copy(&idbfile), core::mem::transmute_copy(&ppfiledesc)).into()
            }
        }
        unsafe extern "system" fn GetDatabaseLogFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idblogfile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::GetDatabaseLogFile(this, core::mem::transmute_copy(&idblogfile), core::mem::transmute_copy(&ppfiledesc)).into()
            }
        }
        unsafe extern "system" fn GetDependency<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idependency: super::UINT, ppdependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::GetDependency(this, core::mem::transmute_copy(&idependency), core::mem::transmute_copy(&ppdependency)).into()
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
#[cfg(all(feature = "shared", feature = "vswriter"))]
impl windows_core::RuntimeName for IVssWMComponent {}
#[cfg(feature = "vswriter")]
#[repr(C)]
pub struct IVssWriterComponentsExt {
    base0: *const core::ffi::c_void,
    base1: *const core::ffi::c_void,
}
#[cfg(feature = "vswriter")]
impl IVssWriterComponentsExt {
    pub fn base0(&self) -> windows_core::InterfaceRef<'_, super::IVssWriterComponents> {
        unsafe { windows_core::InterfaceRef::from_raw(core::ptr::NonNull::new_unchecked(core::ptr::addr_of!(self.base0).cast_mut().cast())) }
    }
    pub fn base1(&self) -> windows_core::InterfaceRef<'_, windows_core::IUnknown> {
        unsafe { windows_core::InterfaceRef::from_raw(core::ptr::NonNull::new_unchecked(core::ptr::addr_of!(self.base1).cast_mut().cast())) }
    }
}
#[cfg(all(feature = "shared", feature = "vswriter"))]
pub type PVSSCOMPONENTINFO = *const VSS_COMPONENTINFO;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "vswriter"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VSS_COMPONENTINFO {
    pub r#type: super::VSS_COMPONENT_TYPE,
    pub bstrLogicalPath: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrComponentName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrCaption: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub pbIcon: *mut super::BYTE,
    pub cbIcon: super::UINT,
    pub bRestoreMetadata: bool,
    pub bNotifyOnBackupComplete: bool,
    pub bSelectable: bool,
    pub bSelectableForRestore: bool,
    pub dwComponentFlags: super::DWORD,
    pub cFileCount: super::UINT,
    pub cDatabases: super::UINT,
    pub cLogFiles: super::UINT,
    pub cDependencies: super::UINT,
}
pub const VSS_SW_BOOTABLE_STATE: i32 = 1;
