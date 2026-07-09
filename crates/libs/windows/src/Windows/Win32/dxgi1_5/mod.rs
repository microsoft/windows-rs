pub type DXGI_FEATURE = i32;
pub const DXGI_FEATURE_PRESENT_ALLOW_TEARING: DXGI_FEATURE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_HDR_METADATA_TYPE = i32;
pub const DXGI_HDR_METADATA_TYPE_HDR10: DXGI_HDR_METADATA_TYPE = 1;
pub const DXGI_HDR_METADATA_TYPE_HDR10PLUS: DXGI_HDR_METADATA_TYPE = 2;
pub const DXGI_HDR_METADATA_TYPE_NONE: DXGI_HDR_METADATA_TYPE = 0;
pub type DXGI_OFFER_RESOURCE_FLAGS = i32;
pub const DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT: DXGI_OFFER_RESOURCE_FLAGS = 1;
pub const DXGI_OUTDUPL_COMPOSITED_UI_CAPTURE_ONLY: DXGI_OUTDUPL_FLAG = 1;
pub type DXGI_OUTDUPL_FLAG = i32;
pub type DXGI_RECLAIM_RESOURCE_RESULTS = i32;
pub const DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED: DXGI_RECLAIM_RESOURCE_RESULTS = 1;
pub const DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED: DXGI_RECLAIM_RESOURCE_RESULTS = 2;
pub const DXGI_RECLAIM_RESOURCE_RESULT_OK: DXGI_RECLAIM_RESOURCE_RESULTS = 0;
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::define_interface!(IDXGIDevice4, IDXGIDevice4_Vtbl, 0x95b4f95f_d8da_4ca4_9ee6_3b76d5968a10);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl core::ops::Deref for IDXGIDevice4 {
    type Target = super::dxgi1_3::IDXGIDevice3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::interface_hierarchy!(IDXGIDevice4, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDevice, super::dxgi::IDXGIDevice1, super::dxgi1_2::IDXGIDevice2, super::dxgi1_3::IDXGIDevice3);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl IDXGIDevice4 {
    pub unsafe fn OfferResources1(&self, ppresources: &[Option<super::dxgi::IDXGIResource>], priority: super::dxgi1_2::DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OfferResources1)(windows_core::Interface::as_raw(self), ppresources.len().try_into().unwrap(), core::mem::transmute(ppresources.as_ptr()), priority, flags) }
    }
    pub unsafe fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<super::dxgi::IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReclaimResources1)(windows_core::Interface::as_raw(self), numresources, core::mem::transmute(ppresources), presults as _) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice4_Vtbl {
    pub base__: super::dxgi1_3::IDXGIDevice3_Vtbl,
    pub OfferResources1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, super::dxgi1_2::DXGI_OFFER_RESOURCE_PRIORITY, u32) -> windows_core::HRESULT,
    pub ReclaimResources1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const *mut core::ffi::c_void, *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
pub trait IDXGIDevice4_Impl: super::dxgi1_3::IDXGIDevice3_Impl {
    fn OfferResources1(&self, numresources: u32, ppresources: *const Option<super::dxgi::IDXGIResource>, priority: super::dxgi1_2::DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::Result<()>;
    fn ReclaimResources1(&self, numresources: u32, ppresources: *const Option<super::dxgi::IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: IDXGIDevice4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OfferResources1<Identity: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, priority: super::dxgi1_2::DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice4_Impl::OfferResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&priority), core::mem::transmute_copy(&flags)).into()
            }
        }
        unsafe extern "system" fn ReclaimResources1<Identity: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numresources: u32, ppresources: *const *mut core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice4_Impl::ReclaimResources1(this, core::mem::transmute_copy(&numresources), core::mem::transmute_copy(&ppresources), core::mem::transmute_copy(&presults)).into()
            }
        }
        Self {
            base__: super::dxgi1_3::IDXGIDevice3_Vtbl::new::<Identity, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice4 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDevice as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDevice1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIDevice2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIDevice3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIDevice4 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
windows_core::imp::define_interface!(IDXGIFactory5, IDXGIFactory5_Vtbl, 0x7632e1f5_ee65_4dca_87fd_84cd75f8838d);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
impl core::ops::Deref for IDXGIFactory5 {
    type Target = super::dxgi1_4::IDXGIFactory4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
windows_core::imp::interface_hierarchy!(IDXGIFactory5, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIFactory, super::dxgi::IDXGIFactory1, super::dxgi1_2::IDXGIFactory2, super::dxgi1_3::IDXGIFactory3, super::dxgi1_4::IDXGIFactory4);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
impl IDXGIFactory5 {
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckFeatureSupport)(windows_core::Interface::as_raw(self), feature, pfeaturesupportdata as _, featuresupportdatasize) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory5_Vtbl {
    pub base__: super::dxgi1_4::IDXGIFactory4_Vtbl,
    pub CheckFeatureSupport: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FEATURE, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory5_Impl: super::dxgi1_4::IDXGIFactory4_Impl {
    fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory5_Vtbl {
    pub const fn new<Identity: IDXGIFactory5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckFeatureSupport<Identity: IDXGIFactory5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut core::ffi::c_void, featuresupportdatasize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory5_Impl::CheckFeatureSupport(this, core::mem::transmute_copy(&feature), core::mem::transmute_copy(&pfeaturesupportdata), core::mem::transmute_copy(&featuresupportdatasize)).into()
            }
        }
        Self { base__: super::dxgi1_4::IDXGIFactory4_Vtbl::new::<Identity, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory5 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIFactory2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIFactory3 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGIFactory4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory5 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
windows_core::imp::define_interface!(IDXGIOutput5, IDXGIOutput5_Vtbl, 0x80a07424_ab52_42eb_833c_0c42fd282d98);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
impl core::ops::Deref for IDXGIOutput5 {
    type Target = super::dxgi1_4::IDXGIOutput4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
windows_core::imp::interface_hierarchy!(IDXGIOutput5, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIOutput, super::dxgi1_2::IDXGIOutput1, super::dxgi1_3::IDXGIOutput2, super::dxgi1_3::IDXGIOutput3, super::dxgi1_4::IDXGIOutput4);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
impl IDXGIOutput5 {
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[super::dxgiformat::DXGI_FORMAT]) -> windows_core::Result<super::dxgi1_2::IDXGIOutputDuplication>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DuplicateOutput1)(windows_core::Interface::as_raw(self), pdevice.param().abi(), flags, psupportedformats.len().try_into().unwrap(), core::mem::transmute(psupportedformats.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput5_Vtbl {
    pub base__: super::dxgi1_4::IDXGIOutput4_Vtbl,
    #[cfg(feature = "Win32_dxgiformat")]
    pub DuplicateOutput1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *const super::dxgiformat::DXGI_FORMAT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    DuplicateOutput1: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput5_Impl: super::dxgi1_4::IDXGIOutput4_Impl {
    fn DuplicateOutput1(&self, pdevice: windows_core::Ref<windows_core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const super::dxgiformat::DXGI_FORMAT) -> windows_core::Result<super::dxgi1_2::IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput5_Vtbl {
    pub const fn new<Identity: IDXGIOutput5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DuplicateOutput1<Identity: IDXGIOutput5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevice: *mut core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const super::dxgiformat::DXGI_FORMAT, ppoutputduplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput5_Impl::DuplicateOutput1(this, core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&supportedformatscount), core::mem::transmute_copy(&psupportedformats)) {
                    Ok(ok__) => {
                        ppoutputduplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::dxgi1_4::IDXGIOutput4_Vtbl::new::<Identity, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput5 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIOutput as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIOutput1 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIOutput2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIOutput3 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGIOutput4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput5 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
windows_core::imp::define_interface!(IDXGISwapChain4, IDXGISwapChain4_Vtbl, 0x3d585d5a_bd4a_489e_b1f4_3dbcb6452ffb);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
impl core::ops::Deref for IDXGISwapChain4 {
    type Target = super::dxgi1_4::IDXGISwapChain3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
windows_core::imp::interface_hierarchy!(IDXGISwapChain4, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDeviceSubObject, super::dxgi::IDXGISwapChain, super::dxgi1_2::IDXGISwapChain1, super::dxgi1_3::IDXGISwapChain2, super::dxgi1_4::IDXGISwapChain3);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
impl IDXGISwapChain4 {
    pub unsafe fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, pmetadata: Option<&[u8]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHDRMetaData)(windows_core::Interface::as_raw(self), r#type, pmetadata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pmetadata.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain4_Vtbl {
    pub base__: super::dxgi1_4::IDXGISwapChain3_Vtbl,
    pub SetHDRMetaData: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_HDR_METADATA_TYPE, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGISwapChain4_Impl: super::dxgi1_4::IDXGISwapChain3_Impl {
    fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGISwapChain4_Vtbl {
    pub const fn new<Identity: IDXGISwapChain4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHDRMetaData<Identity: IDXGISwapChain4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain4_Impl::SetHDRMetaData(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&size), core::mem::transmute_copy(&pmetadata)).into()
            }
        }
        Self { base__: super::dxgi1_4::IDXGISwapChain3_Vtbl::new::<Identity, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGISwapChain as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGISwapChain2 as windows_core::Interface>::IID || iid == &<super::dxgi1_4::IDXGISwapChain3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgi1_4", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGISwapChain4 {}
