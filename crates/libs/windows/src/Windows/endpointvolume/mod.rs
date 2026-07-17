#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AUDIO_VOLUME_NOTIFICATION_DATA {
    pub guidEventContext: windows_core::GUID,
    pub bMuted: windows_core::BOOL,
    pub fMasterVolume: f32,
    pub nChannels: u32,
    pub afChannelVolumes: [f32; 1],
}
impl Default for AUDIO_VOLUME_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ENDPOINT_HARDWARE_SUPPORT_METER: u32 = 4;
pub const ENDPOINT_HARDWARE_SUPPORT_MUTE: u32 = 2;
pub const ENDPOINT_HARDWARE_SUPPORT_VOLUME: u32 = 1;
windows_core::imp::define_interface!(IAudioEndpointVolume, IAudioEndpointVolume_Vtbl, 0x5cdf2c82_841e_4546_9722_0cf74078229a);
windows_core::imp::interface_hierarchy!(IAudioEndpointVolume, windows_core::IUnknown);
impl IAudioEndpointVolume {
    pub unsafe fn RegisterControlChangeNotify<P0>(&self, pnotify: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioEndpointVolumeCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterControlChangeNotify)(windows_core::Interface::as_raw(self), pnotify.param().abi()) }
    }
    pub unsafe fn UnregisterControlChangeNotify<P0>(&self, pnotify: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioEndpointVolumeCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterControlChangeNotify)(windows_core::Interface::as_raw(self), pnotify.param().abi()) }
    }
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolumeLevel)(windows_core::Interface::as_raw(self), fleveldb, pguideventcontext) }
    }
    pub unsafe fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolumeLevelScalar)(windows_core::Interface::as_raw(self), flevel, pguideventcontext) }
    }
    pub unsafe fn GetMasterVolumeLevel(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolumeLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMasterVolumeLevelScalar(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolumeLevelScalar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumeLevel)(windows_core::Interface::as_raw(self), nchannel, fleveldb, pguideventcontext) }
    }
    pub unsafe fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumeLevelScalar)(windows_core::Interface::as_raw(self), nchannel, flevel, pguideventcontext) }
    }
    pub unsafe fn GetChannelVolumeLevel(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolumeLevel)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolumeLevelScalar)(windows_core::Interface::as_raw(self), nchannel, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMute(&self, bmute: bool, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.into(), pguideventcontext) }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeStepInfo)(windows_core::Interface::as_raw(self), pnstep as _, pnstepcount as _) }
    }
    pub unsafe fn VolumeStepUp(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).VolumeStepUp)(windows_core::Interface::as_raw(self), pguideventcontext) }
    }
    pub unsafe fn VolumeStepDown(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).VolumeStepDown)(windows_core::Interface::as_raw(self), pguideventcontext) }
    }
    pub unsafe fn QueryHardwareSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHardwareSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeRange)(windows_core::Interface::as_raw(self), pflvolumemindb as _, pflvolumemaxdb as _, pflvolumeincrementdb as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterControlChangeNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterControlChangeNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMasterVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetMasterVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMasterVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMasterVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetChannelVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetChannelVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelVolumeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub GetChannelVolumeLevelScalar: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetVolumeStepInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub VolumeStepUp: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub VolumeStepDown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetVolumeRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioEndpointVolume_Impl: windows_core::IUnknownImpl {
    fn RegisterControlChangeNotify(&self, pnotify: windows_core::Ref<IAudioEndpointVolumeCallback>) -> windows_core::Result<()>;
    fn UnregisterControlChangeNotify(&self, pnotify: windows_core::Ref<IAudioEndpointVolumeCallback>) -> windows_core::Result<()>;
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMasterVolumeLevel(&self) -> windows_core::Result<f32>;
    fn GetMasterVolumeLevelScalar(&self) -> windows_core::Result<f32>;
    fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelVolumeLevel(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::Result<()>;
    fn VolumeStepUp(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn VolumeStepDown(&self, pguideventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn QueryHardwareSupport(&self) -> windows_core::Result<u32>;
    fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()>;
}
impl IAudioEndpointVolume_Vtbl {
    pub const fn new<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterControlChangeNotify<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::RegisterControlChangeNotify(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::UnregisterControlChangeNotify(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn GetChannelCount<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnchannelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pnchannelcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetMasterVolumeLevel(this, core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetMasterVolumeLevelScalar(this, core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetMasterVolumeLevel(this) {
                    Ok(ok__) => {
                        pfleveldb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetMasterVolumeLevelScalar(this) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetChannelVolumeLevel(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&fleveldb), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetChannelVolumeLevelScalar(this, core::mem::transmute_copy(&nchannel), core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetChannelVolumeLevel(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pfleveldb.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetChannelVolumeLevelScalar(this, core::mem::transmute_copy(&nchannel)) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMute<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: windows_core::BOOL, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute), core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmute.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolumeStepInfo<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::GetVolumeStepInfo(this, core::mem::transmute_copy(&pnstep), core::mem::transmute_copy(&pnstepcount)).into()
            }
        }
        unsafe extern "system" fn VolumeStepUp<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::VolumeStepUp(this, core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn VolumeStepDown<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguideventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::VolumeStepDown(this, core::mem::transmute_copy(&pguideventcontext)).into()
            }
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioEndpointVolume_Impl::QueryHardwareSupport(this) {
                    Ok(ok__) => {
                        pdwhardwaresupportmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolumeRange<Identity: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolume_Impl::GetVolumeRange(this, core::mem::transmute_copy(&pflvolumemindb), core::mem::transmute_copy(&pflvolumemaxdb), core::mem::transmute_copy(&pflvolumeincrementdb)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterControlChangeNotify: RegisterControlChangeNotify::<Identity, OFFSET>,
            UnregisterControlChangeNotify: UnregisterControlChangeNotify::<Identity, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetMasterVolumeLevel: SetMasterVolumeLevel::<Identity, OFFSET>,
            SetMasterVolumeLevelScalar: SetMasterVolumeLevelScalar::<Identity, OFFSET>,
            GetMasterVolumeLevel: GetMasterVolumeLevel::<Identity, OFFSET>,
            GetMasterVolumeLevelScalar: GetMasterVolumeLevelScalar::<Identity, OFFSET>,
            SetChannelVolumeLevel: SetChannelVolumeLevel::<Identity, OFFSET>,
            SetChannelVolumeLevelScalar: SetChannelVolumeLevelScalar::<Identity, OFFSET>,
            GetChannelVolumeLevel: GetChannelVolumeLevel::<Identity, OFFSET>,
            GetChannelVolumeLevelScalar: GetChannelVolumeLevelScalar::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
            GetVolumeStepInfo: GetVolumeStepInfo::<Identity, OFFSET>,
            VolumeStepUp: VolumeStepUp::<Identity, OFFSET>,
            VolumeStepDown: VolumeStepDown::<Identity, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, OFFSET>,
            GetVolumeRange: GetVolumeRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointVolume {}
windows_core::imp::define_interface!(IAudioEndpointVolumeCallback, IAudioEndpointVolumeCallback_Vtbl, 0x657804fa_d6ad_4496_8a60_352752af4f89);
windows_core::imp::interface_hierarchy!(IAudioEndpointVolumeCallback, windows_core::IUnknown);
impl IAudioEndpointVolumeCallback {
    pub unsafe fn OnNotify(&self, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnNotify)(windows_core::Interface::as_raw(self), pnotify as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT,
}
pub trait IAudioEndpointVolumeCallback_Impl: windows_core::IUnknownImpl {
    fn OnNotify(&self, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::Result<()>;
}
impl IAudioEndpointVolumeCallback_Vtbl {
    pub const fn new<Identity: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnNotify<Identity: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolumeCallback_Impl::OnNotify(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointVolumeCallback {}
windows_core::imp::define_interface!(IAudioEndpointVolumeEx, IAudioEndpointVolumeEx_Vtbl, 0x66e11784_f695_4f28_a505_a7080081a78f);
impl core::ops::Deref for IAudioEndpointVolumeEx {
    type Target = IAudioEndpointVolume;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioEndpointVolumeEx, windows_core::IUnknown, IAudioEndpointVolume);
impl IAudioEndpointVolumeEx {
    pub unsafe fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVolumeRangeChannel)(windows_core::Interface::as_raw(self), ichannel, pflvolumemindb as _, pflvolumemaxdb as _, pflvolumeincrementdb as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEndpointVolumeEx_Vtbl {
    pub base__: IAudioEndpointVolume_Vtbl,
    pub GetVolumeRangeChannel: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioEndpointVolumeEx_Impl: IAudioEndpointVolume_Impl {
    fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::Result<()>;
}
impl IAudioEndpointVolumeEx_Vtbl {
    pub const fn new<Identity: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVolumeRangeChannel<Identity: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEndpointVolumeEx_Impl::GetVolumeRangeChannel(this, core::mem::transmute_copy(&ichannel), core::mem::transmute_copy(&pflvolumemindb), core::mem::transmute_copy(&pflvolumemaxdb), core::mem::transmute_copy(&pflvolumeincrementdb)).into()
            }
        }
        Self { base__: IAudioEndpointVolume_Vtbl::new::<Identity, OFFSET>(), GetVolumeRangeChannel: GetVolumeRangeChannel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeEx as windows_core::Interface>::IID || iid == &<IAudioEndpointVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEndpointVolumeEx {}
windows_core::imp::define_interface!(IAudioMeterInformation, IAudioMeterInformation_Vtbl, 0xc02216f6_8c67_4b5b_9d00_d008e73e0064);
windows_core::imp::interface_hierarchy!(IAudioMeterInformation, windows_core::IUnknown);
impl IAudioMeterInformation {
    pub unsafe fn GetPeakValue(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPeakValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMeteringChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMeteringChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChannelsPeakValues(&self, u32channelcount: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelsPeakValues)(windows_core::Interface::as_raw(self), u32channelcount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn QueryHardwareSupport(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHardwareSupport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioMeterInformation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPeakValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetMeteringChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetChannelsPeakValues: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub QueryHardwareSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioMeterInformation_Impl: windows_core::IUnknownImpl {
    fn GetPeakValue(&self) -> windows_core::Result<f32>;
    fn GetMeteringChannelCount(&self) -> windows_core::Result<u32>;
    fn GetChannelsPeakValues(&self, u32channelcount: u32) -> windows_core::Result<f32>;
    fn QueryHardwareSupport(&self) -> windows_core::Result<u32>;
}
impl IAudioMeterInformation_Vtbl {
    pub const fn new<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPeakValue<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfpeak: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::GetPeakValue(this) {
                    Ok(ok__) => {
                        pfpeak.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMeteringChannelCount<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnchannelcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::GetMeteringChannelCount(this) {
                    Ok(ok__) => {
                        pnchannelcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChannelsPeakValues<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::GetChannelsPeakValues(this, core::mem::transmute_copy(&u32channelcount)) {
                    Ok(ok__) => {
                        afpeakvalues.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioMeterInformation_Impl::QueryHardwareSupport(this) {
                    Ok(ok__) => {
                        pdwhardwaresupportmask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeakValue: GetPeakValue::<Identity, OFFSET>,
            GetMeteringChannelCount: GetMeteringChannelCount::<Identity, OFFSET>,
            GetChannelsPeakValues: GetChannelsPeakValues::<Identity, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioMeterInformation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioMeterInformation {}
pub type PAUDIO_VOLUME_NOTIFICATION_DATA = *mut AUDIO_VOLUME_NOTIFICATION_DATA;
