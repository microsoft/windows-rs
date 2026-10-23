#[inline]
pub unsafe fn CreateVssExamineWriterMetadataInternal(bstrxml: &windows_core::BSTR) -> windows_core::Result<IVssExamineWriterMetadata> {
    windows_core::link!("vssapi.dll" "system" fn CreateVssExamineWriterMetadataInternal(bstrxml : *mut core::ffi::c_void, ppmetadata : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateVssExamineWriterMetadataInternal(core::mem::transmute_copy(bstrxml), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "guiddef", feature = "vss"))]
#[inline]
pub unsafe fn GetProviderMgmtInterfaceInternal(providerid: super::VSS_ID, interfaceid: super::IID) -> windows_core::Result<windows_core::IUnknown> {
    windows_core::link!("vssapi.dll" "system" fn GetProviderMgmtInterfaceInternal(providerid : super::VSS_ID, interfaceid : super::IID, ppitf : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetProviderMgmtInterfaceInternal(providerid, interfaceid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "vss"))]
#[inline]
pub unsafe fn IsVolumeSnapshottedInternal(pwszvolumename: super::VSS_PWSZ, pbsnapshotspresent: *mut windows_core::BOOL, plsnapshotcapability: *mut super::LONG) -> windows_core::HRESULT {
    windows_core::link!("vssapi.dll" "system" fn IsVolumeSnapshottedInternal(pwszvolumename : super::VSS_PWSZ, pbsnapshotspresent : *mut windows_core::BOOL, plsnapshotcapability : *mut super::LONG) -> windows_core::HRESULT);
    unsafe { IsVolumeSnapshottedInternal(pwszvolumename, pbsnapshotspresent as _, plsnapshotcapability as _) }
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
    pub unsafe fn GetIncludeFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIncludeFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetExcludeFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludeFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetComponent(&self, icomponent: super::UINT) -> windows_core::Result<IVssWMComponent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponent)(windows_core::Interface::as_raw(self), icomponent, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetRestoreMethod(&self, pmethod: *mut super::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut windows_core::BSTR, pbstruserprocedure: *mut windows_core::BSTR, pwriterrestore: *mut super::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreMethod)(windows_core::Interface::as_raw(self), pmethod as _, core::mem::transmute(pbstrservice), core::mem::transmute(pbstruserprocedure), pwriterrestore as _, pbrebootrequired as _, pcmappings as _) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocationMapping)(windows_core::Interface::as_raw(self), imapping, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetBackupSchema(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupSchema)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "msxml", feature = "oaidl"))]
    pub unsafe fn GetDocument(&self) -> windows_core::Result<super::IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    fn GetIncludeFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
    fn GetExcludeFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
    fn GetComponent(&self, icomponent: super::UINT) -> windows_core::Result<IVssWMComponent>;
    fn GetRestoreMethod(&self, pmethod: *mut super::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut windows_core::BSTR, pbstruserprocedure: *mut windows_core::BSTR, pwriterrestore: *mut super::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut super::UINT) -> windows_core::Result<()>;
    fn GetAlternateLocationMapping(&self, imapping: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
    fn GetBackupSchema(&self) -> windows_core::Result<super::DWORD>;
    fn GetDocument(&self) -> windows_core::Result<super::IXMLDOMDocument>;
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
                match IVssExamineWriterMetadata_Impl::GetIncludeFile(this, core::mem::transmute_copy(&ifile)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExcludeFile<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetComponent<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: super::UINT, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetRestoreMethod<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmethod: *mut super::VSS_RESTOREMETHOD_ENUM, pbstrservice: *mut *mut core::ffi::c_void, pbstruserprocedure: *mut *mut core::ffi::c_void, pwriterrestore: *mut super::VSS_WRITERRESTORE_ENUM, pbrebootrequired: *mut bool, pcmappings: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExamineWriterMetadata_Impl::GetRestoreMethod(this, core::mem::transmute_copy(&pmethod), core::mem::transmute_copy(&pbstrservice), core::mem::transmute_copy(&pbstruserprocedure), core::mem::transmute_copy(&pwriterrestore), core::mem::transmute_copy(&pbrebootrequired), core::mem::transmute_copy(&pcmappings)).into()
            }
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetBackupSchema<Identity: IVssExamineWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwschemamask: *mut super::DWORD) -> windows_core::HRESULT {
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
    pub unsafe fn GetExcludeFromSnapshotCount(&self) -> windows_core::Result<super::UINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludeFromSnapshotCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetExcludeFromSnapshotFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExcludeFromSnapshotFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
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
    fn GetExcludeFromSnapshotCount(&self) -> windows_core::Result<super::UINT>;
    fn GetExcludeFromSnapshotFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
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
                match IVssExamineWriterMetadataEx2_Impl::GetExcludeFromSnapshotCount(this) {
                    Ok(ok__) => {
                        pcexcludedfromsnapshot.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExcludeFromSnapshotFile<Identity: IVssExamineWriterMetadataEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "msxml", feature = "oaidl", feature = "shared", feature = "vss", feature = "vswriter"))]
impl windows_core::RuntimeName for IVssExamineWriterMetadataEx2 {}
windows_core::imp::define_interface!(IVssWMComponent, IVssWMComponent_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMComponent, windows_core::IUnknown);
impl IVssWMComponent {
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetComponentInfo(&self) -> windows_core::Result<PVSSCOMPONENTINFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn FreeComponentInfo(&self, pinfo: PVSSCOMPONENTINFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeComponentInfo)(windows_core::Interface::as_raw(self), pinfo) }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFile)(windows_core::Interface::as_raw(self), ifile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetDatabaseFile(&self, idbfile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDatabaseFile)(windows_core::Interface::as_raw(self), idbfile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetDatabaseLogFile(&self, idblogfile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDatabaseLogFile)(windows_core::Interface::as_raw(self), idblogfile, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "vswriter"))]
    pub unsafe fn GetDependency(&self, idependency: super::UINT) -> windows_core::Result<super::IVssWMDependency> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDependency)(windows_core::Interface::as_raw(self), idependency, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
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
    fn GetComponentInfo(&self) -> windows_core::Result<PVSSCOMPONENTINFO>;
    fn FreeComponentInfo(&self, pinfo: PVSSCOMPONENTINFO) -> windows_core::Result<()>;
    fn GetFile(&self, ifile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
    fn GetDatabaseFile(&self, idbfile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
    fn GetDatabaseLogFile(&self, idblogfile: super::UINT) -> windows_core::Result<super::IVssWMFiledesc>;
    fn GetDependency(&self, idependency: super::UINT) -> windows_core::Result<super::IVssWMDependency>;
}
#[cfg(all(feature = "shared", feature = "vswriter"))]
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
        unsafe extern "system" fn FreeComponentInfo<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinfo: PVSSCOMPONENTINFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWMComponent_Impl::FreeComponentInfo(this, core::mem::transmute_copy(&pinfo)).into()
            }
        }
        unsafe extern "system" fn GetFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDatabaseFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idbfile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDatabaseLogFile<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idblogfile: super::UINT, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetDependency<Identity: IVssWMComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idependency: super::UINT, ppdependency: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "vswriter"))]
impl windows_core::RuntimeName for IVssWMComponent {}
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
