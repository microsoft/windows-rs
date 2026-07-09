pub type DXGI_MEMORY_SEGMENT_GROUP = i32;
pub const DXGI_MEMORY_SEGMENT_GROUP_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 0;
pub const DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL: DXGI_MEMORY_SEGMENT_GROUP = 1;
pub type DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = i32;
pub const DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_QUERY_VIDEO_MEMORY_INFO {
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub AvailableForReservation: u64,
    pub CurrentReservation: u64,
}
pub type DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = i32;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 2;
pub const DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT: DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG = 1;
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::define_interface!(IDXGIAdapter3, IDXGIAdapter3_Vtbl, 0x645967a4_1392_4310_a798_8053ce3e93fd);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl core::ops::Deref for IDXGIAdapter3 {
    type Target = super::dxgi1_2::IDXGIAdapter2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
windows_core::imp::interface_hierarchy!(IDXGIAdapter3, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIAdapter, super::dxgi::IDXGIAdapter1, super::dxgi1_2::IDXGIAdapter2);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
impl IDXGIAdapter3 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterHardwareContentProtectionTeardownStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterHardwareContentProtectionTeardownStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryVideoMemoryInfo)(windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo as _) }
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVideoMemoryReservation)(windows_core::Interface::as_raw(self), nodeindex, memorysegmentgroup, reservation) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterVideoMemoryBudgetChangeNotificationEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterVideoMemoryBudgetChangeNotification)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter3_Vtbl {
    pub base__: super::dxgi1_2::IDXGIAdapter2_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterHardwareContentProtectionTeardownStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterHardwareContentProtectionTeardownStatusEvent: usize,
    pub UnregisterHardwareContentProtectionTeardownStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub QueryVideoMemoryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_MEMORY_SEGMENT_GROUP, *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT,
    pub SetVideoMemoryReservation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, DXGI_MEMORY_SEGMENT_GROUP, u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub RegisterVideoMemoryBudgetChangeNotificationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    RegisterVideoMemoryBudgetChangeNotificationEvent: usize,
    pub UnregisterVideoMemoryBudgetChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
pub trait IDXGIAdapter3_Impl: super::dxgi1_2::IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::Result<()>;
    fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::winnt::HANDLE) -> windows_core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32);
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
impl IDXGIAdapter3_Vtbl {
    pub const fn new<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter3_Impl::RegisterHardwareContentProtectionTeardownStatusEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::UnregisterHardwareContentProtectionTeardownStatus(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::QueryVideoMemoryInfo(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&pvideomemoryinfo)).into()
            }
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::SetVideoMemoryReservation(this, core::mem::transmute_copy(&nodeindex), core::mem::transmute_copy(&memorysegmentgroup), core::mem::transmute_copy(&reservation)).into()
            }
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hevent: super::winnt::HANDLE, pdwcookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter3_Impl::RegisterVideoMemoryBudgetChangeNotificationEvent(this, core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcookie: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIAdapter3_Impl::UnregisterVideoMemoryBudgetChangeNotification(this, core::mem::transmute_copy(&dwcookie));
            }
        }
        Self {
            base__: super::dxgi1_2::IDXGIAdapter2_Vtbl::new::<Identity, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIAdapter as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIAdapter1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIAdapter2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIAdapter3 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::define_interface!(IDXGIFactory4, IDXGIFactory4_Vtbl, 0x1bc6ea02_ef36_464f_bf0c_21ca39e5168a);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl core::ops::Deref for IDXGIFactory4 {
    type Target = super::dxgi1_3::IDXGIFactory3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::interface_hierarchy!(IDXGIFactory4, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIFactory, super::dxgi::IDXGIFactory1, super::dxgi1_2::IDXGIFactory2, super::dxgi1_3::IDXGIFactory3);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl IDXGIFactory4 {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::winnt::LUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumAdapterByLuid)(windows_core::Interface::as_raw(self), core::mem::transmute(adapterluid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).EnumWarpAdapter)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory4_Vtbl {
    pub base__: super::dxgi1_3::IDXGIFactory3_Vtbl,
    #[cfg(feature = "Win32_winnt")]
    pub EnumAdapterByLuid: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::LUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    EnumAdapterByLuid: usize,
    pub EnumWarpAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGIFactory4_Impl: super::dxgi1_3::IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(&self, adapterluid: &super::winnt::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumWarpAdapter(&self, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGIFactory4_Vtbl {
    pub const fn new<Identity: IDXGIFactory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: super::winnt::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory4_Impl::EnumAdapterByLuid(this, core::mem::transmute(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIFactory4_Impl::EnumWarpAdapter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
            }
        }
        Self {
            base__: super::dxgi1_3::IDXGIFactory3_Vtbl::new::<Identity, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIFactory4 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIFactory1 as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIFactory2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGIFactory4 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::define_interface!(IDXGIOutput4, IDXGIOutput4_Vtbl, 0xdc7dca35_2196_414d_9f53_617884032a60);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl core::ops::Deref for IDXGIOutput4 {
    type Target = super::dxgi1_3::IDXGIOutput3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::interface_hierarchy!(IDXGIOutput4, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIOutput, super::dxgi1_2::IDXGIOutput1, super::dxgi1_3::IDXGIOutput2, super::dxgi1_3::IDXGIOutput3);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl IDXGIOutput4 {
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub unsafe fn CheckOverlayColorSpaceSupport<P2>(&self, format: super::dxgiformat::DXGI_FORMAT, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P2) -> windows_core::Result<u32>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckOverlayColorSpaceSupport)(windows_core::Interface::as_raw(self), format, colorspace, pconcerneddevice.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIOutput4_Vtbl {
    pub base__: super::dxgi1_3::IDXGIOutput3_Vtbl,
    #[cfg(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat"))]
    pub CheckOverlayColorSpaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgiformat::DXGI_FORMAT, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_dxgicommon", feature = "Win32_dxgiformat")))]
    CheckOverlayColorSpaceSupport: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
pub trait IDXGIOutput4_Impl: super::dxgi1_3::IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(&self, format: super::dxgiformat::DXGI_FORMAT, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl IDXGIOutput4_Vtbl {
    pub const fn new<Identity: IDXGIOutput4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: IDXGIOutput4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::dxgiformat::DXGI_FORMAT, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut core::ffi::c_void, pflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIOutput4_Impl::CheckOverlayColorSpaceSupport(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&colorspace), core::mem::transmute_copy(&pconcerneddevice)) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::dxgi1_3::IDXGIOutput3_Vtbl::new::<Identity, OFFSET>(),
            CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIOutput4 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIOutput as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGIOutput1 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIOutput2 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGIOutput3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef"))]
impl windows_core::RuntimeName for IDXGIOutput4 {}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::define_interface!(IDXGISwapChain3, IDXGISwapChain3_Vtbl, 0x94d99bdb_f1f8_4ab0_b236_7da0170edab1);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl core::ops::Deref for IDXGISwapChain3 {
    type Target = super::dxgi1_3::IDXGISwapChain2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
windows_core::imp::interface_hierarchy!(IDXGISwapChain3, windows_core::IUnknown, super::dxgi::IDXGIObject, super::dxgi::IDXGIDeviceSubObject, super::dxgi::IDXGISwapChain, super::dxgi1_2::IDXGISwapChain1, super::dxgi1_3::IDXGISwapChain2);
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
impl IDXGISwapChain3 {
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentBackBufferIndex)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckColorSpaceSupport)(windows_core::Interface::as_raw(self), colorspace, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_dxgicommon")]
    pub unsafe fn SetColorSpace1(&self, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetColorSpace1)(windows_core::Interface::as_raw(self), colorspace) }
    }
    #[cfg(feature = "Win32_dxgiformat")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: super::dxgiformat::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers1)(windows_core::Interface::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, core::mem::transmute(pppresentqueue)) }
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3"))]
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain3_Vtbl {
    pub base__: super::dxgi1_3::IDXGISwapChain2_Vtbl,
    pub GetCurrentBackBufferIndex: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_dxgicommon")]
    pub CheckColorSpaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgicommon::DXGI_COLOR_SPACE_TYPE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    CheckColorSpaceSupport: usize,
    #[cfg(feature = "Win32_dxgicommon")]
    pub SetColorSpace1: unsafe extern "system" fn(*mut core::ffi::c_void, super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgicommon"))]
    SetColorSpace1: usize,
    #[cfg(feature = "Win32_dxgiformat")]
    pub ResizeBuffers1: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, super::dxgiformat::DXGI_FORMAT, u32, *const u32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_dxgiformat"))]
    ResizeBuffers1: usize,
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
pub trait IDXGISwapChain3_Impl: super::dxgi1_3::IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(&self) -> u32;
    fn CheckColorSpaceSupport(&self, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<u32>;
    fn SetColorSpace1(&self, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::Result<()>;
    fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: super::dxgiformat::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const Option<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::GetCurrentBackBufferIndex(this)
            }
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGISwapChain3_Impl::CheckColorSpaceSupport(this, core::mem::transmute_copy(&colorspace)) {
                    Ok(ok__) => {
                        pcolorspacesupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, colorspace: super::dxgicommon::DXGI_COLOR_SPACE_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::SetColorSpace1(this, core::mem::transmute_copy(&colorspace)).into()
            }
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: super::dxgiformat::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGISwapChain3_Impl::ResizeBuffers1(this, core::mem::transmute_copy(&buffercount), core::mem::transmute_copy(&width), core::mem::transmute_copy(&height), core::mem::transmute_copy(&format), core::mem::transmute_copy(&swapchainflags), core::mem::transmute_copy(&pcreationnodemask), core::mem::transmute_copy(&pppresentqueue)).into()
            }
        }
        Self {
            base__: super::dxgi1_3::IDXGISwapChain2_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<super::dxgi::IDXGISwapChain as windows_core::Interface>::IID || iid == &<super::dxgi1_2::IDXGISwapChain1 as windows_core::Interface>::IID || iid == &<super::dxgi1_3::IDXGISwapChain2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_dxgi", feature = "Win32_dxgi1_2", feature = "Win32_dxgi1_3", feature = "Win32_dxgicommon", feature = "Win32_dxgiformat", feature = "Win32_dxgitype", feature = "Win32_windef", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IDXGISwapChain3 {}
