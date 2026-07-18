pub const Connector: PartType = 0;
pub type ConnectorType = i32;
pub const DEVTOPO_HARDWARE_INITIATED_EVENTCONTEXT: u32 = 1685217608;
pub type DataFlow = i32;
pub const DeviceTopology: windows_core::GUID = windows_core::GUID::from_u128(0x1df639d0_5ec1_47aa_9379_828dc1aa8c59);
pub const EVENTCONTEXT_VOLUMESLIDER: windows_core::GUID = windows_core::GUID::from_u128(0xe2c2e9de_09b1_4b04_84e5_07931225ee04);
windows_core::imp::define_interface!(IAudioAutoGainControl, IAudioAutoGainControl_Vtbl, 0x85401fd4_6de4_4b9d_9869_2d6753a82f3c);
windows_core::imp::interface_hierarchy!(IAudioAutoGainControl, windows_core::IUnknown);
impl IAudioAutoGainControl {
    pub unsafe fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, benable: bool, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAutoGainControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioAutoGainControl_Impl: windows_core::IUnknownImpl {
    fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnabled(&self, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioAutoGainControl_Vtbl {
    pub const fn new<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabled<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioAutoGainControl_Impl::GetEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
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
pub trait IAudioBass_Impl: IPerChannelDbLevel_Impl {}
impl IAudioBass_Vtbl {
    pub const fn new<Identity: IAudioBass_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioBass as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioBass {}
windows_core::imp::define_interface!(IAudioChannelConfig, IAudioChannelConfig_Vtbl, 0xbb11c46f_ec28_493c_b88a_5db88062ce98);
windows_core::imp::interface_hierarchy!(IAudioChannelConfig, windows_core::IUnknown);
impl IAudioChannelConfig {
    pub unsafe fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelConfig)(windows_core::Interface::as_raw(self), dwconfig, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetChannelConfig(&self) -> windows_core::Result<u32> {
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
    pub SetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelConfig: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioChannelConfig_Impl: windows_core::IUnknownImpl {
    fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelConfig(&self) -> windows_core::Result<u32>;
}
impl IAudioChannelConfig_Vtbl {
    pub const fn new<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetChannelConfig<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfig: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioChannelConfig_Impl::SetChannelConfig(this, core::mem::transmute_copy(&dwconfig), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelConfig<Identity: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconfig: *mut u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IAudioChannelConfig {}
windows_core::imp::define_interface!(IAudioInputSelector, IAudioInputSelector_Vtbl, 0x4f03dc02_5e6e_4653_8f72_a030c123d598);
windows_core::imp::interface_hierarchy!(IAudioInputSelector, windows_core::IUnknown);
impl IAudioInputSelector {
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioInputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioInputSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioInputSelector_Vtbl {
    pub const fn new<Identity: IAudioInputSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioInputSelector_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pnidselected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelection<Identity: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IAudioInputSelector {}
windows_core::imp::define_interface!(IAudioLoudness, IAudioLoudness_Vtbl, 0x7d8b1437_dd53_4350_9c1b_1ee2890bd938);
windows_core::imp::interface_hierarchy!(IAudioLoudness, windows_core::IUnknown);
impl IAudioLoudness {
    pub unsafe fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEnabled(&self, benable: bool, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), benable.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioLoudness_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioLoudness_Impl: windows_core::IUnknownImpl {
    fn GetEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetEnabled(&self, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioLoudness_Vtbl {
    pub const fn new<Identity: IAudioLoudness_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnabled<Identity: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioLoudness_Impl::GetEnabled(this) {
                    Ok(ok__) => {
                        pbenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benable: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
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
pub trait IAudioMidrange_Impl: IPerChannelDbLevel_Impl {}
impl IAudioMidrange_Vtbl {
    pub const fn new<Identity: IAudioMidrange_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMidrange as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioMidrange {}
windows_core::imp::define_interface!(IAudioMute, IAudioMute_Vtbl, 0xdf45aeea_b74a_4b6b_afad_2366b6aa012e);
windows_core::imp::interface_hierarchy!(IAudioMute, windows_core::IUnknown);
impl IAudioMute {
    pub unsafe fn SetMute(&self, bmuted: bool, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmuted.into(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMute_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IAudioMute_Impl: windows_core::IUnknownImpl {
    fn SetMute(&self, bmuted: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IAudioMute_Vtbl {
    pub const fn new<Identity: IAudioMute_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMute<Identity: IAudioMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmuted: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioMute_Impl::SetMute(this, core::mem::transmute_copy(&bmuted), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IAudioMute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmuted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMute_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmuted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetMute: SetMute::<Identity, OFFSET>, GetMute: GetMute::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMute as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioMute {}
windows_core::imp::define_interface!(IAudioOutputSelector, IAudioOutputSelector_Vtbl, 0xbb515f69_94a7_429e_8b9c_271b3f11a3ab);
windows_core::imp::interface_hierarchy!(IAudioOutputSelector, windows_core::IUnknown);
impl IAudioOutputSelector {
    pub unsafe fn GetSelection(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSelection(&self, nidselect: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSelection)(windows_core::Interface::as_raw(self), nidselect, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioOutputSelector_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IAudioOutputSelector_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IAudioOutputSelector_Vtbl {
    pub const fn new<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnidselected: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioOutputSelector_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pnidselected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelection<Identity: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nidselect: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IAudioOutputSelector {}
windows_core::imp::define_interface!(IAudioPeakMeter, IAudioPeakMeter_Vtbl, 0xdd79923c_0599_45e0_b8b6_c8df7db6e796);
windows_core::imp::interface_hierarchy!(IAudioPeakMeter, windows_core::IUnknown);
impl IAudioPeakMeter {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioPeakMeter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioPeakMeter_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
}
impl IAudioPeakMeter_Vtbl {
    pub const fn new<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioPeakMeter_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pcchannels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLevel<Identity: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioPeakMeter_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
pub trait IAudioTreble_Impl: IPerChannelDbLevel_Impl {}
impl IAudioTreble_Vtbl {
    pub const fn new<Identity: IAudioTreble_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioTreble as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
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
pub trait IAudioVolumeLevel_Impl: IPerChannelDbLevel_Impl {}
impl IAudioVolumeLevel_Vtbl {
    pub const fn new<Identity: IAudioVolumeLevel_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioVolumeLevel as windows_core::Interface>::IID || iid == &<IPerChannelDbLevel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioVolumeLevel {}
windows_core::imp::define_interface!(IConnector, IConnector_Vtbl, 0x9c2c4058_23f5_41de_877a_df3af236a09e);
windows_core::imp::interface_hierarchy!(IConnector, windows_core::IUnknown);
impl IConnector {
    pub unsafe fn GetType(&self) -> windows_core::Result<ConnectorType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataFlow(&self) -> windows_core::Result<DataFlow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
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
    pub unsafe fn IsConnected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsConnected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnectedTo(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConnectorIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectorIdConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceIdConnectedTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
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
    fn GetType(&self) -> windows_core::Result<ConnectorType>;
    fn GetDataFlow(&self) -> windows_core::Result<DataFlow>;
    fn ConnectTo(&self, pconnectto: windows_core::Ref<IConnector>) -> windows_core::Result<()>;
    fn Disconnect(&self) -> windows_core::Result<()>;
    fn IsConnected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetConnectedTo(&self) -> windows_core::Result<IConnector>;
    fn GetConnectorIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetDeviceIdConnectedTo(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IConnector_Vtbl {
    pub const fn new<Identity: IConnector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut ConnectorType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataFlow<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflow: *mut DataFlow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetDataFlow(this) {
                    Ok(ok__) => {
                        pflow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
                match IConnector_Impl::IsConnected(this) {
                    Ok(ok__) => {
                        pbconnected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppconto: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetConnectedTo(this) {
                    Ok(ok__) => {
                        ppconto.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrconnectorid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetConnectorIdConnectedTo(this) {
                    Ok(ok__) => {
                        ppwstrconnectorid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Identity: IConnector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IConnector_Impl::GetDeviceIdConnectedTo(this) {
                    Ok(ok__) => {
                        ppwstrdeviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
    pub unsafe fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), dwsenderprocessid, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlChangeNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IControlChangeNotify_Impl: windows_core::IUnknownImpl {
    fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IControlChangeNotify_Vtbl {
    pub const fn new<Identity: IControlChangeNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNotify<Identity: IControlChangeNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IControlChangeNotify {}
windows_core::imp::define_interface!(IControlInterface, IControlInterface_Vtbl, 0x45d37c3f_5140_444a_ae24_400789f3cbf3);
windows_core::imp::interface_hierarchy!(IControlInterface, windows_core::IUnknown);
impl IControlInterface {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
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
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetIID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IControlInterface_Vtbl {
    pub const fn new<Identity: IControlInterface_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IControlInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlInterface_Impl::GetName(this) {
                    Ok(ok__) => {
                        ppwstrname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIID<Identity: IControlInterface_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IControlInterface_Impl::GetIID(this) {
                    Ok(ok__) => {
                        piid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
    pub unsafe fn GetType(&self) -> windows_core::Result<super::VARTYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pvvalue as _, pcbvalue as _) }
    }
    pub unsafe fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), pvvalue, cbvalue, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::HRESULT {
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
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Get4BRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IDeviceSpecificProperty_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> windows_core::Result<super::VARTYPE>;
    fn GetValue(&self, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::Result<()>;
    fn SetValue(&self, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IDeviceSpecificProperty_Vtbl {
    pub const fn new<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvtype: *mut super::VARTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceSpecificProperty_Impl::GetType(this) {
                    Ok(ok__) => {
                        pvtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetValue<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *mut core::ffi::c_void, pcbvalue: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::GetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&pcbvalue)).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvvalue: *const core::ffi::c_void, cbvalue: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeviceSpecificProperty_Impl::SetValue(this, core::mem::transmute_copy(&pvvalue), core::mem::transmute_copy(&cbvalue), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn Get4BRange<Identity: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> windows_core::HRESULT {
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
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IDeviceSpecificProperty {}
windows_core::imp::define_interface!(IDeviceTopology, IDeviceTopology_Vtbl, 0x2a07407e_6497_4a18_9787_32f79bd0d98f);
windows_core::imp::interface_hierarchy!(IDeviceTopology, windows_core::IUnknown);
impl IDeviceTopology {
    pub unsafe fn GetConnectorCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnector)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSubunitCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubunitCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubunit(&self, nindex: u32) -> windows_core::Result<ISubunit> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubunit)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPartById(&self, nid: u32) -> windows_core::Result<IPart> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartById)(windows_core::Interface::as_raw(self), nid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSignalPath<P0, P1>(&self, pipartfrom: P0, pipartto: P1, brejectmixedpaths: bool) -> windows_core::Result<IPartsList>
    where
        P0: windows_core::Param<IPart>,
        P1: windows_core::Param<IPart>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSignalPath)(windows_core::Interface::as_raw(self), pipartfrom.param().abi(), pipartto.param().abi(), brejectmixedpaths.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceTopology_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetConnectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetConnector: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSubunitCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSubunit: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSignalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDeviceTopology_Impl: windows_core::IUnknownImpl {
    fn GetConnectorCount(&self) -> windows_core::Result<u32>;
    fn GetConnector(&self, nindex: u32) -> windows_core::Result<IConnector>;
    fn GetSubunitCount(&self) -> windows_core::Result<u32>;
    fn GetSubunit(&self, nindex: u32) -> windows_core::Result<ISubunit>;
    fn GetPartById(&self, nid: u32) -> windows_core::Result<IPart>;
    fn GetDeviceId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSignalPath(&self, pipartfrom: windows_core::Ref<IPart>, pipartto: windows_core::Ref<IPart>, brejectmixedpaths: windows_core::BOOL) -> windows_core::Result<IPartsList>;
}
impl IDeviceTopology_Vtbl {
    pub const fn new<Identity: IDeviceTopology_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetConnectorCount<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetConnectorCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConnector<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppconnector: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetConnector(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppconnector.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubunitCount<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetSubunitCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubunit<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppsubunit: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetSubunit(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppsubunit.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartById<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nid: u32, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetPartById(this, core::mem::transmute_copy(&nid)) {
                    Ok(ok__) => {
                        pppart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrdeviceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetDeviceId(this) {
                    Ok(ok__) => {
                        ppwstrdeviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSignalPath<Identity: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipartfrom: *mut core::ffi::c_void, pipartto: *mut core::ffi::c_void, brejectmixedpaths: windows_core::BOOL, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDeviceTopology_Impl::GetSignalPath(this, core::mem::transmute_copy(&pipartfrom), core::mem::transmute_copy(&pipartto), core::mem::transmute_copy(&brejectmixedpaths)) {
                    Ok(ok__) => {
                        ppparts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IDeviceTopology {}
windows_core::imp::define_interface!(IKsControl, IKsControl_Vtbl, 0x28f54685_06fd_11d2_b27a_00a0c9223196);
windows_core::imp::interface_hierarchy!(IKsControl, windows_core::IUnknown);
impl IKsControl {
    #[cfg(feature = "ks")]
    pub unsafe fn KsProperty(&self, property: *const super::KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KsProperty)(windows_core::Interface::as_raw(self), property, propertylength, propertydata as _, datalength, bytesreturned as _) }
    }
    #[cfg(feature = "ks")]
    pub unsafe fn KsMethod(&self, method: *const super::KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KsMethod)(windows_core::Interface::as_raw(self), method, methodlength, methoddata as _, datalength, bytesreturned as _) }
    }
    #[cfg(feature = "ks")]
    pub unsafe fn KsEvent(&self, event: *const super::KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KsEvent)(windows_core::Interface::as_raw(self), event, eventlength, eventdata as _, datalength, bytesreturned as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ks")]
    pub KsProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::KSIDENTIFIER, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ks"))]
    KsProperty: usize,
    #[cfg(feature = "ks")]
    pub KsMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::KSIDENTIFIER, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ks"))]
    KsMethod: usize,
    #[cfg(feature = "ks")]
    pub KsEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::KSIDENTIFIER, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "ks"))]
    KsEvent: usize,
}
#[cfg(feature = "ks")]
pub trait IKsControl_Impl: windows_core::IUnknownImpl {
    fn KsProperty(&self, property: *const super::KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn KsMethod(&self, method: *const super::KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
    fn KsEvent(&self, event: *const super::KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "ks")]
impl IKsControl_Vtbl {
    pub const fn new<Identity: IKsControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KsProperty<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const super::KSIDENTIFIER, propertylength: u32, propertydata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsControl_Impl::KsProperty(this, core::mem::transmute_copy(&property), core::mem::transmute_copy(&propertylength), core::mem::transmute_copy(&propertydata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        unsafe extern "system" fn KsMethod<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *const super::KSIDENTIFIER, methodlength: u32, methoddata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKsControl_Impl::KsMethod(this, core::mem::transmute_copy(&method), core::mem::transmute_copy(&methodlength), core::mem::transmute_copy(&methoddata), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&bytesreturned)).into()
            }
        }
        unsafe extern "system" fn KsEvent<Identity: IKsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const super::KSIDENTIFIER, eventlength: u32, eventdata: *mut core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> windows_core::HRESULT {
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
#[cfg(feature = "ks")]
impl windows_core::RuntimeName for IKsControl {}
windows_core::imp::define_interface!(IKsFormatSupport, IKsFormatSupport_Vtbl, 0x3cb4a69d_bb6f_4d2b_95b7_452d2c155db5);
windows_core::imp::interface_hierarchy!(IKsFormatSupport, windows_core::IUnknown);
impl IKsFormatSupport {
    #[cfg(feature = "ks")]
    pub unsafe fn IsFormatSupported(&self, pksformat: *const super::KSDATARANGE, cbformat: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFormatSupported)(windows_core::Interface::as_raw(self), pksformat, cbformat, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ks")]
    pub unsafe fn GetDevicePreferredFormat(&self) -> windows_core::Result<super::PKSDATAFORMAT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevicePreferredFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsFormatSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "ks")]
    pub IsFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::KSDATARANGE, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "ks"))]
    IsFormatSupported: usize,
    #[cfg(feature = "ks")]
    pub GetDevicePreferredFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PKSDATAFORMAT) -> windows_core::HRESULT,
    #[cfg(not(feature = "ks"))]
    GetDevicePreferredFormat: usize,
}
#[cfg(feature = "ks")]
pub trait IKsFormatSupport_Impl: windows_core::IUnknownImpl {
    fn IsFormatSupported(&self, pksformat: *const super::KSDATARANGE, cbformat: u32) -> windows_core::Result<windows_core::BOOL>;
    fn GetDevicePreferredFormat(&self) -> windows_core::Result<super::PKSDATAFORMAT>;
}
#[cfg(feature = "ks")]
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: IKsFormatSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFormatSupported<Identity: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pksformat: *const super::KSDATARANGE, cbformat: u32, pbsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsFormatSupport_Impl::IsFormatSupported(this, core::mem::transmute_copy(&pksformat), core::mem::transmute_copy(&cbformat)) {
                    Ok(ok__) => {
                        pbsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppksformat: *mut super::PKSDATAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsFormatSupport_Impl::GetDevicePreferredFormat(this) {
                    Ok(ok__) => {
                        ppksformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(feature = "ks")]
impl windows_core::RuntimeName for IKsFormatSupport {}
windows_core::imp::define_interface!(IKsJackContainerId, IKsJackContainerId_Vtbl, 0xc99af463_d629_4ec4_8c00_e54d68154248);
windows_core::imp::interface_hierarchy!(IKsJackContainerId, windows_core::IUnknown);
impl IKsJackContainerId {
    pub unsafe fn GetJackContainerId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJackContainerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackContainerId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackContainerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IKsJackContainerId_Impl: windows_core::IUnknownImpl {
    fn GetJackContainerId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: IKsJackContainerId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackContainerId<Identity: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pjackcontainerid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsJackContainerId_Impl::GetJackContainerId(this) {
                    Ok(ok__) => {
                        pjackcontainerid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
    pub unsafe fn GetJackCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetJackDescription(&self, njack: u32, pdescription: *mut super::KSJACK_DESCRIPTION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackDescription)(windows_core::Interface::as_raw(self), njack, pdescription as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetJackDescription: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::KSJACK_DESCRIPTION) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetJackDescription: usize,
}
#[cfg(feature = "ksmedia")]
pub trait IKsJackDescription_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription(&self, njack: u32, pdescription: *mut super::KSJACK_DESCRIPTION) -> windows_core::Result<()>;
}
#[cfg(feature = "ksmedia")]
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: IKsJackDescription_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsJackDescription_Impl::GetJackCount(this) {
                    Ok(ok__) => {
                        pcjacks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription: *mut super::KSJACK_DESCRIPTION) -> windows_core::HRESULT {
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
#[cfg(feature = "ksmedia")]
impl windows_core::RuntimeName for IKsJackDescription {}
windows_core::imp::define_interface!(IKsJackDescription2, IKsJackDescription2_Vtbl, 0x478f3a9b_e0c9_4827_9228_6f5505ffe76a);
windows_core::imp::interface_hierarchy!(IKsJackDescription2, windows_core::IUnknown);
impl IKsJackDescription2 {
    pub unsafe fn GetJackCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetJackDescription2(&self, njack: u32) -> windows_core::Result<super::KSJACK_DESCRIPTION2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJackDescription2)(windows_core::Interface::as_raw(self), njack, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetJackDescription2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::KSJACK_DESCRIPTION2) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetJackDescription2: usize,
}
#[cfg(feature = "ksmedia")]
pub trait IKsJackDescription2_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription2(&self, njack: u32) -> windows_core::Result<super::KSJACK_DESCRIPTION2>;
}
#[cfg(feature = "ksmedia")]
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: IKsJackDescription2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsJackDescription2_Impl::GetJackCount(this) {
                    Ok(ok__) => {
                        pcjacks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription2: *mut super::KSJACK_DESCRIPTION2) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsJackDescription2_Impl::GetJackDescription2(this, core::mem::transmute_copy(&njack)) {
                    Ok(ok__) => {
                        pdescription2.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(feature = "ksmedia")]
impl windows_core::RuntimeName for IKsJackDescription2 {}
windows_core::imp::define_interface!(IKsJackDescription3, IKsJackDescription3_Vtbl, 0xe3f6778b_6660_4cc8_a291_ecc4192d9967);
windows_core::imp::interface_hierarchy!(IKsJackDescription3, windows_core::IUnknown);
impl IKsJackDescription3 {
    pub unsafe fn GetJackCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJackCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "ksmedia")]
    pub unsafe fn GetJackDescription3(&self, njack: u32) -> windows_core::Result<super::KSJACK_DESCRIPTION3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetJackDescription3)(windows_core::Interface::as_raw(self), njack, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackDescription3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetJackCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "ksmedia")]
    pub GetJackDescription3: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::KSJACK_DESCRIPTION3) -> windows_core::HRESULT,
    #[cfg(not(feature = "ksmedia"))]
    GetJackDescription3: usize,
}
#[cfg(feature = "ksmedia")]
pub trait IKsJackDescription3_Impl: windows_core::IUnknownImpl {
    fn GetJackCount(&self) -> windows_core::Result<u32>;
    fn GetJackDescription3(&self, njack: u32) -> windows_core::Result<super::KSJACK_DESCRIPTION3>;
}
#[cfg(feature = "ksmedia")]
impl IKsJackDescription3_Vtbl {
    pub const fn new<Identity: IKsJackDescription3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetJackCount<Identity: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcjacks: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsJackDescription3_Impl::GetJackCount(this) {
                    Ok(ok__) => {
                        pcjacks.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetJackDescription3<Identity: IKsJackDescription3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, njack: u32, pdescription3: *mut super::KSJACK_DESCRIPTION3) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKsJackDescription3_Impl::GetJackDescription3(this, core::mem::transmute_copy(&njack)) {
                    Ok(ok__) => {
                        pdescription3.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(feature = "ksmedia")]
impl windows_core::RuntimeName for IKsJackDescription3 {}
windows_core::imp::define_interface!(IKsJackSinkInformation, IKsJackSinkInformation_Vtbl, 0xd9bd72ed_290f_4581_9ff3_61027a8fe532);
windows_core::imp::interface_hierarchy!(IKsJackSinkInformation, windows_core::IUnknown);
impl IKsJackSinkInformation {
    #[cfg(all(feature = "ksmedia", feature = "winnt"))]
    pub unsafe fn GetJackSinkInformation(&self, pjacksinkinformation: *mut super::KSJACK_SINK_INFORMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetJackSinkInformation)(windows_core::Interface::as_raw(self), pjacksinkinformation as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKsJackSinkInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "ksmedia", feature = "winnt"))]
    pub GetJackSinkInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::KSJACK_SINK_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ksmedia", feature = "winnt")))]
    GetJackSinkInformation: usize,
}
#[cfg(all(feature = "ksmedia", feature = "winnt"))]
pub trait IKsJackSinkInformation_Impl: windows_core::IUnknownImpl {
    fn GetJackSinkInformation(&self, pjacksinkinformation: *mut super::KSJACK_SINK_INFORMATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "ksmedia", feature = "winnt"))]
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
#[cfg(all(feature = "ksmedia", feature = "winnt"))]
impl windows_core::RuntimeName for IKsJackSinkInformation {}
windows_core::imp::define_interface!(IPart, IPart_Vtbl, 0xae2de0e4_5bca_4f2d_aa46_5d13f8fdb3a9);
windows_core::imp::interface_hierarchy!(IPart, windows_core::IUnknown);
impl IPart {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGlobalId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlobalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPartType(&self) -> windows_core::Result<PartType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSubType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetControlInterfaceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControlInterfaceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetControlInterface(&self, nindex: u32) -> windows_core::Result<IControlInterface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControlInterface)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumPartsIncoming(&self) -> windows_core::Result<IPartsList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPartsIncoming)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumPartsOutgoing(&self) -> windows_core::Result<IPartsList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumPartsOutgoing)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTopologyObject(&self) -> windows_core::Result<IDeviceTopology> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopologyObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Activate(&self, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: Option<*mut *mut core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), dwclscontext, refiid, ppvobject.unwrap_or(core::mem::zeroed()) as _) }
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
    pub GetLocalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetGlobalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPartType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PartType) -> windows_core::HRESULT,
    pub GetSubType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetControlInterfaceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetControlInterface: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumPartsIncoming: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumPartsOutgoing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTopologyObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterControlChangeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterControlChangeCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPart_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetLocalId(&self) -> windows_core::Result<u32>;
    fn GetGlobalId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPartType(&self) -> windows_core::Result<PartType>;
    fn GetSubType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetControlInterfaceCount(&self) -> windows_core::Result<u32>;
    fn GetControlInterface(&self, nindex: u32) -> windows_core::Result<IControlInterface>;
    fn EnumPartsIncoming(&self) -> windows_core::Result<IPartsList>;
    fn EnumPartsOutgoing(&self) -> windows_core::Result<IPartsList>;
    fn GetTopologyObject(&self) -> windows_core::Result<IDeviceTopology>;
    fn Activate(&self, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RegisterControlChangeCallback(&self, riid: *const windows_core::GUID, pnotify: windows_core::Ref<IControlChangeNotify>) -> windows_core::Result<()>;
    fn UnregisterControlChangeCallback(&self, pnotify: windows_core::Ref<IControlChangeNotify>) -> windows_core::Result<()>;
}
impl IPart_Vtbl {
    pub const fn new<Identity: IPart_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetName(this) {
                    Ok(ok__) => {
                        ppwstrname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalId<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetLocalId(this) {
                    Ok(ok__) => {
                        pnid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGlobalId<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwstrglobalid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetGlobalId(this) {
                    Ok(ok__) => {
                        ppwstrglobalid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartType<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparttype: *mut PartType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetPartType(this) {
                    Ok(ok__) => {
                        pparttype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubType<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psubtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetSubType(this) {
                    Ok(ok__) => {
                        psubtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetControlInterfaceCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetControlInterface<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetControlInterface(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        ppinterfacedesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::EnumPartsIncoming(this) {
                    Ok(ok__) => {
                        ppparts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppparts: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::EnumPartsOutgoing(this) {
                    Ok(ok__) => {
                        ppparts.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTopologyObject<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPart_Impl::GetTopologyObject(this) {
                    Ok(ok__) => {
                        pptopology.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Activate<Identity: IPart_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclscontext: u32, refiid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IPart {}
windows_core::imp::define_interface!(IPartsList, IPartsList_Vtbl, 0x6daa848c_5eb0_45cc_aea5_998a2cda1ffb);
windows_core::imp::interface_hierarchy!(IPartsList, windows_core::IUnknown);
impl IPartsList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPart(&self, nindex: u32) -> windows_core::Result<IPart> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPart)(windows_core::Interface::as_raw(self), nindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPartsList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPartsList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetPart(&self, nindex: u32) -> windows_core::Result<IPart>;
}
impl IPartsList_Vtbl {
    pub const fn new<Identity: IPartsList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IPartsList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartsList_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPart<Identity: IPartsList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nindex: u32, pppart: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPartsList_Impl::GetPart(this, core::mem::transmute_copy(&nindex)) {
                    Ok(ok__) => {
                        pppart.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetPart: GetPart::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPartsList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPartsList {}
windows_core::imp::define_interface!(IPerChannelDbLevel, IPerChannelDbLevel_Vtbl, 0xc2f8e001_f205_4bc9_99bc_c13b1e048ccb);
windows_core::imp::interface_hierarchy!(IPerChannelDbLevel, windows_core::IUnknown);
impl IPerChannelDbLevel {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLevelRange)(windows_core::Interface::as_raw(self), nchannel, pfminleveldb as _, pfmaxleveldb as _, pfstepping as _) }
    }
    pub unsafe fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevelUniform)(windows_core::Interface::as_raw(self), fleveldb, pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetLevelAllChannels(&self, alevelsdb: &[f32], pguideventcontext: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLevelAllChannels)(windows_core::Interface::as_raw(self), alevelsdb.as_ptr(), alevelsdb.len().try_into().unwrap(), pguideventcontext.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPerChannelDbLevel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetLevelRange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetLevelUniform: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetLevelAllChannels: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IPerChannelDbLevel_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::Result<()>;
    fn GetLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IPerChannelDbLevel_Vtbl {
    pub const fn new<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcchannels: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerChannelDbLevel_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pcchannels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLevelRange<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::GetLevelRange(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&pfminleveldb), core::mem::transmute_copy(&pfmaxleveldb), core::mem::transmute_copy(&pfstepping)).into()
            }
        }
        unsafe extern "system" fn GetLevel<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPerChannelDbLevel_Impl::GetLevel(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pfleveldb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLevel<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetLevelUniform<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPerChannelDbLevel_Impl::SetLevelUniform(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetLevelAllChannels<Identity: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
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
