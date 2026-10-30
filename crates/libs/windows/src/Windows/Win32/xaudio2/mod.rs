#[cfg(feature = "shared")]
#[inline]
pub unsafe fn XAudio2CreateWithVersionInfo(ppxaudio2: *mut Option<IXAudio2>, flags: super::UINT32, xaudio2processor: XAUDIO2_PROCESSOR, ntddiversion: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("xaudio2_9.dll" "system" fn XAudio2CreateWithVersionInfo(ppxaudio2 : *mut *mut core::ffi::c_void, flags : super::UINT32, xaudio2processor : XAUDIO2_PROCESSOR, ntddiversion : super::DWORD) -> windows_core::HRESULT);
    unsafe { XAudio2CreateWithVersionInfo(core::mem::transmute(ppxaudio2), flags, xaudio2processor, ntddiversion) }
}
pub const BandPassFilter: XAUDIO2_FILTER_TYPE = 1;
pub const FACILITY_XAUDIO2: i32 = 2198;
pub const HighPassFilter: XAUDIO2_FILTER_TYPE = 2;
pub const HighPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 5;
pub const IID_IXAudio2: windows_core::GUID = windows_core::GUID::from_u128(0x2b02e3cf_2e0b_4ec3_be45_1b2a3fe7210d);
pub const IID_IXAudio2Extension: windows_core::GUID = windows_core::GUID::from_u128(0x84ac29bb_d619_44d2_b197_e4acf7df3ed6);
windows_core::imp::define_interface!(IXAudio2, IXAudio2_Vtbl, 0x2b02e3cf_2e0b_4ec3_be45_1b2a3fe7210d);
windows_core::imp::interface_hierarchy!(IXAudio2, windows_core::IUnknown);
impl IXAudio2 {
    pub unsafe fn RegisterForCallbacks<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IXAudio2EngineCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterForCallbacks)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    pub unsafe fn UnregisterForCallbacks<P0>(&self, pcallback: P0)
    where
        P0: windows_core::Param<IXAudio2EngineCallback>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterForCallbacks)(windows_core::Interface::as_raw(self), pcallback.param().abi());
        }
    }
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub unsafe fn CreateSourceVoice<P4>(&self, ppsourcevoice: *mut Option<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: super::UINT32, maxfrequencyratio: f32, pcallback: P4, psendlist: Option<*const XAUDIO2_VOICE_SENDS>, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IXAudio2VoiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateSourceVoice)(windows_core::Interface::as_raw(self), core::mem::transmute(ppsourcevoice), psourceformat, flags, maxfrequencyratio, pcallback.param().abi(), psendlist.unwrap_or(core::mem::zeroed()) as _, peffectchain.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CreateSubmixVoice(&self, ppsubmixvoice: *mut Option<IXAudio2SubmixVoice>, inputchannels: super::UINT32, inputsamplerate: super::UINT32, flags: super::UINT32, processingstage: super::UINT32, psendlist: Option<*const XAUDIO2_VOICE_SENDS>, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateSubmixVoice)(windows_core::Interface::as_raw(self), core::mem::transmute(ppsubmixvoice), inputchannels, inputsamplerate, flags, processingstage, psendlist.unwrap_or(core::mem::zeroed()) as _, peffectchain.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "audiosessiontypes", feature = "shared"))]
    pub unsafe fn CreateMasteringVoice<P4>(&self, ppmasteringvoice: *mut Option<IXAudio2MasteringVoice>, inputchannels: super::UINT32, inputsamplerate: super::UINT32, flags: super::UINT32, szdeviceid: P4, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::HRESULT
    where
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMasteringVoice)(windows_core::Interface::as_raw(self), core::mem::transmute(ppmasteringvoice), inputchannels, inputsamplerate, flags, szdeviceid.param().abi(), peffectchain.unwrap_or(core::mem::zeroed()) as _, streamcategory) }
    }
    pub unsafe fn StartEngine(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartEngine)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn StopEngine(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).StopEngine)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CommitChanges(&self, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitChanges)(windows_core::Interface::as_raw(self), operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
        unsafe {
            (windows_core::Interface::vtable(self).GetPerformanceData)(windows_core::Interface::as_raw(self), pperfdata as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetDebugConfiguration(&self, pdebugconfiguration: Option<*const XAUDIO2_DEBUG_CONFIGURATION>, preserved: Option<*mut core::ffi::c_void>) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDebugConfiguration)(windows_core::Interface::as_raw(self), pdebugconfiguration.unwrap_or(core::mem::zeroed()) as _, preserved.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterForCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterForCallbacks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    #[cfg(all(feature = "mmeapi", feature = "shared"))]
    pub CreateSourceVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *const super::WAVEFORMATEX, super::UINT32, f32, *mut core::ffi::c_void, *const XAUDIO2_VOICE_SENDS, *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mmeapi", feature = "shared")))]
    CreateSourceVoice: usize,
    #[cfg(feature = "shared")]
    pub CreateSubmixVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::UINT32, super::UINT32, super::UINT32, super::UINT32, *const XAUDIO2_VOICE_SENDS, *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CreateSubmixVoice: usize,
    #[cfg(all(feature = "audiosessiontypes", feature = "shared"))]
    pub CreateMasteringVoice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, super::UINT32, super::UINT32, super::UINT32, windows_core::PCWSTR, *const XAUDIO2_EFFECT_CHAIN, super::AUDIO_STREAM_CATEGORY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "audiosessiontypes", feature = "shared")))]
    CreateMasteringVoice: usize,
    pub StartEngine: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StopEngine: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "shared")]
    pub CommitChanges: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CommitChanges: usize,
    #[cfg(feature = "shared")]
    pub GetPerformanceData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_PERFORMANCE_DATA),
    #[cfg(not(feature = "shared"))]
    GetPerformanceData: usize,
    #[cfg(feature = "shared")]
    pub SetDebugConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_DEBUG_CONFIGURATION, *mut core::ffi::c_void),
    #[cfg(not(feature = "shared"))]
    SetDebugConfiguration: usize,
}
#[cfg(all(feature = "audiosessiontypes", feature = "mmeapi", feature = "shared"))]
pub trait IXAudio2_Impl: windows_core::IUnknownImpl {
    fn RegisterForCallbacks(&self, pcallback: windows_core::Ref<IXAudio2EngineCallback>) -> windows_core::Result<()>;
    fn UnregisterForCallbacks(&self, pcallback: windows_core::Ref<IXAudio2EngineCallback>);
    fn CreateSourceVoice(&self, ppsourcevoice: windows_core::OutRef<IXAudio2SourceVoice>, psourceformat: *const super::WAVEFORMATEX, flags: super::UINT32, maxfrequencyratio: f32, pcallback: windows_core::Ref<IXAudio2VoiceCallback>, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn CreateSubmixVoice(&self, ppsubmixvoice: windows_core::OutRef<IXAudio2SubmixVoice>, inputchannels: super::UINT32, inputsamplerate: super::UINT32, flags: super::UINT32, processingstage: super::UINT32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn CreateMasteringVoice(&self, ppmasteringvoice: windows_core::OutRef<IXAudio2MasteringVoice>, inputchannels: super::UINT32, inputsamplerate: super::UINT32, flags: super::UINT32, szdeviceid: &windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::Result<()>;
    fn StartEngine(&self) -> windows_core::Result<()>;
    fn StopEngine(&self);
    fn CommitChanges(&self, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetPerformanceData(&self, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA);
    fn SetDebugConfiguration(&self, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut core::ffi::c_void);
}
#[cfg(all(feature = "audiosessiontypes", feature = "mmeapi", feature = "shared"))]
impl IXAudio2_Vtbl {
    pub const fn new<Identity: IXAudio2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterForCallbacks<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::RegisterForCallbacks(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn UnregisterForCallbacks<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::UnregisterForCallbacks(this, core::mem::transmute_copy(&pcallback));
            }
        }
        unsafe extern "system" fn CreateSourceVoice<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsourcevoice: *mut *mut core::ffi::c_void, psourceformat: *const super::WAVEFORMATEX, flags: super::UINT32, maxfrequencyratio: f32, pcallback: *mut core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CreateSourceVoice(this, core::mem::transmute_copy(&ppsourcevoice), core::mem::transmute_copy(&psourceformat), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&maxfrequencyratio), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&psendlist), core::mem::transmute_copy(&peffectchain)).into()
            }
        }
        unsafe extern "system" fn CreateSubmixVoice<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsubmixvoice: *mut *mut core::ffi::c_void, inputchannels: super::UINT32, inputsamplerate: super::UINT32, flags: super::UINT32, processingstage: super::UINT32, psendlist: *const XAUDIO2_VOICE_SENDS, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CreateSubmixVoice(this, core::mem::transmute_copy(&ppsubmixvoice), core::mem::transmute_copy(&inputchannels), core::mem::transmute_copy(&inputsamplerate), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&processingstage), core::mem::transmute_copy(&psendlist), core::mem::transmute_copy(&peffectchain)).into()
            }
        }
        unsafe extern "system" fn CreateMasteringVoice<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmasteringvoice: *mut *mut core::ffi::c_void, inputchannels: super::UINT32, inputsamplerate: super::UINT32, flags: super::UINT32, szdeviceid: windows_core::PCWSTR, peffectchain: *const XAUDIO2_EFFECT_CHAIN, streamcategory: super::AUDIO_STREAM_CATEGORY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CreateMasteringVoice(this, core::mem::transmute_copy(&ppmasteringvoice), core::mem::transmute_copy(&inputchannels), core::mem::transmute_copy(&inputsamplerate), core::mem::transmute_copy(&flags), core::mem::transmute(&szdeviceid), core::mem::transmute_copy(&peffectchain), core::mem::transmute_copy(&streamcategory)).into()
            }
        }
        unsafe extern "system" fn StartEngine<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::StartEngine(this).into()
            }
        }
        unsafe extern "system" fn StopEngine<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::StopEngine(this);
            }
        }
        unsafe extern "system" fn CommitChanges<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::CommitChanges(this, core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetPerformanceData<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperfdata: *mut XAUDIO2_PERFORMANCE_DATA) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::GetPerformanceData(this, core::mem::transmute_copy(&pperfdata));
            }
        }
        unsafe extern "system" fn SetDebugConfiguration<Identity: IXAudio2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdebugconfiguration: *const XAUDIO2_DEBUG_CONFIGURATION, preserved: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2_Impl::SetDebugConfiguration(this, core::mem::transmute_copy(&pdebugconfiguration), core::mem::transmute_copy(&preserved));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterForCallbacks: RegisterForCallbacks::<Identity, OFFSET>,
            UnregisterForCallbacks: UnregisterForCallbacks::<Identity, OFFSET>,
            CreateSourceVoice: CreateSourceVoice::<Identity, OFFSET>,
            CreateSubmixVoice: CreateSubmixVoice::<Identity, OFFSET>,
            CreateMasteringVoice: CreateMasteringVoice::<Identity, OFFSET>,
            StartEngine: StartEngine::<Identity, OFFSET>,
            StopEngine: StopEngine::<Identity, OFFSET>,
            CommitChanges: CommitChanges::<Identity, OFFSET>,
            GetPerformanceData: GetPerformanceData::<Identity, OFFSET>,
            SetDebugConfiguration: SetDebugConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAudio2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "audiosessiontypes", feature = "mmeapi", feature = "shared"))]
impl windows_core::RuntimeName for IXAudio2 {}
windows_core::imp::define_interface!(IXAudio2EngineCallback, IXAudio2EngineCallback_Vtbl);
impl IXAudio2EngineCallback {
    pub unsafe fn OnProcessingPassStart(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnProcessingPassStart)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnProcessingPassEnd(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnProcessingPassEnd)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnCriticalError(&self, error: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).OnCriticalError)(windows_core::Interface::as_raw(self), error);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2EngineCallback_Vtbl {
    pub OnProcessingPassStart: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnProcessingPassEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnCriticalError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
}
pub trait IXAudio2EngineCallback_Impl {
    fn OnProcessingPassStart(&self);
    fn OnProcessingPassEnd(&self);
    fn OnCriticalError(&self, error: windows_core::HRESULT);
}
impl IXAudio2EngineCallback_Vtbl {
    pub const fn new<Identity: IXAudio2EngineCallback_Impl>() -> Self {
        unsafe extern "system" fn OnProcessingPassStart<Identity: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2EngineCallback_Impl::OnProcessingPassStart(this);
            }
        }
        unsafe extern "system" fn OnProcessingPassEnd<Identity: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2EngineCallback_Impl::OnProcessingPassEnd(this);
            }
        }
        unsafe extern "system" fn OnCriticalError<Identity: IXAudio2EngineCallback_Impl>(this: *mut core::ffi::c_void, error: windows_core::HRESULT) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2EngineCallback_Impl::OnCriticalError(this, core::mem::transmute_copy(&error));
            }
        }
        Self {
            OnProcessingPassStart: OnProcessingPassStart::<Identity>,
            OnProcessingPassEnd: OnProcessingPassEnd::<Identity>,
            OnCriticalError: OnCriticalError::<Identity>,
        }
    }
}
struct IXAudio2EngineCallback_ImplVtbl<T: IXAudio2EngineCallback_Impl>(core::marker::PhantomData<T>);
impl<T: IXAudio2EngineCallback_Impl> IXAudio2EngineCallback_ImplVtbl<T> {
    const VTABLE: IXAudio2EngineCallback_Vtbl = IXAudio2EngineCallback_Vtbl::new::<T>();
}
impl IXAudio2EngineCallback {
    pub fn new<'a, T: IXAudio2EngineCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2EngineCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
windows_core::imp::define_interface!(IXAudio2Extension, IXAudio2Extension_Vtbl, 0x84ac29bb_d619_44d2_b197_e4acf7df3ed6);
windows_core::imp::interface_hierarchy!(IXAudio2Extension, windows_core::IUnknown);
impl IXAudio2Extension {
    #[cfg(feature = "shared")]
    pub unsafe fn GetProcessingQuantum(&self, quantumnumerator: *mut super::UINT32, quantumdenominator: *mut super::UINT32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetProcessingQuantum)(windows_core::Interface::as_raw(self), quantumnumerator as _, quantumdenominator as _);
        }
    }
    pub unsafe fn GetProcessor(&self, processor: *mut XAUDIO2_PROCESSOR) {
        unsafe {
            (windows_core::Interface::vtable(self).GetProcessor)(windows_core::Interface::as_raw(self), processor as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Extension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetProcessingQuantum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32, *mut super::UINT32),
    #[cfg(not(feature = "shared"))]
    GetProcessingQuantum: usize,
    pub GetProcessor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_PROCESSOR),
}
#[cfg(feature = "shared")]
pub trait IXAudio2Extension_Impl: windows_core::IUnknownImpl {
    fn GetProcessingQuantum(&self, quantumnumerator: *mut super::UINT32, quantumdenominator: *mut super::UINT32);
    fn GetProcessor(&self, processor: *mut XAUDIO2_PROCESSOR);
}
#[cfg(feature = "shared")]
impl IXAudio2Extension_Vtbl {
    pub const fn new<Identity: IXAudio2Extension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProcessingQuantum<Identity: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, quantumnumerator: *mut super::UINT32, quantumdenominator: *mut super::UINT32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2Extension_Impl::GetProcessingQuantum(this, core::mem::transmute_copy(&quantumnumerator), core::mem::transmute_copy(&quantumdenominator));
            }
        }
        unsafe extern "system" fn GetProcessor<Identity: IXAudio2Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processor: *mut XAUDIO2_PROCESSOR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXAudio2Extension_Impl::GetProcessor(this, core::mem::transmute_copy(&processor));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProcessingQuantum: GetProcessingQuantum::<Identity, OFFSET>,
            GetProcessor: GetProcessor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXAudio2Extension as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IXAudio2Extension {}
windows_core::imp::define_interface!(IXAudio2MasteringVoice, IXAudio2MasteringVoice_Vtbl);
impl core::ops::Deref for IXAudio2MasteringVoice {
    type Target = IXAudio2Voice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAudio2MasteringVoice, IXAudio2Voice);
impl IXAudio2MasteringVoice {
    #[cfg(feature = "shared")]
    pub unsafe fn GetChannelMask(&self, pchannelmask: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChannelMask)(windows_core::Interface::as_raw(self), pchannelmask as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2MasteringVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
    #[cfg(feature = "shared")]
    pub GetChannelMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetChannelMask: usize,
}
#[cfg(feature = "shared")]
pub trait IXAudio2MasteringVoice_Impl: IXAudio2Voice_Impl {
    fn GetChannelMask(&self, pchannelmask: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IXAudio2MasteringVoice_Vtbl {
    pub const fn new<Identity: IXAudio2MasteringVoice_Impl>() -> Self {
        unsafe extern "system" fn GetChannelMask<Identity: IXAudio2MasteringVoice_Impl>(this: *mut core::ffi::c_void, pchannelmask: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2MasteringVoice_Impl::GetChannelMask(this, core::mem::transmute_copy(&pchannelmask)).into()
            }
        }
        Self { base__: IXAudio2Voice_Vtbl::new::<Identity>(), GetChannelMask: GetChannelMask::<Identity> }
    }
}
#[cfg(feature = "shared")]
struct IXAudio2MasteringVoice_ImplVtbl<T: IXAudio2MasteringVoice_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "shared")]
impl<T: IXAudio2MasteringVoice_Impl> IXAudio2MasteringVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2MasteringVoice_Vtbl = IXAudio2MasteringVoice_Vtbl::new::<T>();
}
#[cfg(feature = "shared")]
impl IXAudio2MasteringVoice {
    pub fn new<'a, T: IXAudio2MasteringVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2MasteringVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
windows_core::imp::define_interface!(IXAudio2SourceVoice, IXAudio2SourceVoice_Vtbl);
impl core::ops::Deref for IXAudio2SourceVoice {
    type Target = IXAudio2Voice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAudio2SourceVoice, IXAudio2Voice);
impl IXAudio2SourceVoice {
    #[cfg(feature = "shared")]
    pub unsafe fn Start(&self, flags: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), flags, operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Stop(&self, flags: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self), flags, operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SubmitSourceBuffer(&self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: Option<*const XAUDIO2_BUFFER_WMA>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SubmitSourceBuffer)(windows_core::Interface::as_raw(self), pbuffer, pbufferwma.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn FlushSourceBuffers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FlushSourceBuffers)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Discontinuity(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Discontinuity)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ExitLoop(&self, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExitLoop)(windows_core::Interface::as_raw(self), operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: super::UINT32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), pvoicestate as _, flags);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFrequencyRatio(&self, ratio: f32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFrequencyRatio)(windows_core::Interface::as_raw(self), ratio, operationset) }
    }
    pub unsafe fn GetFrequencyRatio(&self, pratio: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetFrequencyRatio)(windows_core::Interface::as_raw(self), pratio as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetSourceSampleRate(&self, newsourcesamplerate: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceSampleRate)(windows_core::Interface::as_raw(self), newsourcesamplerate) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SourceVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
    #[cfg(feature = "shared")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Start: usize,
    #[cfg(feature = "shared")]
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Stop: usize,
    #[cfg(feature = "shared")]
    pub SubmitSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_BUFFER, *const XAUDIO2_BUFFER_WMA) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SubmitSourceBuffer: usize,
    pub FlushSourceBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Discontinuity: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub ExitLoop: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ExitLoop: usize,
    #[cfg(feature = "shared")]
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_VOICE_STATE, super::UINT32),
    #[cfg(not(feature = "shared"))]
    GetState: usize,
    #[cfg(feature = "shared")]
    pub SetFrequencyRatio: unsafe extern "system" fn(*mut core::ffi::c_void, f32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFrequencyRatio: usize,
    pub GetFrequencyRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32),
    #[cfg(feature = "shared")]
    pub SetSourceSampleRate: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetSourceSampleRate: usize,
}
#[cfg(feature = "shared")]
pub trait IXAudio2SourceVoice_Impl: IXAudio2Voice_Impl {
    fn Start(&self, flags: super::UINT32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn Stop(&self, flags: super::UINT32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn SubmitSourceBuffer(&self, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> windows_core::Result<()>;
    fn FlushSourceBuffers(&self) -> windows_core::Result<()>;
    fn Discontinuity(&self) -> windows_core::Result<()>;
    fn ExitLoop(&self, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetState(&self, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: super::UINT32);
    fn SetFrequencyRatio(&self, ratio: f32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetFrequencyRatio(&self, pratio: *mut f32);
    fn SetSourceSampleRate(&self, newsourcesamplerate: super::UINT32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IXAudio2SourceVoice_Vtbl {
    pub const fn new<Identity: IXAudio2SourceVoice_Impl>() -> Self {
        unsafe extern "system" fn Start<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, flags: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::Start(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, flags: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::Stop(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn SubmitSourceBuffer<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pbuffer: *const XAUDIO2_BUFFER, pbufferwma: *const XAUDIO2_BUFFER_WMA) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::SubmitSourceBuffer(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&pbufferwma)).into()
            }
        }
        unsafe extern "system" fn FlushSourceBuffers<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::FlushSourceBuffers(this).into()
            }
        }
        unsafe extern "system" fn Discontinuity<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::Discontinuity(this).into()
            }
        }
        unsafe extern "system" fn ExitLoop<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::ExitLoop(this, core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pvoicestate: *mut XAUDIO2_VOICE_STATE, flags: super::UINT32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::GetState(this, core::mem::transmute_copy(&pvoicestate), core::mem::transmute_copy(&flags));
            }
        }
        unsafe extern "system" fn SetFrequencyRatio<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, ratio: f32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::SetFrequencyRatio(this, core::mem::transmute_copy(&ratio), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetFrequencyRatio<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, pratio: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::GetFrequencyRatio(this, core::mem::transmute_copy(&pratio));
            }
        }
        unsafe extern "system" fn SetSourceSampleRate<Identity: IXAudio2SourceVoice_Impl>(this: *mut core::ffi::c_void, newsourcesamplerate: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2SourceVoice_Impl::SetSourceSampleRate(this, core::mem::transmute_copy(&newsourcesamplerate)).into()
            }
        }
        Self {
            base__: IXAudio2Voice_Vtbl::new::<Identity>(),
            Start: Start::<Identity>,
            Stop: Stop::<Identity>,
            SubmitSourceBuffer: SubmitSourceBuffer::<Identity>,
            FlushSourceBuffers: FlushSourceBuffers::<Identity>,
            Discontinuity: Discontinuity::<Identity>,
            ExitLoop: ExitLoop::<Identity>,
            GetState: GetState::<Identity>,
            SetFrequencyRatio: SetFrequencyRatio::<Identity>,
            GetFrequencyRatio: GetFrequencyRatio::<Identity>,
            SetSourceSampleRate: SetSourceSampleRate::<Identity>,
        }
    }
}
#[cfg(feature = "shared")]
struct IXAudio2SourceVoice_ImplVtbl<T: IXAudio2SourceVoice_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "shared")]
impl<T: IXAudio2SourceVoice_Impl> IXAudio2SourceVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2SourceVoice_Vtbl = IXAudio2SourceVoice_Vtbl::new::<T>();
}
#[cfg(feature = "shared")]
impl IXAudio2SourceVoice {
    pub fn new<'a, T: IXAudio2SourceVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2SourceVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
windows_core::imp::define_interface!(IXAudio2SubmixVoice, IXAudio2SubmixVoice_Vtbl);
impl core::ops::Deref for IXAudio2SubmixVoice {
    type Target = IXAudio2Voice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXAudio2SubmixVoice, IXAudio2Voice);
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2SubmixVoice_Vtbl {
    pub base__: IXAudio2Voice_Vtbl,
}
#[cfg(feature = "shared")]
pub trait IXAudio2SubmixVoice_Impl: IXAudio2Voice_Impl {}
#[cfg(feature = "shared")]
impl IXAudio2SubmixVoice_Vtbl {
    pub const fn new<Identity: IXAudio2SubmixVoice_Impl>() -> Self {
        Self { base__: IXAudio2Voice_Vtbl::new::<Identity>() }
    }
}
#[cfg(feature = "shared")]
struct IXAudio2SubmixVoice_ImplVtbl<T: IXAudio2SubmixVoice_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "shared")]
impl<T: IXAudio2SubmixVoice_Impl> IXAudio2SubmixVoice_ImplVtbl<T> {
    const VTABLE: IXAudio2SubmixVoice_Vtbl = IXAudio2SubmixVoice_Vtbl::new::<T>();
}
#[cfg(feature = "shared")]
impl IXAudio2SubmixVoice {
    pub fn new<'a, T: IXAudio2SubmixVoice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2SubmixVoice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
windows_core::imp::define_interface!(IXAudio2Voice, IXAudio2Voice_Vtbl);
impl IXAudio2Voice {
    #[cfg(feature = "shared")]
    pub unsafe fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
        unsafe {
            (windows_core::Interface::vtable(self).GetVoiceDetails)(windows_core::Interface::as_raw(self), pvoicedetails as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputVoices(&self, psendlist: Option<*const XAUDIO2_VOICE_SENDS>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOutputVoices)(windows_core::Interface::as_raw(self), psendlist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetEffectChain(&self, peffectchain: Option<*const XAUDIO2_EFFECT_CHAIN>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffectChain)(windows_core::Interface::as_raw(self), peffectchain.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn EnableEffect(&self, effectindex: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableEffect)(windows_core::Interface::as_raw(self), effectindex, operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DisableEffect(&self, effectindex: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableEffect)(windows_core::Interface::as_raw(self), effectindex, operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEffectState(&self, effectindex: super::UINT32, penabled: *mut windows_core::BOOL) {
        unsafe {
            (windows_core::Interface::vtable(self).GetEffectState)(windows_core::Interface::as_raw(self), effectindex, penabled as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetEffectParameters(&self, effectindex: super::UINT32, pparameters: *const core::ffi::c_void, parametersbytesize: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEffectParameters)(windows_core::Interface::as_raw(self), effectindex, pparameters, parametersbytesize, operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEffectParameters(&self, effectindex: super::UINT32, pparameters: *mut core::ffi::c_void, parametersbytesize: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEffectParameters)(windows_core::Interface::as_raw(self), effectindex, pparameters as _, parametersbytesize) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFilterParameters)(windows_core::Interface::as_raw(self), pparameters, operationset) }
    }
    pub unsafe fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
        unsafe {
            (windows_core::Interface::vtable(self).GetFilterParameters)(windows_core::Interface::as_raw(self), pparameters as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: super::UINT32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFilterParameters)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), pparameters, operationset) }
    }
    pub unsafe fn GetOutputFilterParameters<P0>(&self, pdestinationvoice: P0, pparameters: *mut XAUDIO2_FILTER_PARAMETERS)
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetOutputFilterParameters)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), pparameters as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetVolume(&self, volume: f32, operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume, operationset) }
    }
    pub unsafe fn GetVolume(&self, pvolume: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), pvolume as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetChannelVolumes(&self, pvolumes: &[f32], operationset: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolumes)(windows_core::Interface::as_raw(self), pvolumes.len().try_into().unwrap(), pvolumes.as_ptr(), operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetChannelVolumes(&self, channels: super::UINT32, pvolumes: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetChannelVolumes)(windows_core::Interface::as_raw(self), channels, pvolumes as _);
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: super::UINT32, destinationchannels: super::UINT32, plevelmatrix: *const f32, operationset: super::UINT32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputMatrix)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), sourcechannels, destinationchannels, plevelmatrix, operationset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetOutputMatrix<P0>(&self, pdestinationvoice: P0, sourcechannels: super::UINT32, destinationchannels: super::UINT32, plevelmatrix: *mut f32)
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetOutputMatrix)(windows_core::Interface::as_raw(self), pdestinationvoice.param().abi(), sourcechannels, destinationchannels, plevelmatrix as _);
        }
    }
    pub unsafe fn DestroyVoice(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DestroyVoice)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2Voice_Vtbl {
    #[cfg(feature = "shared")]
    pub GetVoiceDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_VOICE_DETAILS),
    #[cfg(not(feature = "shared"))]
    GetVoiceDetails: usize,
    #[cfg(feature = "shared")]
    pub SetOutputVoices: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_VOICE_SENDS) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputVoices: usize,
    #[cfg(feature = "shared")]
    pub SetEffectChain: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetEffectChain: usize,
    #[cfg(feature = "shared")]
    pub EnableEffect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    EnableEffect: usize,
    #[cfg(feature = "shared")]
    pub DisableEffect: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DisableEffect: usize,
    #[cfg(feature = "shared")]
    pub GetEffectState: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut windows_core::BOOL),
    #[cfg(not(feature = "shared"))]
    GetEffectState: usize,
    #[cfg(feature = "shared")]
    pub SetEffectParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *const core::ffi::c_void, super::UINT32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetEffectParameters: usize,
    #[cfg(feature = "shared")]
    pub GetEffectParameters: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEffectParameters: usize,
    #[cfg(feature = "shared")]
    pub SetFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *const XAUDIO2_FILTER_PARAMETERS, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetFilterParameters: usize,
    pub GetFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XAUDIO2_FILTER_PARAMETERS),
    #[cfg(feature = "shared")]
    pub SetOutputFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const XAUDIO2_FILTER_PARAMETERS, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputFilterParameters: usize,
    pub GetOutputFilterParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut XAUDIO2_FILTER_PARAMETERS),
    #[cfg(feature = "shared")]
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetVolume: usize,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32),
    #[cfg(feature = "shared")]
    pub SetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *const f32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetChannelVolumes: usize,
    #[cfg(feature = "shared")]
    pub GetChannelVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32, *mut f32),
    #[cfg(not(feature = "shared"))]
    GetChannelVolumes: usize,
    #[cfg(feature = "shared")]
    pub SetOutputMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT32, super::UINT32, *const f32, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetOutputMatrix: usize,
    #[cfg(feature = "shared")]
    pub GetOutputMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT32, super::UINT32, *mut f32),
    #[cfg(not(feature = "shared"))]
    GetOutputMatrix: usize,
    pub DestroyVoice: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "shared")]
pub trait IXAudio2Voice_Impl {
    fn GetVoiceDetails(&self, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS);
    fn SetOutputVoices(&self, psendlist: *const XAUDIO2_VOICE_SENDS) -> windows_core::Result<()>;
    fn SetEffectChain(&self, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::Result<()>;
    fn EnableEffect(&self, effectindex: super::UINT32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn DisableEffect(&self, effectindex: super::UINT32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetEffectState(&self, effectindex: super::UINT32, penabled: *mut windows_core::BOOL);
    fn SetEffectParameters(&self, effectindex: super::UINT32, pparameters: *const core::ffi::c_void, parametersbytesize: super::UINT32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetEffectParameters(&self, effectindex: super::UINT32, pparameters: *mut core::ffi::c_void, parametersbytesize: super::UINT32) -> windows_core::Result<()>;
    fn SetFilterParameters(&self, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetFilterParameters(&self, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetOutputFilterParameters(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetOutputFilterParameters(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, pparameters: *mut XAUDIO2_FILTER_PARAMETERS);
    fn SetVolume(&self, volume: f32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetVolume(&self, pvolume: *mut f32);
    fn SetChannelVolumes(&self, channels: super::UINT32, pvolumes: *const f32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetChannelVolumes(&self, channels: super::UINT32, pvolumes: *mut f32);
    fn SetOutputMatrix(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, sourcechannels: super::UINT32, destinationchannels: super::UINT32, plevelmatrix: *const f32, operationset: super::UINT32) -> windows_core::Result<()>;
    fn GetOutputMatrix(&self, pdestinationvoice: windows_core::Ref<IXAudio2Voice>, sourcechannels: super::UINT32, destinationchannels: super::UINT32, plevelmatrix: *mut f32);
    fn DestroyVoice(&self);
}
#[cfg(feature = "shared")]
impl IXAudio2Voice_Vtbl {
    pub const fn new<Identity: IXAudio2Voice_Impl>() -> Self {
        unsafe extern "system" fn GetVoiceDetails<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pvoicedetails: *mut XAUDIO2_VOICE_DETAILS) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetVoiceDetails(this, core::mem::transmute_copy(&pvoicedetails));
            }
        }
        unsafe extern "system" fn SetOutputVoices<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, psendlist: *const XAUDIO2_VOICE_SENDS) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetOutputVoices(this, core::mem::transmute_copy(&psendlist)).into()
            }
        }
        unsafe extern "system" fn SetEffectChain<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, peffectchain: *const XAUDIO2_EFFECT_CHAIN) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetEffectChain(this, core::mem::transmute_copy(&peffectchain)).into()
            }
        }
        unsafe extern "system" fn EnableEffect<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::EnableEffect(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn DisableEffect<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::DisableEffect(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetEffectState<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: super::UINT32, penabled: *mut windows_core::BOOL) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetEffectState(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&penabled));
            }
        }
        unsafe extern "system" fn SetEffectParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: super::UINT32, pparameters: *const core::ffi::c_void, parametersbytesize: super::UINT32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetEffectParameters(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parametersbytesize), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetEffectParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, effectindex: super::UINT32, pparameters: *mut core::ffi::c_void, parametersbytesize: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetEffectParameters(this, core::mem::transmute_copy(&effectindex), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&parametersbytesize)).into()
            }
        }
        unsafe extern "system" fn SetFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetFilterParameters(this, core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetFilterParameters(this, core::mem::transmute_copy(&pparameters));
            }
        }
        unsafe extern "system" fn SetOutputFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, pparameters: *const XAUDIO2_FILTER_PARAMETERS, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetOutputFilterParameters(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&pparameters), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetOutputFilterParameters<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, pparameters: *mut XAUDIO2_FILTER_PARAMETERS) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetOutputFilterParameters(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&pparameters));
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, volume: f32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetVolume(this, core::mem::transmute_copy(&volume), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pvolume: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetVolume(this, core::mem::transmute_copy(&pvolume));
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, channels: super::UINT32, pvolumes: *const f32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetChannelVolumes(this, core::mem::transmute_copy(&channels), core::mem::transmute_copy(&pvolumes), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, channels: super::UINT32, pvolumes: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetChannelVolumes(this, core::mem::transmute_copy(&channels), core::mem::transmute_copy(&pvolumes));
            }
        }
        unsafe extern "system" fn SetOutputMatrix<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, sourcechannels: super::UINT32, destinationchannels: super::UINT32, plevelmatrix: *const f32, operationset: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::SetOutputMatrix(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&sourcechannels), core::mem::transmute_copy(&destinationchannels), core::mem::transmute_copy(&plevelmatrix), core::mem::transmute_copy(&operationset)).into()
            }
        }
        unsafe extern "system" fn GetOutputMatrix<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void, pdestinationvoice: *mut core::ffi::c_void, sourcechannels: super::UINT32, destinationchannels: super::UINT32, plevelmatrix: *mut f32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::GetOutputMatrix(this, core::mem::transmute_copy(&pdestinationvoice), core::mem::transmute_copy(&sourcechannels), core::mem::transmute_copy(&destinationchannels), core::mem::transmute_copy(&plevelmatrix));
            }
        }
        unsafe extern "system" fn DestroyVoice<Identity: IXAudio2Voice_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2Voice_Impl::DestroyVoice(this);
            }
        }
        Self {
            GetVoiceDetails: GetVoiceDetails::<Identity>,
            SetOutputVoices: SetOutputVoices::<Identity>,
            SetEffectChain: SetEffectChain::<Identity>,
            EnableEffect: EnableEffect::<Identity>,
            DisableEffect: DisableEffect::<Identity>,
            GetEffectState: GetEffectState::<Identity>,
            SetEffectParameters: SetEffectParameters::<Identity>,
            GetEffectParameters: GetEffectParameters::<Identity>,
            SetFilterParameters: SetFilterParameters::<Identity>,
            GetFilterParameters: GetFilterParameters::<Identity>,
            SetOutputFilterParameters: SetOutputFilterParameters::<Identity>,
            GetOutputFilterParameters: GetOutputFilterParameters::<Identity>,
            SetVolume: SetVolume::<Identity>,
            GetVolume: GetVolume::<Identity>,
            SetChannelVolumes: SetChannelVolumes::<Identity>,
            GetChannelVolumes: GetChannelVolumes::<Identity>,
            SetOutputMatrix: SetOutputMatrix::<Identity>,
            GetOutputMatrix: GetOutputMatrix::<Identity>,
            DestroyVoice: DestroyVoice::<Identity>,
        }
    }
}
#[cfg(feature = "shared")]
struct IXAudio2Voice_ImplVtbl<T: IXAudio2Voice_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "shared")]
impl<T: IXAudio2Voice_Impl> IXAudio2Voice_ImplVtbl<T> {
    const VTABLE: IXAudio2Voice_Vtbl = IXAudio2Voice_Vtbl::new::<T>();
}
#[cfg(feature = "shared")]
impl IXAudio2Voice {
    pub fn new<'a, T: IXAudio2Voice_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2Voice_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
windows_core::imp::define_interface!(IXAudio2VoiceCallback, IXAudio2VoiceCallback_Vtbl);
impl IXAudio2VoiceCallback {
    #[cfg(feature = "shared")]
    pub unsafe fn OnVoiceProcessingPassStart(&self, bytesrequired: super::UINT32) {
        unsafe {
            (windows_core::Interface::vtable(self).OnVoiceProcessingPassStart)(windows_core::Interface::as_raw(self), bytesrequired);
        }
    }
    pub unsafe fn OnVoiceProcessingPassEnd(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnVoiceProcessingPassEnd)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnStreamEnd(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnStreamEnd)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnBufferStart(&self, pbuffercontext: *mut core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).OnBufferStart)(windows_core::Interface::as_raw(self), pbuffercontext as _);
        }
    }
    pub unsafe fn OnBufferEnd(&self, pbuffercontext: *mut core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).OnBufferEnd)(windows_core::Interface::as_raw(self), pbuffercontext as _);
        }
    }
    pub unsafe fn OnLoopEnd(&self, pbuffercontext: *mut core::ffi::c_void) {
        unsafe {
            (windows_core::Interface::vtable(self).OnLoopEnd)(windows_core::Interface::as_raw(self), pbuffercontext as _);
        }
    }
    pub unsafe fn OnVoiceError(&self, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).OnVoiceError)(windows_core::Interface::as_raw(self), pbuffercontext as _, error);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXAudio2VoiceCallback_Vtbl {
    #[cfg(feature = "shared")]
    pub OnVoiceProcessingPassStart: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32),
    #[cfg(not(feature = "shared"))]
    OnVoiceProcessingPassStart: usize,
    pub OnVoiceProcessingPassEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnStreamEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnBufferStart: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnBufferEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnLoopEnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnVoiceError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::HRESULT),
}
#[cfg(feature = "shared")]
pub trait IXAudio2VoiceCallback_Impl {
    fn OnVoiceProcessingPassStart(&self, bytesrequired: super::UINT32);
    fn OnVoiceProcessingPassEnd(&self);
    fn OnStreamEnd(&self);
    fn OnBufferStart(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnBufferEnd(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnLoopEnd(&self, pbuffercontext: *mut core::ffi::c_void);
    fn OnVoiceError(&self, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT);
}
#[cfg(feature = "shared")]
impl IXAudio2VoiceCallback_Vtbl {
    pub const fn new<Identity: IXAudio2VoiceCallback_Impl>() -> Self {
        unsafe extern "system" fn OnVoiceProcessingPassStart<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, bytesrequired: super::UINT32) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnVoiceProcessingPassStart(this, core::mem::transmute_copy(&bytesrequired));
            }
        }
        unsafe extern "system" fn OnVoiceProcessingPassEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnVoiceProcessingPassEnd(this);
            }
        }
        unsafe extern "system" fn OnStreamEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnStreamEnd(this);
            }
        }
        unsafe extern "system" fn OnBufferStart<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnBufferStart(this, core::mem::transmute_copy(&pbuffercontext));
            }
        }
        unsafe extern "system" fn OnBufferEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnBufferEnd(this, core::mem::transmute_copy(&pbuffercontext));
            }
        }
        unsafe extern "system" fn OnLoopEnd<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnLoopEnd(this, core::mem::transmute_copy(&pbuffercontext));
            }
        }
        unsafe extern "system" fn OnVoiceError<Identity: IXAudio2VoiceCallback_Impl>(this: *mut core::ffi::c_void, pbuffercontext: *mut core::ffi::c_void, error: windows_core::HRESULT) {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IXAudio2VoiceCallback_Impl::OnVoiceError(this, core::mem::transmute_copy(&pbuffercontext), core::mem::transmute_copy(&error));
            }
        }
        Self {
            OnVoiceProcessingPassStart: OnVoiceProcessingPassStart::<Identity>,
            OnVoiceProcessingPassEnd: OnVoiceProcessingPassEnd::<Identity>,
            OnStreamEnd: OnStreamEnd::<Identity>,
            OnBufferStart: OnBufferStart::<Identity>,
            OnBufferEnd: OnBufferEnd::<Identity>,
            OnLoopEnd: OnLoopEnd::<Identity>,
            OnVoiceError: OnVoiceError::<Identity>,
        }
    }
}
#[cfg(feature = "shared")]
struct IXAudio2VoiceCallback_ImplVtbl<T: IXAudio2VoiceCallback_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "shared")]
impl<T: IXAudio2VoiceCallback_Impl> IXAudio2VoiceCallback_ImplVtbl<T> {
    const VTABLE: IXAudio2VoiceCallback_Vtbl = IXAudio2VoiceCallback_Vtbl::new::<T>();
}
#[cfg(feature = "shared")]
impl IXAudio2VoiceCallback {
    pub fn new<'a, T: IXAudio2VoiceCallback_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IXAudio2VoiceCallback_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
pub const LowPassFilter: XAUDIO2_FILTER_TYPE = 0;
pub const LowPassOnePoleFilter: XAUDIO2_FILTER_TYPE = 4;
pub const NotchFilter: XAUDIO2_FILTER_TYPE = 3;
pub const Processor1: i32 = 1;
pub const Processor10: i32 = 512;
pub const Processor11: i32 = 1024;
pub const Processor12: i32 = 2048;
pub const Processor13: i32 = 4096;
pub const Processor14: i32 = 8192;
pub const Processor15: i32 = 16384;
pub const Processor16: i32 = 32768;
pub const Processor17: i32 = 65536;
pub const Processor18: i32 = 131072;
pub const Processor19: i32 = 262144;
pub const Processor2: i32 = 2;
pub const Processor20: i32 = 524288;
pub const Processor21: i32 = 1048576;
pub const Processor22: i32 = 2097152;
pub const Processor23: i32 = 4194304;
pub const Processor24: i32 = 8388608;
pub const Processor25: i32 = 16777216;
pub const Processor26: i32 = 33554432;
pub const Processor27: i32 = 67108864;
pub const Processor28: i32 = 134217728;
pub const Processor29: i32 = 268435456;
pub const Processor3: i32 = 4;
pub const Processor30: i32 = 536870912;
pub const Processor31: i32 = 1073741824;
pub const Processor32: u32 = 2147483648;
pub const Processor4: i32 = 8;
pub const Processor5: i32 = 16;
pub const Processor6: i32 = 32;
pub const Processor7: i32 = 64;
pub const Processor8: i32 = 128;
pub const Processor9: i32 = 256;
pub const XAUDIO2D_DLL: windows_core::PCSTR = windows_core::s!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_A: windows_core::PCSTR = windows_core::s!("xaudio2_9d.dll");
pub const XAUDIO2D_DLL_W: windows_core::PCWSTR = windows_core::w!("xaudio2_9d.dll");
pub const XAUDIO2_1024_QUANTUM: i32 = 32768;
pub const XAUDIO2_ANY_PROCESSOR: u32 = 4294967295;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_BUFFER {
    pub Flags: super::UINT32,
    pub AudioBytes: super::UINT32,
    pub pAudioData: *const super::BYTE,
    pub PlayBegin: super::UINT32,
    pub PlayLength: super::UINT32,
    pub LoopBegin: super::UINT32,
    pub LoopLength: super::UINT32,
    pub LoopCount: super::UINT32,
    pub pContext: *mut core::ffi::c_void,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_BUFFER_WMA {
    pub pDecodedPacketCumulativeBytes: *const super::UINT32,
    pub PacketCount: super::UINT32,
}
pub const XAUDIO2_COMMIT_ALL: i32 = 0;
pub const XAUDIO2_COMMIT_NOW: i32 = 0;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_DEBUG_CONFIGURATION {
    pub TraceMask: super::UINT32,
    pub BreakMask: super::UINT32,
    pub LogThreadID: windows_core::BOOL,
    pub LogFileline: windows_core::BOOL,
    pub LogFunctionName: windows_core::BOOL,
    pub LogTiming: windows_core::BOOL,
}
pub const XAUDIO2_DEBUG_ENGINE: i32 = 1;
pub const XAUDIO2_DEFAULT_CHANNELS: i32 = 0;
pub const XAUDIO2_DEFAULT_FILTER_FREQUENCY: f32 = 1.0;
pub const XAUDIO2_DEFAULT_FILTER_ONEOVERQ: f32 = 1.0;
pub const XAUDIO2_DEFAULT_FILTER_TYPE: XAUDIO2_FILTER_TYPE = 0;
pub const XAUDIO2_DEFAULT_FREQ_RATIO: f32 = 2.0;
pub const XAUDIO2_DEFAULT_PROCESSOR: i32 = 1;
pub const XAUDIO2_DEFAULT_SAMPLERATE: i32 = 0;
pub const XAUDIO2_DLL: windows_core::PCSTR = windows_core::s!("xaudio2_9.dll");
pub const XAUDIO2_DLL_A: windows_core::PCSTR = windows_core::s!("xaudio2_9.dll");
pub const XAUDIO2_DLL_W: windows_core::PCWSTR = windows_core::w!("xaudio2_9.dll");
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_EFFECT_CHAIN {
    pub EffectCount: super::UINT32,
    pub pEffectDescriptors: *mut XAUDIO2_EFFECT_DESCRIPTOR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Default)]
pub struct XAUDIO2_EFFECT_DESCRIPTOR {
    pub pEffect: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub InitialState: windows_core::BOOL,
    pub OutputChannels: super::UINT32,
}
pub const XAUDIO2_END_OF_STREAM: i32 = 64;
pub const XAUDIO2_E_DEVICE_INVALIDATED: windows_core::HRESULT = windows_core::HRESULT(0x88960004_u32 as _);
pub const XAUDIO2_E_INVALID_CALL: windows_core::HRESULT = windows_core::HRESULT(0x88960001_u32 as _);
pub const XAUDIO2_E_XAPO_CREATION_FAILED: windows_core::HRESULT = windows_core::HRESULT(0x88960003_u32 as _);
pub const XAUDIO2_E_XMA_DECODER_ERROR: windows_core::HRESULT = windows_core::HRESULT(0x88960002_u32 as _);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_FILTER_PARAMETERS {
    pub Type: XAUDIO2_FILTER_TYPE,
    pub Frequency: f32,
    pub OneOverQ: f32,
}
pub type XAUDIO2_FILTER_TYPE = i32;
#[cfg(feature = "shared")]
pub const XAUDIO2_INVALID_OPSET: super::UINT32 = 4294967295;
pub const XAUDIO2_LOG_API_CALLS: i32 = 16;
pub const XAUDIO2_LOG_DETAIL: i32 = 8;
pub const XAUDIO2_LOG_ERRORS: i32 = 1;
pub const XAUDIO2_LOG_FUNC_CALLS: i32 = 32;
pub const XAUDIO2_LOG_INFO: i32 = 4;
pub const XAUDIO2_LOG_LOCKS: i32 = 128;
pub const XAUDIO2_LOG_MEMORY: i32 = 256;
pub const XAUDIO2_LOG_STREAMING: i32 = 4096;
pub const XAUDIO2_LOG_TIMING: i32 = 64;
pub const XAUDIO2_LOG_WARNINGS: i32 = 2;
pub const XAUDIO2_LOOP_INFINITE: i32 = 255;
pub const XAUDIO2_MAX_AUDIO_CHANNELS: i32 = 64;
pub const XAUDIO2_MAX_BUFFERS_SYSTEM: i32 = 2;
pub const XAUDIO2_MAX_BUFFER_BYTES: u32 = 2147483648;
pub const XAUDIO2_MAX_FILTER_FREQUENCY: f32 = 1.0;
pub const XAUDIO2_MAX_FILTER_ONEOVERQ: f32 = 1.5;
pub const XAUDIO2_MAX_FREQ_RATIO: f32 = 1024.0;
pub const XAUDIO2_MAX_INSTANCES: i32 = 8;
pub const XAUDIO2_MAX_LOOP_COUNT: i32 = 254;
pub const XAUDIO2_MAX_QUEUED_BUFFERS: i32 = 64;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MONO: i32 = 600000;
pub const XAUDIO2_MAX_RATIO_TIMES_RATE_XMA_MULTICHANNEL: i32 = 300000;
pub const XAUDIO2_MAX_SAMPLE_RATE: i32 = 384000;
pub const XAUDIO2_MAX_VOLUME_LEVEL: f32 = 16777216.0;
pub const XAUDIO2_MIN_FREQ_RATIO: f32 = 0.0009765625;
pub const XAUDIO2_MIN_SAMPLE_RATE: i32 = 1000;
pub const XAUDIO2_NO_LOOP_REGION: i32 = 0;
pub const XAUDIO2_NO_VIRTUAL_AUDIO_CLIENT: i32 = 65536;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_PERFORMANCE_DATA {
    pub AudioCyclesSinceLastQuery: super::UINT64,
    pub TotalCyclesSinceLastQuery: super::UINT64,
    pub MinimumCyclesPerQuantum: super::UINT32,
    pub MaximumCyclesPerQuantum: super::UINT32,
    pub MemoryUsageInBytes: super::UINT32,
    pub CurrentLatencyInSamples: super::UINT32,
    pub GlitchesSinceEngineStarted: super::UINT32,
    pub ActiveSourceVoiceCount: super::UINT32,
    pub TotalSourceVoiceCount: super::UINT32,
    pub ActiveSubmixVoiceCount: super::UINT32,
    pub ActiveResamplerCount: super::UINT32,
    pub ActiveMatrixMixCount: super::UINT32,
    pub ActiveXmaSourceVoices: super::UINT32,
    pub ActiveXmaStreams: super::UINT32,
}
pub const XAUDIO2_PLAY_TAILS: i32 = 32;
pub type XAUDIO2_PROCESSOR = u32;
pub const XAUDIO2_QUANTUM_DENOMINATOR: i32 = 100;
pub const XAUDIO2_QUANTUM_MS: f32 = 10.0;
pub const XAUDIO2_QUANTUM_NUMERATOR: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Default)]
pub struct XAUDIO2_SEND_DESCRIPTOR {
    pub Flags: super::UINT32,
    pub pOutputVoice: core::mem::ManuallyDrop<Option<IXAudio2Voice>>,
}
pub const XAUDIO2_SEND_USEFILTER: i32 = 128;
pub const XAUDIO2_STOP_ENGINE_WHEN_IDLE: i32 = 8192;
pub const XAUDIO2_USE_DEFAULT_PROCESSOR: i32 = 0;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_DETAILS {
    pub CreationFlags: super::UINT32,
    pub ActiveFlags: super::UINT32,
    pub InputChannels: super::UINT32,
    pub InputSampleRate: super::UINT32,
}
pub const XAUDIO2_VOICE_NOPITCH: i32 = 2;
pub const XAUDIO2_VOICE_NOSAMPLESPLAYED: i32 = 256;
pub const XAUDIO2_VOICE_NOSRC: i32 = 4;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_SENDS {
    pub SendCount: super::UINT32,
    pub pSends: *mut XAUDIO2_SEND_DESCRIPTOR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XAUDIO2_VOICE_STATE {
    pub pCurrentBufferContext: *mut core::ffi::c_void,
    pub BuffersQueued: super::UINT32,
    pub SamplesPlayed: super::UINT64,
}
pub const XAUDIO2_VOICE_USEFILTER: i32 = 8;
