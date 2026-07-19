pub type AudioSessionDisconnectReason = i32;
pub const DisconnectReasonDeviceRemoval: AudioSessionDisconnectReason = 0;
pub const DisconnectReasonExclusiveModeOverride: AudioSessionDisconnectReason = 5;
pub const DisconnectReasonFormatChanged: AudioSessionDisconnectReason = 2;
pub const DisconnectReasonServerShutdown: AudioSessionDisconnectReason = 1;
pub const DisconnectReasonSessionDisconnected: AudioSessionDisconnectReason = 4;
pub const DisconnectReasonSessionLogoff: AudioSessionDisconnectReason = 3;
windows_core::imp::define_interface!(IAudioSessionControl, IAudioSessionControl_Vtbl, 0xf4b1a599_7266_4319_a8ca_e70acb11e8cd);
windows_core::imp::interface_hierarchy!(IAudioSessionControl, windows_core::IUnknown);
impl IAudioSessionControl {
    #[cfg(feature = "audiosessiontypes")]
    pub unsafe fn GetState(&self) -> windows_core::Result<super::AudioSessionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayName<P0>(&self, value: P0, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), value.param().abi(), eventcontext) }
    }
    pub unsafe fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIconPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIconPath<P0>(&self, value: P0, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIconPath)(windows_core::Interface::as_raw(self), value.param().abi(), eventcontext) }
    }
    pub unsafe fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroupingParam)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGroupingParam(&self, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGroupingParam)(windows_core::Interface::as_raw(self), r#override, eventcontext) }
    }
    pub unsafe fn RegisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSessionEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterAudioSessionNotification)(windows_core::Interface::as_raw(self), newnotifications.param().abi()) }
    }
    pub unsafe fn UnregisterAudioSessionNotification<P0>(&self, newnotifications: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSessionEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAudioSessionNotification)(windows_core::Interface::as_raw(self), newnotifications.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "audiosessiontypes")]
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AudioSessionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "audiosessiontypes"))]
    GetState: usize,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGroupingParam: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGroupingParam: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RegisterAudioSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAudioSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "audiosessiontypes")]
pub trait IAudioSessionControl_Impl: windows_core::IUnknownImpl {
    fn GetState(&self) -> windows_core::Result<super::AudioSessionState>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetDisplayName(&self, value: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIconPath(&self, value: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGroupingParam(&self, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RegisterAudioSessionNotification(&self, newnotifications: windows_core::Ref<IAudioSessionEvents>) -> windows_core::Result<()>;
    fn UnregisterAudioSessionNotification(&self, newnotifications: windows_core::Ref<IAudioSessionEvents>) -> windows_core::Result<()>;
}
#[cfg(feature = "audiosessiontypes")]
impl IAudioSessionControl_Vtbl {
    pub const fn new<Identity: IAudioSessionControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetState<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut super::AudioSessionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::SetDisplayName(this, core::mem::transmute(&value), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetIconPath<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetIconPath(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIconPath<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::SetIconPath(this, core::mem::transmute(&value), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetGroupingParam<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl_Impl::GetGroupingParam(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupingParam<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#override: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::SetGroupingParam(this, core::mem::transmute_copy(&r#override), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newnotifications: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::RegisterAudioSessionNotification(this, core::mem::transmute_copy(&newnotifications)).into()
            }
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Identity: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newnotifications: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl_Impl::UnregisterAudioSessionNotification(this, core::mem::transmute_copy(&newnotifications)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetIconPath: GetIconPath::<Identity, OFFSET>,
            SetIconPath: SetIconPath::<Identity, OFFSET>,
            GetGroupingParam: GetGroupingParam::<Identity, OFFSET>,
            SetGroupingParam: SetGroupingParam::<Identity, OFFSET>,
            RegisterAudioSessionNotification: RegisterAudioSessionNotification::<Identity, OFFSET>,
            UnregisterAudioSessionNotification: UnregisterAudioSessionNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "audiosessiontypes")]
impl windows_core::RuntimeName for IAudioSessionControl {}
windows_core::imp::define_interface!(IAudioSessionControl2, IAudioSessionControl2_Vtbl, 0xbfb7ff88_7239_4fc9_8fa2_07c950be9c6d);
impl core::ops::Deref for IAudioSessionControl2 {
    type Target = IAudioSessionControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionControl2, windows_core::IUnknown, IAudioSessionControl);
impl IAudioSessionControl2 {
    pub unsafe fn GetSessionIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSessionInstanceIdentifier(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionInstanceIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProcessId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSystemSoundsSession(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsSystemSoundsSession)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetDuckingPreference(&self, optout: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDuckingPreference)(windows_core::Interface::as_raw(self), optout.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionControl2_Vtbl {
    pub base__: IAudioSessionControl_Vtbl,
    pub GetSessionIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSessionInstanceIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsSystemSoundsSession: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDuckingPreference: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "audiosessiontypes")]
pub trait IAudioSessionControl2_Impl: IAudioSessionControl_Impl {
    fn GetSessionIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSessionInstanceIdentifier(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetProcessId(&self) -> windows_core::Result<u32>;
    fn IsSystemSoundsSession(&self) -> windows_core::Result<()>;
    fn SetDuckingPreference(&self, optout: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "audiosessiontypes")]
impl IAudioSessionControl2_Vtbl {
    pub const fn new<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSessionIdentifier<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl2_Impl::GetSessionIdentifier(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl2_Impl::GetSessionInstanceIdentifier(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProcessId<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionControl2_Impl::GetProcessId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSystemSoundsSession<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl2_Impl::IsSystemSoundsSession(this).into()
            }
        }
        unsafe extern "system" fn SetDuckingPreference<Identity: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, optout: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionControl2_Impl::SetDuckingPreference(this, core::mem::transmute_copy(&optout)).into()
            }
        }
        Self {
            base__: IAudioSessionControl_Vtbl::new::<Identity, OFFSET>(),
            GetSessionIdentifier: GetSessionIdentifier::<Identity, OFFSET>,
            GetSessionInstanceIdentifier: GetSessionInstanceIdentifier::<Identity, OFFSET>,
            GetProcessId: GetProcessId::<Identity, OFFSET>,
            IsSystemSoundsSession: IsSystemSoundsSession::<Identity, OFFSET>,
            SetDuckingPreference: SetDuckingPreference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionControl2 as windows_core::Interface>::IID || iid == &<IAudioSessionControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "audiosessiontypes")]
impl windows_core::RuntimeName for IAudioSessionControl2 {}
windows_core::imp::define_interface!(IAudioSessionEnumerator, IAudioSessionEnumerator_Vtbl, 0xe2f5bb11_0570_40ca_acdd_3aa01277dee8);
windows_core::imp::interface_hierarchy!(IAudioSessionEnumerator, windows_core::IUnknown);
impl IAudioSessionEnumerator {
    pub unsafe fn GetCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSession(&self, sessioncount: i32) -> windows_core::Result<IAudioSessionControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSession)(windows_core::Interface::as_raw(self), sessioncount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetSession: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetSession(&self, sessioncount: i32) -> windows_core::Result<IAudioSessionControl>;
}
impl IAudioSessionEnumerator_Vtbl {
    pub const fn new<Identity: IAudioSessionEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessioncount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionEnumerator_Impl::GetCount(this) {
                    Ok(ok__) => {
                        sessioncount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSession<Identity: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessioncount: i32, session: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionEnumerator_Impl::GetSession(this, core::mem::transmute_copy(&sessioncount)) {
                    Ok(ok__) => {
                        session.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetSession: GetSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioSessionEnumerator {}
windows_core::imp::define_interface!(IAudioSessionEvents, IAudioSessionEvents_Vtbl, 0x24918acc_64b3_37c1_8ca9_74a66e9957a8);
windows_core::imp::interface_hierarchy!(IAudioSessionEvents, windows_core::IUnknown);
impl IAudioSessionEvents {
    pub unsafe fn OnDisplayNameChanged<P0>(&self, newdisplayname: P0, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDisplayNameChanged)(windows_core::Interface::as_raw(self), newdisplayname.param().abi(), eventcontext) }
    }
    pub unsafe fn OnIconPathChanged<P0>(&self, newiconpath: P0, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnIconPathChanged)(windows_core::Interface::as_raw(self), newiconpath.param().abi(), eventcontext) }
    }
    pub unsafe fn OnSimpleVolumeChanged(&self, newvolume: f32, newmute: bool, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSimpleVolumeChanged)(windows_core::Interface::as_raw(self), newvolume, newmute.into(), eventcontext) }
    }
    pub unsafe fn OnChannelVolumeChanged(&self, newchannelvolumearray: &[f32], changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnChannelVolumeChanged)(windows_core::Interface::as_raw(self), newchannelvolumearray.len().try_into().unwrap(), newchannelvolumearray.as_ptr(), changedchannel, eventcontext) }
    }
    pub unsafe fn OnGroupingParamChanged(&self, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnGroupingParamChanged)(windows_core::Interface::as_raw(self), newgroupingparam, eventcontext) }
    }
    #[cfg(feature = "audiosessiontypes")]
    pub unsafe fn OnStateChanged(&self, newstate: super::AudioSessionState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), newstate) }
    }
    pub unsafe fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnSessionDisconnected)(windows_core::Interface::as_raw(self), disconnectreason) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDisplayNameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnIconPathChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnSimpleVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, f32, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnChannelVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub OnGroupingParamChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "audiosessiontypes")]
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, super::AudioSessionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "audiosessiontypes"))]
    OnStateChanged: usize,
    pub OnSessionDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, AudioSessionDisconnectReason) -> windows_core::HRESULT,
}
#[cfg(feature = "audiosessiontypes")]
pub trait IAudioSessionEvents_Impl: windows_core::IUnknownImpl {
    fn OnDisplayNameChanged(&self, newdisplayname: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnIconPathChanged(&self, newiconpath: &windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnSimpleVolumeChanged(&self, newvolume: f32, newmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnChannelVolumeChanged(&self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnGroupingParamChanged(&self, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OnStateChanged(&self, newstate: super::AudioSessionState) -> windows_core::Result<()>;
    fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> windows_core::Result<()>;
}
#[cfg(feature = "audiosessiontypes")]
impl IAudioSessionEvents_Vtbl {
    pub const fn new<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDisplayNameChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newdisplayname: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnDisplayNameChanged(this, core::mem::transmute(&newdisplayname), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnIconPathChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newiconpath: windows_core::PCWSTR, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnIconPathChanged(this, core::mem::transmute(&newiconpath), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newvolume: f32, newmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnSimpleVolumeChanged(this, core::mem::transmute_copy(&newvolume), core::mem::transmute_copy(&newmute), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnChannelVolumeChanged(this, core::mem::transmute_copy(&channelcount), core::mem::transmute_copy(&newchannelvolumearray), core::mem::transmute_copy(&changedchannel), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnGroupingParamChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newgroupingparam: *const windows_core::GUID, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnGroupingParamChanged(this, core::mem::transmute_copy(&newgroupingparam), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn OnStateChanged<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newstate: super::AudioSessionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnStateChanged(this, core::mem::transmute_copy(&newstate)).into()
            }
        }
        unsafe extern "system" fn OnSessionDisconnected<Identity: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionEvents_Impl::OnSessionDisconnected(this, core::mem::transmute_copy(&disconnectreason)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDisplayNameChanged: OnDisplayNameChanged::<Identity, OFFSET>,
            OnIconPathChanged: OnIconPathChanged::<Identity, OFFSET>,
            OnSimpleVolumeChanged: OnSimpleVolumeChanged::<Identity, OFFSET>,
            OnChannelVolumeChanged: OnChannelVolumeChanged::<Identity, OFFSET>,
            OnGroupingParamChanged: OnGroupingParamChanged::<Identity, OFFSET>,
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnSessionDisconnected: OnSessionDisconnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "audiosessiontypes")]
impl windows_core::RuntimeName for IAudioSessionEvents {}
windows_core::imp::define_interface!(IAudioSessionManager, IAudioSessionManager_Vtbl, 0xbfa971f1_4d5e_40bb_935e_967039bfbee4);
windows_core::imp::interface_hierarchy!(IAudioSessionManager, windows_core::IUnknown);
impl IAudioSessionManager {
    pub unsafe fn GetAudioSessionControl(&self, audiosessionguid: Option<*const windows_core::GUID>, streamflags: u32) -> windows_core::Result<IAudioSessionControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioSessionControl)(windows_core::Interface::as_raw(self), audiosessionguid.unwrap_or(core::mem::zeroed()) as _, streamflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "audioclient")]
    pub unsafe fn GetSimpleAudioVolume(&self, audiosessionguid: Option<*const windows_core::GUID>, streamflags: u32) -> windows_core::Result<super::ISimpleAudioVolume> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSimpleAudioVolume)(windows_core::Interface::as_raw(self), audiosessionguid.unwrap_or(core::mem::zeroed()) as _, streamflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAudioSessionControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "audioclient")]
    pub GetSimpleAudioVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "audioclient"))]
    GetSimpleAudioVolume: usize,
}
#[cfg(feature = "audioclient")]
pub trait IAudioSessionManager_Impl: windows_core::IUnknownImpl {
    fn GetAudioSessionControl(&self, audiosessionguid: *const windows_core::GUID, streamflags: u32) -> windows_core::Result<IAudioSessionControl>;
    fn GetSimpleAudioVolume(&self, audiosessionguid: *const windows_core::GUID, streamflags: u32) -> windows_core::Result<super::ISimpleAudioVolume>;
}
#[cfg(feature = "audioclient")]
impl IAudioSessionManager_Vtbl {
    pub const fn new<Identity: IAudioSessionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAudioSessionControl<Identity: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *const windows_core::GUID, streamflags: u32, sessioncontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionManager_Impl::GetAudioSessionControl(this, core::mem::transmute_copy(&audiosessionguid), core::mem::transmute_copy(&streamflags)) {
                    Ok(ok__) => {
                        sessioncontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Identity: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audiosessionguid: *const windows_core::GUID, streamflags: u32, audiovolume: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionManager_Impl::GetSimpleAudioVolume(this, core::mem::transmute_copy(&audiosessionguid), core::mem::transmute_copy(&streamflags)) {
                    Ok(ok__) => {
                        audiovolume.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAudioSessionControl: GetAudioSessionControl::<Identity, OFFSET>,
            GetSimpleAudioVolume: GetSimpleAudioVolume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "audioclient")]
impl windows_core::RuntimeName for IAudioSessionManager {}
windows_core::imp::define_interface!(IAudioSessionManager2, IAudioSessionManager2_Vtbl, 0x77aa99a0_1bd6_484f_8bc7_2c654c9a9b6f);
impl core::ops::Deref for IAudioSessionManager2 {
    type Target = IAudioSessionManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioSessionManager2, windows_core::IUnknown, IAudioSessionManager);
impl IAudioSessionManager2 {
    pub unsafe fn GetSessionEnumerator(&self) -> windows_core::Result<IAudioSessionEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterSessionNotification<P0>(&self, sessionnotification: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSessionNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterSessionNotification)(windows_core::Interface::as_raw(self), sessionnotification.param().abi()) }
    }
    pub unsafe fn UnregisterSessionNotification<P0>(&self, sessionnotification: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSessionNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterSessionNotification)(windows_core::Interface::as_raw(self), sessionnotification.param().abi()) }
    }
    pub unsafe fn RegisterDuckNotification<P0, P1>(&self, sessionid: P0, ducknotification: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IAudioVolumeDuckNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterDuckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi(), ducknotification.param().abi()) }
    }
    pub unsafe fn UnregisterDuckNotification<P0>(&self, ducknotification: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioVolumeDuckNotification>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterDuckNotification)(windows_core::Interface::as_raw(self), ducknotification.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionManager2_Vtbl {
    pub base__: IAudioSessionManager_Vtbl,
    pub GetSessionEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterSessionNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "audioclient")]
pub trait IAudioSessionManager2_Impl: IAudioSessionManager_Impl {
    fn GetSessionEnumerator(&self) -> windows_core::Result<IAudioSessionEnumerator>;
    fn RegisterSessionNotification(&self, sessionnotification: windows_core::Ref<IAudioSessionNotification>) -> windows_core::Result<()>;
    fn UnregisterSessionNotification(&self, sessionnotification: windows_core::Ref<IAudioSessionNotification>) -> windows_core::Result<()>;
    fn RegisterDuckNotification(&self, sessionid: &windows_core::PCWSTR, ducknotification: windows_core::Ref<IAudioVolumeDuckNotification>) -> windows_core::Result<()>;
    fn UnregisterDuckNotification(&self, ducknotification: windows_core::Ref<IAudioVolumeDuckNotification>) -> windows_core::Result<()>;
}
#[cfg(feature = "audioclient")]
impl IAudioSessionManager2_Vtbl {
    pub const fn new<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSessionEnumerator<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSessionManager2_Impl::GetSessionEnumerator(this) {
                    Ok(ok__) => {
                        sessionenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterSessionNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionnotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::RegisterSessionNotification(this, core::mem::transmute_copy(&sessionnotification)).into()
            }
        }
        unsafe extern "system" fn UnregisterSessionNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionnotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::UnregisterSessionNotification(this, core::mem::transmute_copy(&sessionnotification)).into()
            }
        }
        unsafe extern "system" fn RegisterDuckNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, ducknotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::RegisterDuckNotification(this, core::mem::transmute(&sessionid), core::mem::transmute_copy(&ducknotification)).into()
            }
        }
        unsafe extern "system" fn UnregisterDuckNotification<Identity: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ducknotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionManager2_Impl::UnregisterDuckNotification(this, core::mem::transmute_copy(&ducknotification)).into()
            }
        }
        Self {
            base__: IAudioSessionManager_Vtbl::new::<Identity, OFFSET>(),
            GetSessionEnumerator: GetSessionEnumerator::<Identity, OFFSET>,
            RegisterSessionNotification: RegisterSessionNotification::<Identity, OFFSET>,
            UnregisterSessionNotification: UnregisterSessionNotification::<Identity, OFFSET>,
            RegisterDuckNotification: RegisterDuckNotification::<Identity, OFFSET>,
            UnregisterDuckNotification: UnregisterDuckNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionManager2 as windows_core::Interface>::IID || iid == &<IAudioSessionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "audioclient")]
impl windows_core::RuntimeName for IAudioSessionManager2 {}
windows_core::imp::define_interface!(IAudioSessionNotification, IAudioSessionNotification_Vtbl, 0x641dd20b_4d41_49cc_aba3_174b9477bb08);
windows_core::imp::interface_hierarchy!(IAudioSessionNotification, windows_core::IUnknown);
impl IAudioSessionNotification {
    pub unsafe fn OnSessionCreated<P0>(&self, newsession: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSessionControl>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSessionCreated)(windows_core::Interface::as_raw(self), newsession.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSessionNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSessionCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioSessionNotification_Impl: windows_core::IUnknownImpl {
    fn OnSessionCreated(&self, newsession: windows_core::Ref<IAudioSessionControl>) -> windows_core::Result<()>;
}
impl IAudioSessionNotification_Vtbl {
    pub const fn new<Identity: IAudioSessionNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSessionCreated<Identity: IAudioSessionNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newsession: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSessionNotification_Impl::OnSessionCreated(this, core::mem::transmute_copy(&newsession)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSessionCreated: OnSessionCreated::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSessionNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioSessionNotification {}
windows_core::imp::define_interface!(IAudioVolumeDuckNotification, IAudioVolumeDuckNotification_Vtbl, 0xc3b284d4_6d39_4359_b3cf_b56ddb3bb39c);
windows_core::imp::interface_hierarchy!(IAudioVolumeDuckNotification, windows_core::IUnknown);
impl IAudioVolumeDuckNotification {
    pub unsafe fn OnVolumeDuckNotification<P0>(&self, sessionid: P0, countcommunicationsessions: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnVolumeDuckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi(), countcommunicationsessions) }
    }
    pub unsafe fn OnVolumeUnduckNotification<P0>(&self, sessionid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnVolumeUnduckNotification)(windows_core::Interface::as_raw(self), sessionid.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioVolumeDuckNotification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnVolumeDuckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnVolumeUnduckNotification: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAudioVolumeDuckNotification_Impl: windows_core::IUnknownImpl {
    fn OnVolumeDuckNotification(&self, sessionid: &windows_core::PCWSTR, countcommunicationsessions: u32) -> windows_core::Result<()>;
    fn OnVolumeUnduckNotification(&self, sessionid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAudioVolumeDuckNotification_Vtbl {
    pub const fn new<Identity: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnVolumeDuckNotification<Identity: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, countcommunicationsessions: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioVolumeDuckNotification_Impl::OnVolumeDuckNotification(this, core::mem::transmute(&sessionid), core::mem::transmute_copy(&countcommunicationsessions)).into()
            }
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Identity: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioVolumeDuckNotification_Impl::OnVolumeUnduckNotification(this, core::mem::transmute(&sessionid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnVolumeDuckNotification: OnVolumeDuckNotification::<Identity, OFFSET>,
            OnVolumeUnduckNotification: OnVolumeUnduckNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioVolumeDuckNotification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioVolumeDuckNotification {}
