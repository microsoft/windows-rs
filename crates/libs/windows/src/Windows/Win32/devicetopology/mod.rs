pub const Connector: PartType = 0;
pub type ConnectorType = i32;
pub const DEVTOPO_HARDWARE_INITIATED_EVENTCONTEXT: i32 = 1685217608;
pub type DataFlow = i32;
pub const DeviceTopology: windows_core::GUID = windows_core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
pub const EVENTCONTEXT_VOLUMESLIDER: windows_core::GUID = windows_core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
windows_core::imp::define_interface!(IAudioAutoGainControl, IAudioAutoGainControl_Vtbl, 0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
windows_core::imp::interface_hierarchy!(IAudioAutoGainControl, windows_core::IUnknown);
impl IAudioAutoGainControl {
    pub unsafe fn GetEnabled(&self, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), pbenabled as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetEnabled(&self, benable: bool, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "guiddef")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetEnabled: usize,
}
#[cfg(feature = "guiddef")]
pub trait IAudioAutoGainControl_Impl: windows_core::IUnknownImpl {
    fn GetEnabled(&self, pbenabled: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetEnabled(&self, benable: windows_core::BOOL, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IAudioAutoGainControl_Vtbl {
    pub const fn new<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabled<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAutoGainControl_Impl::GetEnabled(this, core::mem::transmute_copy(&pbenabled)).into()
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAutoGainControl_Impl::SetEnabled(this, core::mem::transmute_copy(&benable), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnabled: GetEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioAutoGainControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IAudioAutoGainControl {}
windows_core::imp::define_interface!(IAudioBass, IAudioBass_Vtbl, 0xa2b1a1d9_4db3_425d_a2b2_bd335cb3e2e5);
impl core::ops::Deref for IAudioBass {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioBass, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioBass_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioBass_Impl: IPerChannelDbLevel_Impl {}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioBass_Vtbl {
    pub const fn new<Identity: IAudioBass_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioBass as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioBass {}
windows_core::imp::define_interface!(IAudioChannelConfig, IAudioChannelConfig_Vtbl, 0xbb11c46f_ec28_493c_b88a_5db88062ce98);
windows_core::imp::interface_hierarchy!(IAudioChannelConfig, windows_core::IUnknown);
impl IAudioChannelConfig {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetChannelConfig(&self, dwconfig: super::DWORD, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelConfig)(windows_core::Interface::as_raw(self), dwconfig, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetChannelConfig(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelConfig)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioChannelConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetChannelConfig: usize,
    #[cfg(feature = "shared")]
    pub GetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetChannelConfig: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioChannelConfig_Impl: windows_core::IUnknownImpl {
    fn SetChannelConfig(&self, dwconfig: super::DWORD, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
    fn GetChannelConfig(&self) -> windows_core::Result<super::DWORD>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioChannelConfig_Vtbl {
    pub const fn new<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetChannelConfig<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfig: super::DWORD, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioChannelConfig_Impl::SetChannelConfig(this, core::mem::transmute_copy(&dwconfig), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelConfig<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconfig: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioChannelConfig_Impl::GetChannelConfig(this) {
                    Ok(ok__) => {
                        pdwconfig.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetChannelConfig: SetChannelConfig::<Identity, OFFSET>,
            GetChannelConfig: GetChannelConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioChannelConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioChannelConfig {}
windows_core::imp::define_interface!(IAudioInputSelector, IAudioInputSelector_Vtbl, 0x4f03dc02_5e6e_4653_8f72_a030c123d598);
windows_core::imp::interface_hierarchy!(IAudioInputSelector, windows_core::IUnknown);
impl IAudioInputSelector {
    #[cfg(feature = "shared")]
    pub unsafe fn GetSelection(&self, pnidselected: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), pnidselected as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetSelection(&self, nidselect: super::UINT, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSelection: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetSelection: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioInputSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self, pnidselected: *mut super::UINT) -> windows_core::Result<()>;
    fn SetSelection(&self, nidselect: super::UINT, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioInputSelector_Vtbl {
    pub const fn new<Identity: IAudioInputSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioInputSelector_Impl::GetSelection(this, core::mem::transmute_copy(&pnidselected)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: super::UINT, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioInputSelector_Impl::SetSelection(this, core::mem::transmute_copy(&nidselect), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioInputSelector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioInputSelector {}
windows_core::imp::define_interface!(IAudioLoudness, IAudioLoudness_Vtbl, 0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
windows_core::imp::interface_hierarchy!(IAudioLoudness, windows_core::IUnknown);
impl IAudioLoudness {
    pub unsafe fn GetEnabled(&self, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), pbenabled as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetEnabled(&self, benable: bool, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "guiddef")]
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetEnabled: usize,
}
#[cfg(feature = "guiddef")]
pub trait IAudioLoudness_Impl: windows_core::IUnknownImpl {
    fn GetEnabled(&self, pbenabled: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn SetEnabled(&self, benable: windows_core::BOOL, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IAudioLoudness_Vtbl {
    pub const fn new<Identity: IAudioLoudness_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabled<Identity: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioLoudness_Impl::GetEnabled(this, core::mem::transmute_copy(&pbenabled)).into()
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioLoudness_Impl::SetEnabled(this, core::mem::transmute_copy(&benable), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnabled: GetEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioLoudness as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IAudioLoudness {}
windows_core::imp::define_interface!(IAudioMidrange, IAudioMidrange_Vtbl, 0x5e54b6d7_b44b_40d9_9a9e_e691d9ce6edf);
impl core::ops::Deref for IAudioMidrange {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioMidrange, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMidrange_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioMidrange_Impl: IPerChannelDbLevel_Impl {}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioMidrange_Vtbl {
    pub const fn new<Identity: IAudioMidrange_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMidrange as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioMidrange {}
windows_core::imp::define_interface!(IAudioMute, IAudioMute_Vtbl, 0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
windows_core::imp::interface_hierarchy!(IAudioMute, windows_core::IUnknown);
impl IAudioMute {
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetMute(&self, bmuted: bool, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmuted.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetMute(&self, pbmuted: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), pbmuted as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "guiddef")]
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetMute: usize,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "guiddef")]
pub trait IAudioMute_Impl: windows_core::IUnknownImpl {
    fn SetMute(&self, bmuted: windows_core::BOOL, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
    fn GetMute(&self, pbmuted: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "guiddef")]
impl IAudioMute_Vtbl {
    pub const fn new<Identity: IAudioMute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMute<Identity: IAudioMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmuted: windows_core::BOOL, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioMute_Impl::SetMute(this, core::mem::transmute_copy(&bmuted), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IAudioMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmuted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioMute_Impl::GetMute(this, core::mem::transmute_copy(&pbmuted)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMute: SetMute::<Identity, OFFSET>, GetMute: GetMute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMute as windows_core::Interface>::IID
    }
}
#[cfg(feature = "guiddef")]
impl windows_core::RuntimeName for IAudioMute {}
windows_core::imp::define_interface!(IAudioOutputSelector, IAudioOutputSelector_Vtbl, 0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
windows_core::imp::interface_hierarchy!(IAudioOutputSelector, windows_core::IUnknown);
impl IAudioOutputSelector {
    #[cfg(feature = "shared")]
    pub unsafe fn GetSelection(&self, pnidselected: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), pnidselected as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetSelection(&self, nidselect: super::UINT, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSelection: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetSelection: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioOutputSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self, pnidselected: *mut super::UINT) -> windows_core::Result<()>;
    fn SetSelection(&self, nidselect: super::UINT, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioOutputSelector_Vtbl {
    pub const fn new<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioOutputSelector_Impl::GetSelection(this, core::mem::transmute_copy(&pnidselected)).into()
            }
        }
        unsafe extern "system" fn SetSelection<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: super::UINT, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioOutputSelector_Impl::SetSelection(this, core::mem::transmute_copy(&nidselect), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, OFFSET>,
            SetSelection: SetSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioOutputSelector as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioOutputSelector {}
windows_core::imp::define_interface!(IAudioPeakMeter, IAudioPeakMeter_Vtbl, 0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
windows_core::imp::interface_hierarchy!(IAudioPeakMeter, windows_core::IUnknown);
impl IAudioPeakMeter {
    #[cfg(feature = "shared")]
    pub unsafe fn GetChannelCount(&self, pcchannels: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), pcchannels as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevel(&self, nchannel: super::UINT, pflevel: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, pflevel as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetChannelCount: usize,
    #[cfg(feature = "shared")]
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLevel: usize,
}
#[cfg(feature = "shared")]
pub trait IAudioPeakMeter_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self, pcchannels: *mut super::UINT) -> windows_core::Result<()>;
    fn GetLevel(&self, nchannel: super::UINT, pflevel: *mut f32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IAudioPeakMeter_Vtbl {
    pub const fn new<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioPeakMeter_Impl::GetChannelCount(this, core::mem::transmute_copy(&pcchannels)).into()
            }
        }
        unsafe extern "system" fn GetLevel<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: super::UINT, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioPeakMeter_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&pflevel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            GetLevel: GetLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioPeakMeter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IAudioPeakMeter {}
windows_core::imp::define_interface!(IAudioTreble, IAudioTreble_Vtbl, 0x0a717812_694e_4907_b74b_bafa5cfdca7b);
impl core::ops::Deref for IAudioTreble {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioTreble, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioTreble_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioTreble_Impl: IPerChannelDbLevel_Impl {}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioTreble_Vtbl {
    pub const fn new<Identity: IAudioTreble_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioTreble as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioTreble {}
windows_core::imp::define_interface!(IAudioVolumeLevel, IAudioVolumeLevel_Vtbl, 0x7fb7b48f_531d_44a2_bcb3_5ad5a134b3dc);
impl core::ops::Deref for IAudioVolumeLevel {
    type Target = IPerChannelDbLevel;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioVolumeLevel, windows_core::IUnknown, IPerChannelDbLevel);
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeLevel_Vtbl {
    pub base__: IPerChannelDbLevel_Vtbl,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IAudioVolumeLevel_Impl: IPerChannelDbLevel_Impl {}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IAudioVolumeLevel_Vtbl {
    pub const fn new<Identity: IAudioVolumeLevel_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioVolumeLevel as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IAudioVolumeLevel {}
windows_core::imp::define_interface!(IConnector, IConnector_Vtbl, 0x9c2c4058_23f5_41de_877a_df3af236a09e);
windows_core::imp::interface_hierarchy!(IConnector, windows_core::IUnknown);
impl IConnector {
    pub unsafe fn GetType(&self, ptype: *mut ConnectorType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), ptype as _) }
    }
    pub unsafe fn GetDataFlow(&self, pflow: *mut DataFlow) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), pflow as _) }
    }
    pub unsafe fn ConnectTo<P0>(&self, pconnectto: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectTo)(windows_core::Interface::as_raw(self), pconnectto.param().abi()) }
    }
    pub unsafe fn Disconnect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disconnect)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsConnected(&self, pbconnected: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), pbconnected as _) }
    }
    pub unsafe fn GetConnectedTo(&self, ppconto: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnectedTo)(windows_core::Interface::as_raw(self), core::mem::transmute(ppconto)) }
    }
    pub unsafe fn GetConnectorIdConnectedTo(&self, ppwstrconnectorid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnectorIdConnectedTo)(windows_core::Interface::as_raw(self), ppwstrconnectorid as _) }
    }
    pub unsafe fn GetDeviceIdConnectedTo(&self, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceIdConnectedTo)(windows_core::Interface::as_raw(self), ppwstrdeviceid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConnector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConnectorType) -> windows_core::HRESULT,
    pub GetDataFlow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DataFlow) -> windows_core::HRESULT,
    pub ConnectTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disconnect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConnectorIdConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetDeviceIdConnectedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IConnector_Impl: windows_core::IUnknownImpl {
    fn GetType(&self, ptype: *mut ConnectorType) -> windows_core::Result<()>;
    fn GetDataFlow(&self, pflow: *mut DataFlow) -> windows_core::Result<()>;
    fn ConnectTo(&self, pconnectto: windows_core::Ref<IConnector>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn IsConnected(&self, pbconnected: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetConnectedTo(&self, ppconto: windows_core::OutRef<IConnector>) -> windows_core::Result<()>;
    fn GetConnectorIdConnectedTo(&self, ppwstrconnectorid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetDeviceIdConnectedTo(&self, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
}
impl IConnector_Vtbl {
    pub const fn new<Identity: IConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut ConnectorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::GetType(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn GetDataFlow<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflow: *mut DataFlow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::GetDataFlow(this, core::mem::transmute_copy(&pflow)).into()
            }
        }
        unsafe extern "system" fn ConnectTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconnectto: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::ConnectTo(this, core::mem::transmute_copy(&pconnectto)).into()
            }
        }
        unsafe extern "system" fn Disconnect<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::Disconnect(this).into()
            }
        }
        unsafe extern "system" fn IsConnected<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbconnected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::IsConnected(this, core::mem::transmute_copy(&pbconnected)).into()
            }
        }
        unsafe extern "system" fn GetConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::GetConnectedTo(this, core::mem::transmute_copy(&ppconto)).into()
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrconnectorid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::GetConnectorIdConnectedTo(this, core::mem::transmute_copy(&ppwstrconnectorid)).into()
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IConnector_Impl::GetDeviceIdConnectedTo(this, core::mem::transmute_copy(&ppwstrdeviceid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetDataFlow: GetDataFlow::<Identity, OFFSET>,
            ConnectTo: ConnectTo::<Identity, OFFSET>,
            Disconnect: Disconnect::<Identity, OFFSET>,
            IsConnected: IsConnected::<Identity, OFFSET>,
            GetConnectedTo: GetConnectedTo::<Identity, OFFSET>,
            GetConnectorIdConnectedTo: GetConnectorIdConnectedTo::<Identity, OFFSET>,
            GetDeviceIdConnectedTo: GetDeviceIdConnectedTo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IConnector as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IConnector {}
windows_core::imp::define_interface!(IControlChangeNotify, IControlChangeNotify_Vtbl, 0xa09513ed_c709_4d21_bd7b_5f34c47f3947);
windows_core::imp::interface_hierarchy!(IControlChangeNotify, windows_core::IUnknown);
impl IControlChangeNotify {
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn OnNotify(&self, dwsenderprocessid: super::DWORD, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), dwsenderprocessid, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    OnNotify: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IControlChangeNotify_Impl: windows_core::IUnknownImpl {
    fn OnNotify(&self, dwsenderprocessid: super::DWORD, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IControlChangeNotify_Vtbl {
    pub const fn new<Identity: IControlChangeNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNotify<Identity: IControlChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsenderprocessid: super::DWORD, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlChangeNotify_Impl::OnNotify(this, core::mem::transmute_copy(&dwsenderprocessid), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlChangeNotify as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IControlChangeNotify {}
windows_core::imp::define_interface!(IControlInterface, IControlInterface_Vtbl, 0x45d37c3f_5140_444a_ae24_400789f3cbf3);
windows_core::imp::interface_hierarchy!(IControlInterface, windows_core::IUnknown);
impl IControlInterface {
    pub unsafe fn GetName(&self, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), ppwstrname as _) }
    }
    pub unsafe fn GetIID(&self, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), piid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlInterface_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetIID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IControlInterface_Impl: windows_core::IUnknownImpl {
    fn GetName(&self, ppwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetIID(&self, piid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IControlInterface_Vtbl {
    pub const fn new<Identity: IControlInterface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IControlInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlInterface_Impl::GetName(this, core::mem::transmute_copy(&ppwstrname)).into()
            }
        }
        unsafe extern "system" fn GetIID<Identity: IControlInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IControlInterface_Impl::GetIID(this, core::mem::transmute_copy(&piid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetName: GetName::<Identity, OFFSET>, GetIID: GetIID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IControlInterface as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IControlInterface {}
windows_core::imp::define_interface!(IDeviceSpecificProperty, IDeviceSpecificProperty_Vtbl, 0x3b22bcbf_2586_4af0_8583_205d391b807c);
windows_core::imp::interface_hierarchy!(IDeviceSpecificProperty, windows_core::IUnknown);
impl IDeviceSpecificProperty {
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetType(&self, pvtype: *mut super::VARTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), pvtype as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pvvalue as _, pcbvalue as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: super::DWORD, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), pvvalue, cbvalue, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Get4BRange(&self, plmin: *mut super::LONG, plmax: *mut super::LONG, plstepping: *mut super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Get4BRange)(windows_core::Interface::as_raw(self), plmin as _, plmax as _, plstepping as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceSpecificProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARTYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetType: usize,
    #[cfg(feature = "shared")]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetValue: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, super::DWORD, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetValue: usize,
    #[cfg(feature = "shared")]
    pub Get4BRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG, *mut super::LONG, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Get4BRange: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypes"))]
pub trait IDeviceSpecificProperty_Impl: windows_core::IUnknownImpl {
    fn GetType(&self, pvtype: *mut super::VARTYPE) -> windows_core::Result<()>;
    fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut super::DWORD) -> windows_core::Result<()>;
    fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: super::DWORD, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
    fn Get4BRange(&self, plmin: *mut super::LONG, plmax: *mut super::LONG, plstepping: *mut super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypes"))]
impl IDeviceSpecificProperty_Vtbl {
    pub const fn new<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *mut super::VARTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::GetType(this, core::mem::transmute_copy(&pvtype)).into()
            }
        }
        unsafe extern "system" fn GetValue<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::GetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&pcbvalue)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *const core::ffi::c_void, cbvalue: super::DWORD, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::SetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&cbvalue), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn Get4BRange<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut super::LONG, plmax: *mut super::LONG, plstepping: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::Get4BRange(this, core::mem::transmute_copy(&plmin), core::mem::transmute_copy(&plmax), core::mem::transmute_copy(&plstepping)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Get4BRange: Get4BRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceSpecificProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypes"))]
impl windows_core::RuntimeName for IDeviceSpecificProperty {}
windows_core::imp::define_interface!(IDeviceTopology, IDeviceTopology_Vtbl, 0x2a07407e_6497_4a18_9787_32f79bd0d98f);
windows_core::imp::interface_hierarchy!(IDeviceTopology, windows_core::IUnknown);
impl IDeviceTopology {
    #[cfg(feature = "shared")]
    pub unsafe fn GetConnectorCount(&self, pcount: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnectorCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetConnector(&self, nindex: super::UINT, ppconnector: *mut Option<IConnector>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetConnector)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute(ppconnector)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSubunitCount(&self, pcount: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubunitCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSubunit(&self, nindex: super::UINT, ppsubunit: *mut Option<ISubunit>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubunit)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute(ppsubunit)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPartById(&self, nid: super::UINT, pppart: *mut Option<IPart>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPartById)(windows_core::Interface::as_raw(self), nid, core::mem::transmute(pppart)) }
    }
    pub unsafe fn GetDeviceId(&self, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), ppwstrdeviceid as _) }
    }
    pub unsafe fn GetSignalPath<P0, P1>(&self, pipartfrom: P0, pipartto: P1, brejectmixedpaths: bool, ppparts: *mut Option<IPartsList>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IPart>,
        P1: windows_core::Param<IPart>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetSignalPath)(windows_core::Interface::as_raw(self), pipartfrom.param().abi(), pipartto.param().abi(), brejectmixedpaths.into(), core::mem::transmute(ppparts)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetConnectorCount: usize,
    #[cfg(feature = "shared")]
    pub GetConnector: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetConnector: usize,
    #[cfg(feature = "shared")]
    pub GetSubunitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSubunitCount: usize,
    #[cfg(feature = "shared")]
    pub GetSubunit: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSubunit: usize,
    #[cfg(feature = "shared")]
    pub GetPartById: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPartById: usize,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDeviceTopology_Impl: windows_core::IUnknownImpl {
    fn GetConnectorCount(&self, pcount: *mut super::UINT) -> windows_core::Result<()>;
    fn GetConnector(&self, nindex: super::UINT, ppconnector: windows_core::OutRef<IConnector>) -> windows_core::Result<()>;
    fn GetSubunitCount(&self, pcount: *mut super::UINT) -> windows_core::Result<()>;
    fn GetSubunit(&self, nindex: super::UINT, ppsubunit: windows_core::OutRef<ISubunit>) -> windows_core::Result<()>;
    fn GetPartById(&self, nid: super::UINT, pppart: windows_core::OutRef<IPart>) -> windows_core::Result<()>;
    fn GetDeviceId(&self, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetSignalPath(&self, pipartfrom: windows_core::Ref<IPart>, pipartto: windows_core::Ref<IPart>, brejectmixedpaths: windows_core::BOOL, ppparts: windows_core::OutRef<IPartsList>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDeviceTopology_Vtbl {
    pub const fn new<Identity: IDeviceTopology_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectorCount<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetConnectorCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetConnector<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: super::UINT, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetConnector(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppconnector)).into()
            }
        }
        unsafe extern "system" fn GetSubunitCount<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetSubunitCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetSubunit<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: super::UINT, ppsubunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetSubunit(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppsubunit)).into()
            }
        }
        unsafe extern "system" fn GetPartById<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nid: super::UINT, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetPartById(this, core::mem::transmute_copy(&nid), core::mem::transmute_copy(&pppart)).into()
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetDeviceId(this, core::mem::transmute_copy(&ppwstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn GetSignalPath<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipartfrom: *mut core::ffi::c_void, pipartto: *mut core::ffi::c_void, brejectmixedpaths: windows_core::BOOL, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceTopology_Impl::GetSignalPath(this, core::mem::transmute_copy(&pipartfrom), core::mem::transmute_copy(&pipartto), core::mem::transmute_copy(&brejectmixedpaths), core::mem::transmute_copy(&ppparts)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetConnectorCount: GetConnectorCount::<Identity, OFFSET>,
            GetConnector: GetConnector::<Identity, OFFSET>,
            GetSubunitCount: GetSubunitCount::<Identity, OFFSET>,
            GetSubunit: GetSubunit::<Identity, OFFSET>,
            GetPartById: GetPartById::<Identity, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, OFFSET>,
            GetSignalPath: GetSignalPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeviceTopology as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDeviceTopology {}
windows_core::imp::define_interface!(IKsControl, IKsControl_Vtbl, 0x28f54685_06fd_11d2_b27a_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsControl, windows_core::IUnknown);
impl IKsControl {
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub unsafe fn KsProperty(&self, property: super::PKSPROPERTY, propertylength: super::ULONG, propertydata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KsProperty)(windows_core::Interface::as_raw(self), property, propertylength, propertydata as _, datalength, bytesreturned as _) }
    }
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub unsafe fn KsMethod(&self, method: super::PKSMETHOD, methodlength: super::ULONG, methoddata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KsMethod)(windows_core::Interface::as_raw(self), method, methodlength, methoddata as _, datalength, bytesreturned as _) }
    }
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub unsafe fn KsEvent(&self, event: super::PKSEVENT, eventlength: super::ULONG, eventdata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KsEvent)(windows_core::Interface::as_raw(self), event, eventlength, eventdata as _, datalength, bytesreturned as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub KsProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::PKSPROPERTY, super::ULONG, *mut core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ks", feature = "shared")))]
    KsProperty: usize,
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub KsMethod: unsafe extern "system" fn(*mut core::ffi::c_void, super::PKSMETHOD, super::ULONG, *mut core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ks", feature = "shared")))]
    KsMethod: usize,
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub KsEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::PKSEVENT, super::ULONG, *mut core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ks", feature = "shared")))]
    KsEvent: usize,
}
#[cfg(all(feature = "ks", feature = "shared"))]
pub trait IKsControl_Impl: windows_core::IUnknownImpl {
    fn KsProperty(&self, property: super::PKSPROPERTY, propertylength: super::ULONG, propertydata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn KsMethod(&self, method: super::PKSMETHOD, methodlength: super::ULONG, methoddata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::Result<()>;
    fn KsEvent(&self, event: super::PKSEVENT, eventlength: super::ULONG, eventdata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ks", feature = "shared"))]
impl IKsControl_Vtbl {
    pub const fn new<Identity: IKsControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsProperty<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: super::PKSPROPERTY, propertylength: super::ULONG, propertydata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsControl_Impl::KsProperty(this, core::mem::transmute_copy(&property), core::mem::transmute_copy(&propertylength), core::mem::transmute_copy(&propertydata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        unsafe extern "system" fn KsMethod<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: super::PKSMETHOD, methodlength: super::ULONG, methoddata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsControl_Impl::KsMethod(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&methodlength), core::mem::transmute_copy(&methoddata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        unsafe extern "system" fn KsEvent<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: super::PKSEVENT, eventlength: super::ULONG, eventdata: *mut core::ffi::c_void, datalength: super::ULONG, bytesreturned: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsControl_Impl::KsEvent(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&eventlength), core::mem::transmute_copy(&eventdata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KsProperty: KsProperty::<Identity, OFFSET>,
            KsMethod: KsMethod::<Identity, OFFSET>,
            KsEvent: KsEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ks", feature = "shared"))]
impl windows_core::RuntimeName for IKsControl {}
windows_core::imp::define_interface!(IKsFormatSupport, IKsFormatSupport_Vtbl, 0x3cb4a69d_bb6f_4d2b_95b7_452d2c155db5);
windows_core::imp::interface_hierarchy!(IKsFormatSupport, windows_core::IUnknown);
impl IKsFormatSupport {
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub unsafe fn IsFormatSupported(&self, pksformat: super::PKSDATAFORMAT, cbformat: super::DWORD, pbsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsFormatSupported)(windows_core::Interface::as_raw(self), pksformat, cbformat, pbsupported as _) }
    }
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub unsafe fn GetDevicePreferredFormat(&self, ppksformat: *mut super::PKSDATAFORMAT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevicePreferredFormat)(windows_core::Interface::as_raw(self), ppksformat as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsFormatSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub IsFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::PKSDATAFORMAT, super::DWORD, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ks", feature = "shared")))]
    IsFormatSupported: usize,
    #[cfg(all(feature = "ks", feature = "shared"))]
    pub GetDevicePreferredFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PKSDATAFORMAT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ks", feature = "shared")))]
    GetDevicePreferredFormat: usize,
}
#[cfg(all(feature = "ks", feature = "shared"))]
pub trait IKsFormatSupport_Impl: windows_core::IUnknownImpl {
    fn IsFormatSupported(&self, pksformat: super::PKSDATAFORMAT, cbformat: super::DWORD, pbsupported: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn GetDevicePreferredFormat(&self, ppksformat: *mut super::PKSDATAFORMAT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ks", feature = "shared"))]
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: IKsFormatSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFormatSupported<Identity: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pksformat: super::PKSDATAFORMAT, cbformat: super::DWORD, pbsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsFormatSupport_Impl::IsFormatSupported(this, core::mem::transmute_copy(&pksformat), core::mem::transmute_copy(&cbformat), core::mem::transmute_copy(&pbsupported)).into()
            }
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppksformat: *mut super::PKSDATAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsFormatSupport_Impl::GetDevicePreferredFormat(this, core::mem::transmute_copy(&ppksformat)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Identity, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsFormatSupport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ks", feature = "shared"))]
impl windows_core::RuntimeName for IKsFormatSupport {}
windows_core::imp::define_interface!(IKsJackContainerId, IKsJackContainerId_Vtbl, 0xc99af463_d629_4ec4_8c00_e54d68154248);
windows_core::imp::interface_hierarchy!(IKsJackContainerId, windows_core::IUnknown);
impl IKsJackContainerId {
    pub unsafe fn GetJackContainerId(&self, pjackcontainerid: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackContainerId)(windows_core::Interface::as_raw(self), pjackcontainerid as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackContainerId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackContainerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IKsJackContainerId_Impl: windows_core::IUnknownImpl {
    fn GetJackContainerId(&self, pjackcontainerid: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: IKsJackContainerId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackContainerId<Identity: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjackcontainerid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackContainerId_Impl::GetJackContainerId(this, core::mem::transmute_copy(&pjackcontainerid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackContainerId: GetJackContainerId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackContainerId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IKsJackContainerId {}
windows_core::imp::define_interface!(IKsJackDescription, IKsJackDescription_Vtbl, 0x4509f757_2d46_4637_8e62_ce7db944f57b);
windows_core::imp::interface_hierarchy!(IKsJackDescription, windows_core::IUnknown);
impl IKsJackDescription {
    #[cfg(feature = "shared")]
    pub unsafe fn GetJackCount(&self, pcjacks: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), pcjacks as _) }
    }
    #[cfg(all(feature = "ksmedia", feature = "shared"))]
    pub unsafe fn GetJackDescription(&self, njack: super::UINT, pdescription: *mut super::KSJACK_DESCRIPTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackDescription)(windows_core::Interface::as_raw(self), njack, pdescription as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetJackCount: usize,
    #[cfg(all(feature = "ksmedia", feature = "shared"))]
    pub GetJackDescription: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::KSJACK_DESCRIPTION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "shared")))]
    GetJackDescription: usize,
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
pub trait IKsJackDescription_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self, pcjacks: *mut super::UINT) -> windows_core::Result<()>;
    fn GetJackDescription(&self, njack: super::UINT, pdescription: *mut super::KSJACK_DESCRIPTION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: IKsJackDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackDescription_Impl::GetJackCount(this, core::mem::transmute_copy(&pcjacks)).into()
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: super::UINT, pdescription: *mut super::KSJACK_DESCRIPTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackDescription_Impl::GetJackDescription(this, core::mem::transmute_copy(&njack), core::mem::transmute_copy(&pdescription)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
impl windows_core::RuntimeName for IKsJackDescription {}
windows_core::imp::define_interface!(IKsJackDescription2, IKsJackDescription2_Vtbl, 0x478f3a9b_e0c9_4827_9228_6f5505ffe76a);
windows_core::imp::interface_hierarchy!(IKsJackDescription2, windows_core::IUnknown);
impl IKsJackDescription2 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetJackCount(&self, pcjacks: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), pcjacks as _) }
    }
    #[cfg(all(feature = "ksmedia", feature = "shared"))]
    pub unsafe fn GetJackDescription2(&self, njack: super::UINT, pdescription2: *mut super::KSJACK_DESCRIPTION2) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackDescription2)(windows_core::Interface::as_raw(self), njack, pdescription2 as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetJackCount: usize,
    #[cfg(all(feature = "ksmedia", feature = "shared"))]
    pub GetJackDescription2: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::KSJACK_DESCRIPTION2) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "shared")))]
    GetJackDescription2: usize,
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
pub trait IKsJackDescription2_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self, pcjacks: *mut super::UINT) -> windows_core::Result<()>;
    fn GetJackDescription2(&self, njack: super::UINT, pdescription2: *mut super::KSJACK_DESCRIPTION2) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: IKsJackDescription2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackDescription2_Impl::GetJackCount(this, core::mem::transmute_copy(&pcjacks)).into()
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: super::UINT, pdescription2: *mut super::KSJACK_DESCRIPTION2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackDescription2_Impl::GetJackDescription2(this, core::mem::transmute_copy(&njack), core::mem::transmute_copy(&pdescription2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, OFFSET>,
            GetJackDescription2: GetJackDescription2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
impl windows_core::RuntimeName for IKsJackDescription2 {}
windows_core::imp::define_interface!(IKsJackDescription3, IKsJackDescription3_Vtbl, 0xe3f6778b_6660_4cc8_a291_ecc4192d9967);
windows_core::imp::interface_hierarchy!(IKsJackDescription3, windows_core::IUnknown);
impl IKsJackDescription3 {
    #[cfg(feature = "shared")]
    pub unsafe fn GetJackCount(&self, pcjacks: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), pcjacks as _) }
    }
    #[cfg(all(feature = "ksmedia", feature = "shared"))]
    pub unsafe fn GetJackDescription3(&self, njack: super::UINT, pdescription3: *mut super::KSJACK_DESCRIPTION3) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackDescription3)(windows_core::Interface::as_raw(self), njack, pdescription3 as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetJackCount: usize,
    #[cfg(all(feature = "ksmedia", feature = "shared"))]
    pub GetJackDescription3: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut super::KSJACK_DESCRIPTION3) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "shared")))]
    GetJackDescription3: usize,
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
pub trait IKsJackDescription3_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self, pcjacks: *mut super::UINT) -> windows_core::Result<()>;
    fn GetJackDescription3(&self, njack: super::UINT, pdescription3: *mut super::KSJACK_DESCRIPTION3) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
impl IKsJackDescription3_Vtbl {
    pub const fn new<Identity: IKsJackDescription3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackDescription3_Impl::GetJackCount(this, core::mem::transmute_copy(&pcjacks)).into()
            }
        }
        unsafe extern "system" fn GetJackDescription3<Identity: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: super::UINT, pdescription3: *mut super::KSJACK_DESCRIPTION3) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackDescription3_Impl::GetJackDescription3(this, core::mem::transmute_copy(&njack), core::mem::transmute_copy(&pdescription3)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, OFFSET>,
            GetJackDescription3: GetJackDescription3::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackDescription3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "shared"))]
impl windows_core::RuntimeName for IKsJackDescription3 {}
windows_core::imp::define_interface!(IKsJackSinkInformation, IKsJackSinkInformation_Vtbl, 0xd9bd72ed_290f_4581_9ff3_61027a8fe532);
windows_core::imp::interface_hierarchy!(IKsJackSinkInformation, windows_core::IUnknown);
impl IKsJackSinkInformation {
    #[cfg(all(feature = "ksmedia", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetJackSinkInformation(&self, pjacksinkinformation: *mut super::KSJACK_SINK_INFORMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackSinkInformation)(windows_core::Interface::as_raw(self), pjacksinkinformation as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackSinkInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "ksmedia", feature = "shared", feature = "winnt"))]
    pub GetJackSinkInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::KSJACK_SINK_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "shared", feature = "winnt")))]
    GetJackSinkInformation: usize,
}
#[cfg(all(feature = "ksmedia", feature = "shared", feature = "winnt"))]
pub trait IKsJackSinkInformation_Impl: windows_core::IUnknownImpl {
    fn GetJackSinkInformation(&self, pjacksinkinformation: *mut super::KSJACK_SINK_INFORMATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "shared", feature = "winnt"))]
impl IKsJackSinkInformation_Vtbl {
    pub const fn new<Identity: IKsJackSinkInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackSinkInformation<Identity: IKsJackSinkInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjacksinkinformation: *mut super::KSJACK_SINK_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsJackSinkInformation_Impl::GetJackSinkInformation(this, core::mem::transmute_copy(&pjacksinkinformation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ksmedia", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IKsJackSinkInformation {}
windows_core::imp::define_interface!(IPart, IPart_Vtbl, 0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
windows_core::imp::interface_hierarchy!(IPart, windows_core::IUnknown);
impl IPart {
    pub unsafe fn GetName(&self, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), ppwstrname as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLocalId(&self, pnid: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLocalId)(windows_core::Interface::as_raw(self), pnid as _) }
    }
    pub unsafe fn GetGlobalId(&self, ppwstrglobalid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetGlobalId)(windows_core::Interface::as_raw(self), ppwstrglobalid as _) }
    }
    pub unsafe fn GetPartType(&self, pparttype: *mut PartType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPartType)(windows_core::Interface::as_raw(self), pparttype as _) }
    }
    pub unsafe fn GetSubType(&self, psubtype: *mut windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self), psubtype as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetControlInterfaceCount(&self, pcount: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetControlInterfaceCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetControlInterface(&self, nindex: super::UINT, ppinterfacedesc: *mut Option<IControlInterface>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetControlInterface)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute(ppinterfacedesc)) }
    }
    pub unsafe fn EnumPartsIncoming(&self, ppparts: *mut Option<IPartsList>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumPartsIncoming)(windows_core::Interface::as_raw(self), core::mem::transmute(ppparts)) }
    }
    pub unsafe fn EnumPartsOutgoing(&self, ppparts: *mut Option<IPartsList>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnumPartsOutgoing)(windows_core::Interface::as_raw(self), core::mem::transmute(ppparts)) }
    }
    pub unsafe fn GetTopologyObject(&self, pptopology: *mut Option<IDeviceTopology>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTopologyObject)(windows_core::Interface::as_raw(self), core::mem::transmute(pptopology)) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn Activate<T>(&self, dwclscontext: super::DWORD, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), dwclscontext, &T::IID, result__ as *mut _ as *mut _).ok() }
    }
    pub unsafe fn RegisterControlChangeCallback<P1>(&self, riid: *const windows_core::GUID, pnotify: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IControlChangeNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterControlChangeCallback)(windows_core::Interface::as_raw(self), riid, pnotify.param().abi()) }
    }
    pub unsafe fn UnregisterControlChangeCallback<P0>(&self, pnotify: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IControlChangeNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterControlChangeCallback)(windows_core::Interface::as_raw(self), pnotify.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPart_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLocalId: usize,
    pub GetGlobalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPartType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PartType) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetControlInterfaceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetControlInterfaceCount: usize,
    #[cfg(feature = "shared")]
    pub GetControlInterface: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetControlInterface: usize,
    pub EnumPartsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumPartsOutgoing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTopologyObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    Activate: usize,
    pub RegisterControlChangeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterControlChangeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IPart_Impl: windows_core::IUnknownImpl {
    fn GetName(&self, ppwstrname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetLocalId(&self, pnid: *mut super::UINT) -> windows_core::Result<()>;
    fn GetGlobalId(&self, ppwstrglobalid: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetPartType(&self, pparttype: *mut PartType) -> windows_core::Result<()>;
    fn GetSubType(&self, psubtype: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetControlInterfaceCount(&self, pcount: *mut super::UINT) -> windows_core::Result<()>;
    fn GetControlInterface(&self, nindex: super::UINT, ppinterfacedesc: windows_core::OutRef<IControlInterface>) -> windows_core::Result<()>;
    fn EnumPartsIncoming(&self, ppparts: windows_core::OutRef<IPartsList>) -> windows_core::Result<()>;
    fn EnumPartsOutgoing(&self, ppparts: windows_core::OutRef<IPartsList>) -> windows_core::Result<()>;
    fn GetTopologyObject(&self, pptopology: windows_core::OutRef<IDeviceTopology>) -> windows_core::Result<()>;
    fn Activate(&self, dwclscontext: super::DWORD, refiid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RegisterControlChangeCallback(&self, riid: *const windows_core::GUID, pnotify: windows_core::Ref<IControlChangeNotify>) -> windows_core::Result<()>;
    fn UnregisterControlChangeCallback(&self, pnotify: windows_core::Ref<IControlChangeNotify>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IPart_Vtbl {
    pub const fn new<Identity: IPart_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetName(this, core::mem::transmute_copy(&ppwstrname)).into()
            }
        }
        unsafe extern "system" fn GetLocalId<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnid: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetLocalId(this, core::mem::transmute_copy(&pnid)).into()
            }
        }
        unsafe extern "system" fn GetGlobalId<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrglobalid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetGlobalId(this, core::mem::transmute_copy(&ppwstrglobalid)).into()
            }
        }
        unsafe extern "system" fn GetPartType<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparttype: *mut PartType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetPartType(this, core::mem::transmute_copy(&pparttype)).into()
            }
        }
        unsafe extern "system" fn GetSubType<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetSubType(this, core::mem::transmute_copy(&psubtype)).into()
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetControlInterfaceCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetControlInterface<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: super::UINT, ppinterfacedesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetControlInterface(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&ppinterfacedesc)).into()
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::EnumPartsIncoming(this, core::mem::transmute_copy(&ppparts)).into()
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::EnumPartsOutgoing(this, core::mem::transmute_copy(&ppparts)).into()
            }
        }
        unsafe extern "system" fn GetTopologyObject<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::GetTopologyObject(this, core::mem::transmute_copy(&pptopology)).into()
            }
        }
        unsafe extern "system" fn Activate<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: super::DWORD, refiid: *const super::IID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::Activate(this, core::mem::transmute_copy(&dwclscontext), core::mem::transmute_copy(&refiid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::RegisterControlChangeCallback(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPart_Impl::UnregisterControlChangeCallback(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetLocalId: GetLocalId::<Identity, OFFSET>,
            GetGlobalId: GetGlobalId::<Identity, OFFSET>,
            GetPartType: GetPartType::<Identity, OFFSET>,
            GetSubType: GetSubType::<Identity, OFFSET>,
            GetControlInterfaceCount: GetControlInterfaceCount::<Identity, OFFSET>,
            GetControlInterface: GetControlInterface::<Identity, OFFSET>,
            EnumPartsIncoming: EnumPartsIncoming::<Identity, OFFSET>,
            EnumPartsOutgoing: EnumPartsOutgoing::<Identity, OFFSET>,
            GetTopologyObject: GetTopologyObject::<Identity, OFFSET>,
            Activate: Activate::<Identity, OFFSET>,
            RegisterControlChangeCallback: RegisterControlChangeCallback::<Identity, OFFSET>,
            UnregisterControlChangeCallback: UnregisterControlChangeCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPart as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IPart {}
windows_core::imp::define_interface!(IPartsList, IPartsList_Vtbl, 0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
windows_core::imp::interface_hierarchy!(IPartsList, windows_core::IUnknown);
impl IPartsList {
    #[cfg(feature = "shared")]
    pub unsafe fn GetCount(&self, pcount: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), pcount as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPart(&self, nindex: super::UINT, pppart: *mut Option<IPart>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPart)(windows_core::Interface::as_raw(self), nindex, core::mem::transmute(pppart)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCount: usize,
    #[cfg(feature = "shared")]
    pub GetPart: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPart: usize,
}
#[cfg(feature = "shared")]
pub trait IPartsList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self, pcount: *mut super::UINT) -> windows_core::Result<()>;
    fn GetPart(&self, nindex: super::UINT, pppart: windows_core::OutRef<IPart>) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IPartsList_Vtbl {
    pub const fn new<Identity: IPartsList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPartsList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartsList_Impl::GetCount(this, core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn GetPart<Identity: IPartsList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: super::UINT, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPartsList_Impl::GetPart(this, core::mem::transmute_copy(&nindex), core::mem::transmute_copy(&pppart)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetPart: GetPart::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartsList as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IPartsList {}
windows_core::imp::define_interface!(IPerChannelDbLevel, IPerChannelDbLevel_Vtbl, 0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
windows_core::imp::interface_hierarchy!(IPerChannelDbLevel, windows_core::IUnknown);
impl IPerChannelDbLevel {
    #[cfg(feature = "shared")]
    pub unsafe fn GetChannelCount(&self, pcchannels: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), pcchannels as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevelRange(&self, nchannel: super::UINT, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelRange)(windows_core::Interface::as_raw(self), nchannel, pfminleveldb as _, pfmaxleveldb as _, pfstepping as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetLevel(&self, nchannel: super::UINT, pfleveldb: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, pfleveldb as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetLevel(&self, nchannel: super::UINT, fleveldb: f32, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "guiddef")]
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevelUniform)(windows_core::Interface::as_raw(self), fleveldb, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: Option<super::LPCGUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevelAllChannels)(windows_core::Interface::as_raw(self), alevelsdb.as_ptr(), alevelsdb.len().try_into().unwrap(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetChannelCount: usize,
    #[cfg(feature = "shared")]
    pub GetLevelRange: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLevelRange: usize,
    #[cfg(feature = "shared")]
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetLevel: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, f32, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetLevel: usize,
    #[cfg(feature = "guiddef")]
    pub SetLevelUniform: unsafe extern "system" fn(*mut core::ffi::c_void, f32, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "guiddef"))]
    SetLevelUniform: usize,
    #[cfg(all(feature = "guiddef", feature = "shared"))]
    pub SetLevelAllChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, super::ULONG, super::LPCGUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "shared")))]
    SetLevelAllChannels: usize,
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
pub trait IPerChannelDbLevel_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self, pcchannels: *mut super::UINT) -> windows_core::Result<()>;
    fn GetLevelRange(&self, nchannel: super::UINT, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::Result<()>;
    fn GetLevel(&self, nchannel: super::UINT, pfleveldb: *mut f32) -> windows_core::Result<()>;
    fn SetLevel(&self, nchannel: super::UINT, fleveldb: f32, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
    fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
    fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: super::ULONG, pguideventcontext: super::LPCGUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl IPerChannelDbLevel_Vtbl {
    pub const fn new<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::GetChannelCount(this, core::mem::transmute_copy(&pcchannels)).into()
            }
        }
        unsafe extern "system" fn GetLevelRange<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: super::UINT, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::GetLevelRange(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&pfminleveldb), core::mem::transmute_copy(&pfmaxleveldb), core::mem::transmute_copy(&pfstepping)).into()
            }
        }
        unsafe extern "system" fn GetLevel<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: super::UINT, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&pfleveldb)).into()
            }
        }
        unsafe extern "system" fn SetLevel<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: super::UINT, fleveldb: f32, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetLevelUniform<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevelUniform(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetLevelAllChannels<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alevelsdb: *const f32, cchannels: super::ULONG, pguideventcontext: super::LPCGUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevelAllChannels(this, core::mem::transmute_copy(&alevelsdb), core::mem::transmute_copy(&cchannels), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            GetLevelRange: GetLevelRange::<Identity, OFFSET>,
            GetLevel: GetLevel::<Identity, OFFSET>,
            SetLevel: SetLevel::<Identity, OFFSET>,
            SetLevelUniform: SetLevelUniform::<Identity, OFFSET>,
            SetLevelAllChannels: SetLevelAllChannels::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "shared"))]
impl windows_core::RuntimeName for IPerChannelDbLevel {}
windows_core::imp::define_interface!(ISubunit, ISubunit_Vtbl, 0x82149a85_dba6_4487_86bb_ea8f7fefcc71);
windows_core::imp::interface_hierarchy!(ISubunit, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct ISubunit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait ISubunit_Impl: windows_core::IUnknownImpl {}
impl ISubunit_Vtbl {
    pub const fn new<Identity: ISubunit_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISubunit as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISubunit {}
pub const In: DataFlow = 0;
pub const Network: ConnectorType = 5;
pub const Out: DataFlow = 1;
pub type PartType = i32;
pub const Physical_External: ConnectorType = 2;
pub const Physical_Internal: ConnectorType = 1;
pub const Software_Fixed: ConnectorType = 4;
pub const Software_IO: ConnectorType = 3;
pub const Subunit: PartType = 1;
pub const Unknown_Connector: ConnectorType = 0;
