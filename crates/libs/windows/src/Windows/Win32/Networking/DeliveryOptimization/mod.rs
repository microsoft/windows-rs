#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DODownloadCostPolicy(pub i32);
pub const DODownloadCostPolicy_Always: DODownloadCostPolicy = DODownloadCostPolicy(0i32);
pub const DODownloadCostPolicy_NoCellular: DODownloadCostPolicy = DODownloadCostPolicy(5i32);
pub const DODownloadCostPolicy_NoRoaming: DODownloadCostPolicy = DODownloadCostPolicy(3i32);
pub const DODownloadCostPolicy_NoSurcharge: DODownloadCostPolicy = DODownloadCostPolicy(4i32);
pub const DODownloadCostPolicy_Standard: DODownloadCostPolicy = DODownloadCostPolicy(2i32);
pub const DODownloadCostPolicy_Unrestricted: DODownloadCostPolicy = DODownloadCostPolicy(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DODownloadProperty(pub i32);
pub const DODownloadProperty_BlockingMode: DODownloadProperty = DODownloadProperty(12i32);
pub const DODownloadProperty_CallbackFreqPercent: DODownloadProperty = DODownloadProperty(8i32);
pub const DODownloadProperty_CallbackFreqSeconds: DODownloadProperty = DODownloadProperty(9i32);
pub const DODownloadProperty_CallbackInterface: DODownloadProperty = DODownloadProperty(13i32);
pub const DODownloadProperty_ContentId: DODownloadProperty = DODownloadProperty(2i32);
pub const DODownloadProperty_CorrelationVector: DODownloadProperty = DODownloadProperty(17i32);
pub const DODownloadProperty_CostPolicy: DODownloadProperty = DODownloadProperty(6i32);
pub const DODownloadProperty_DecryptionInfo: DODownloadProperty = DODownloadProperty(18i32);
pub const DODownloadProperty_DisallowOnCellular: DODownloadProperty = DODownloadProperty(22i32);
pub const DODownloadProperty_DisplayName: DODownloadProperty = DODownloadProperty(3i32);
pub const DODownloadProperty_ForegroundPriority: DODownloadProperty = DODownloadProperty(11i32);
pub const DODownloadProperty_HttpAllowSecureToNonSecureRedirect: DODownloadProperty = DODownloadProperty(24i32);
pub const DODownloadProperty_HttpCustomAuthHeaders: DODownloadProperty = DODownloadProperty(23i32);
pub const DODownloadProperty_HttpCustomHeaders: DODownloadProperty = DODownloadProperty(5i32);
pub const DODownloadProperty_HttpRedirectionTarget: DODownloadProperty = DODownloadProperty(26i32);
pub const DODownloadProperty_HttpResponseHeaders: DODownloadProperty = DODownloadProperty(27i32);
pub const DODownloadProperty_HttpServerIPAddress: DODownloadProperty = DODownloadProperty(28i32);
pub const DODownloadProperty_HttpStatusCode: DODownloadProperty = DODownloadProperty(29i32);
pub const DODownloadProperty_Id: DODownloadProperty = DODownloadProperty(0i32);
pub const DODownloadProperty_IntegrityCheckInfo: DODownloadProperty = DODownloadProperty(19i32);
pub const DODownloadProperty_IntegrityCheckMandatory: DODownloadProperty = DODownloadProperty(20i32);
pub const DODownloadProperty_LocalPath: DODownloadProperty = DODownloadProperty(4i32);
pub const DODownloadProperty_NetworkToken: DODownloadProperty = DODownloadProperty(16i32);
pub const DODownloadProperty_NoProgressTimeoutSeconds: DODownloadProperty = DODownloadProperty(10i32);
pub const DODownloadProperty_NonVolatile: DODownloadProperty = DODownloadProperty(25i32);
pub const DODownloadProperty_SecurityContext: DODownloadProperty = DODownloadProperty(15i32);
pub const DODownloadProperty_SecurityFlags: DODownloadProperty = DODownloadProperty(7i32);
pub const DODownloadProperty_StreamInterface: DODownloadProperty = DODownloadProperty(14i32);
pub const DODownloadProperty_TotalSizeBytes: DODownloadProperty = DODownloadProperty(21i32);
pub const DODownloadProperty_Uri: DODownloadProperty = DODownloadProperty(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DODownloadState(pub i32);
pub const DODownloadState_Aborted: DODownloadState = DODownloadState(4i32);
pub const DODownloadState_Created: DODownloadState = DODownloadState(0i32);
pub const DODownloadState_Finalized: DODownloadState = DODownloadState(3i32);
pub const DODownloadState_Paused: DODownloadState = DODownloadState(5i32);
pub const DODownloadState_Transferred: DODownloadState = DODownloadState(2i32);
pub const DODownloadState_Transferring: DODownloadState = DODownloadState(1i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DO_DOWNLOAD_ENUM_CATEGORY {
    pub Property: DODownloadProperty,
    pub Value: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DO_DOWNLOAD_RANGE {
    pub Offset: u64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DO_DOWNLOAD_RANGES_INFO {
    pub RangeCount: u32,
    pub Ranges: [DO_DOWNLOAD_RANGE; 1],
}
impl Default for DO_DOWNLOAD_RANGES_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DO_DOWNLOAD_STATUS {
    pub BytesTotal: u64,
    pub BytesTransferred: u64,
    pub State: DODownloadState,
    pub Error: windows_core::HRESULT,
    pub ExtendedError: windows_core::HRESULT,
}
pub const DecryptionInfo_AlgorithmName: windows_core::PCWSTR = windows_core::w!("AlgorithmName");
pub const DecryptionInfo_ChainingMode: windows_core::PCWSTR = windows_core::w!("ChainingMode");
pub const DecryptionInfo_EncryptionBufferSize: windows_core::PCWSTR = windows_core::w!("EncryptionBufferSize");
pub const DecryptionInfo_KeyData: windows_core::PCWSTR = windows_core::w!("KeyData");
pub const DeliveryOptimization: windows_core::GUID = windows_core::GUID::from_u128(0x5b99fa76_721c_423c_adac_56d03c8a8007);
windows_core::imp::define_interface!(IDODownload, IDODownload_Vtbl, 0xfbbd7fc0_c147_4727_a38d_827ef071ee77);
windows_core::imp::interface_hierarchy!(IDODownload, windows_core::IUnknown);
impl IDODownload {
    pub unsafe fn Start(&self, ranges: *const DO_DOWNLOAD_RANGES_INFO) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ranges).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Abort(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Finalize(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Finalize)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetStatus(&self, status: *mut DO_DOWNLOAD_STATUS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), status as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn GetProperty(&self, propid: DODownloadProperty) -> windows_core::Result<super::super::System::Variant::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), propid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub unsafe fn SetProperty(&self, propid: DODownloadProperty, propval: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), propid, core::mem::transmute(propval)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDODownload_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *const DO_DOWNLOAD_RANGES_INFO) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Finalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DO_DOWNLOAD_STATUS) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, DODownloadProperty, *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, DODownloadProperty, *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant")))]
    SetProperty: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDODownload_Impl: windows_core::IUnknownImpl {
    fn Start(&self, ranges: *const DO_DOWNLOAD_RANGES_INFO) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn Finalize(&self) -> windows_core::Result<()>;
    fn GetStatus(&self, status: *mut DO_DOWNLOAD_STATUS) -> windows_core::Result<()>;
    fn GetProperty(&self, propid: DODownloadProperty) -> windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, propid: DODownloadProperty, propval: *const super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDODownload_Vtbl {
    pub const fn new<Identity: IDODownload_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ranges: *const DO_DOWNLOAD_RANGES_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownload_Impl::Start(this, core::mem::transmute_copy(&ranges)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownload_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownload_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn Finalize<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownload_Impl::Finalize(this).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DO_DOWNLOAD_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownload_Impl::GetStatus(this, core::mem::transmute_copy(&status)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: DODownloadProperty, propval: *mut super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDODownload_Impl::GetProperty(this, core::mem::transmute_copy(&propid)) {
                    Ok(ok__) => {
                        propval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IDODownload_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: DODownloadProperty, propval: *const super::super::System::Variant::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownload_Impl::SetProperty(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propval)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            Finalize: Finalize::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDODownload as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDODownload {}
windows_core::imp::define_interface!(IDODownloadStatusCallback, IDODownloadStatusCallback_Vtbl, 0xd166e8e3_a90e_4392_8e87_05e996d3747d);
windows_core::imp::interface_hierarchy!(IDODownloadStatusCallback, windows_core::IUnknown);
impl IDODownloadStatusCallback {
    pub unsafe fn OnStatusChange<P0>(&self, download: P0, status: *const DO_DOWNLOAD_STATUS) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDODownload>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStatusChange)(windows_core::Interface::as_raw(self), download.param().abi(), status).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDODownloadStatusCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const DO_DOWNLOAD_STATUS) -> windows_core::HRESULT,
}
pub trait IDODownloadStatusCallback_Impl: windows_core::IUnknownImpl {
    fn OnStatusChange(&self, download: windows_core::Ref<IDODownload>, status: *const DO_DOWNLOAD_STATUS) -> windows_core::Result<()>;
}
impl IDODownloadStatusCallback_Vtbl {
    pub const fn new<Identity: IDODownloadStatusCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStatusChange<Identity: IDODownloadStatusCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, download: *mut core::ffi::c_void, status: *const DO_DOWNLOAD_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDODownloadStatusCallback_Impl::OnStatusChange(this, core::mem::transmute_copy(&download), core::mem::transmute_copy(&status)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatusChange: OnStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDODownloadStatusCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDODownloadStatusCallback {}
windows_core::imp::define_interface!(IDOManager, IDOManager_Vtbl, 0x400e2d4a_1431_4c1a_a748_39ca472cfdb1);
windows_core::imp::interface_hierarchy!(IDOManager, windows_core::IUnknown);
impl IDOManager {
    pub unsafe fn CreateDownload(&self) -> windows_core::Result<IDODownload> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDownload)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumDownloads(&self, category: *const DO_DOWNLOAD_ENUM_CATEGORY) -> windows_core::Result<super::super::System::Com::IEnumUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDownloads)(windows_core::Interface::as_raw(self), category, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDOManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDownload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumDownloads: unsafe extern "system" fn(*mut core::ffi::c_void, *const DO_DOWNLOAD_ENUM_CATEGORY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumDownloads: usize,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDOManager_Impl: windows_core::IUnknownImpl {
    fn CreateDownload(&self) -> windows_core::Result<IDODownload>;
    fn EnumDownloads(&self, category: *const DO_DOWNLOAD_ENUM_CATEGORY) -> windows_core::Result<super::super::System::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl IDOManager_Vtbl {
    pub const fn new<Identity: IDOManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDownload<Identity: IDOManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, download: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDOManager_Impl::CreateDownload(this) {
                    Ok(ok__) => {
                        download.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumDownloads<Identity: IDOManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: *const DO_DOWNLOAD_ENUM_CATEGORY, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDOManager_Impl::EnumDownloads(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDownload: CreateDownload::<Identity, OFFSET>,
            EnumDownloads: EnumDownloads::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDOManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IDOManager {}
pub const IntegrityCheckInfo_HashOfHashes: windows_core::PCWSTR = windows_core::w!("HashOfHashes");
pub const IntegrityCheckInfo_PiecesHashFileDigest: windows_core::PCWSTR = windows_core::w!("PiecesHashFileDigest");
pub const IntegrityCheckInfo_PiecesHashFileDigestAlgorithm: windows_core::PCWSTR = windows_core::w!("PiecesHashFileDigestAlgorithm");
pub const IntegrityCheckInfo_PiecesHashFileUrl: windows_core::PCWSTR = windows_core::w!("PiecesHashFileUrl");
