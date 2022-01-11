pub trait IApoAcousticEchoCancellationImpl: Sized {}
impl IApoAcousticEchoCancellationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAcousticEchoCancellationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApoAcousticEchoCancellationVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAcousticEchoCancellation as ::windows::core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputConfigurationImpl: Sized {
    fn AddAuxiliaryInput();
    fn RemoveAuxiliaryInput();
    fn IsInputFormatSupported();
}
impl IApoAuxiliaryInputConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApoAuxiliaryInputConfigurationVtbl {
        unsafe extern "system" fn AddAuxiliaryInput<Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAuxiliaryInput<Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddAuxiliaryInput::<Impl, IMPL_OFFSET>, RemoveAuxiliaryInput::<Impl, IMPL_OFFSET>, IsInputFormatSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputConfiguration as ::windows::core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputRTImpl: Sized {
    fn AcceptInput();
}
impl IApoAuxiliaryInputRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputRTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApoAuxiliaryInputRTVtbl {
        unsafe extern "system" fn AcceptInput<Impl: IApoAuxiliaryInputRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AcceptInput::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputRT as ::windows::core::Interface>::IID
    }
}
pub trait IAudioDeviceModulesClientImpl: Sized {
    fn SetAudioDeviceModulesManager();
}
impl IAudioDeviceModulesClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesClientImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceModulesClientVtbl {
        unsafe extern "system" fn SetAudioDeviceModulesManager<Impl: IAudioDeviceModulesClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAudioDeviceModulesManager::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModulesClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMediaTypeImpl: Sized {
    fn IsCompressedFormat();
    fn IsEqual();
    fn GetAudioFormat();
    fn GetUncompressedAudioFormat();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioMediaTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMediaTypeVtbl {
        unsafe extern "system" fn IsCompressedFormat<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piaudiotype: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAudioFormat<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUncompressedAudioFormat<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsCompressedFormat::<Impl, IMPL_OFFSET>, IsEqual::<Impl, IMPL_OFFSET>, GetAudioFormat::<Impl, IMPL_OFFSET>, GetUncompressedAudioFormat::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMediaType as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectImpl: Sized {
    fn Reset();
    fn GetLatency();
    fn GetRegistrationProperties();
    fn Initialize();
    fn IsInputFormatSupported();
    fn IsOutputFormatSupported();
    fn GetInputChannelCount();
}
impl IAudioProcessingObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectVtbl {
        unsafe extern "system" fn Reset<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLatency<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegistrationProperties<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOutputFormatSupported<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedoutputformat: ::windows::core::RawPtr, ppsupportedoutputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputChannelCount<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Reset::<Impl, IMPL_OFFSET>, GetLatency::<Impl, IMPL_OFFSET>, GetRegistrationProperties::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, IsInputFormatSupported::<Impl, IMPL_OFFSET>, IsOutputFormatSupported::<Impl, IMPL_OFFSET>, GetInputChannelCount::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObject as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectConfigurationImpl: Sized {
    fn LockForProcess();
    fn UnlockForProcess();
}
impl IAudioProcessingObjectConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectConfigurationVtbl {
        unsafe extern "system" fn LockForProcess<Impl: IAudioProcessingObjectConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockForProcess<Impl: IAudioProcessingObjectConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, LockForProcess::<Impl, IMPL_OFFSET>, UnlockForProcess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioProcessingObjectLoggingServiceImpl: Sized {
    fn ApoLog();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioProcessingObjectLoggingServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectLoggingServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectLoggingServiceVtbl {
        unsafe extern "system" fn ApoLog<Impl: IAudioProcessingObjectLoggingServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: super::super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ApoLog::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectLoggingService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAudioProcessingObjectNotificationsImpl: Sized {
    fn GetApoNotificationRegistrationInfo();
    fn HandleNotification();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IAudioProcessingObjectNotificationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectNotificationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectNotificationsVtbl {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo<Impl: IAudioProcessingObjectNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HandleNotification<Impl: IAudioProcessingObjectNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetApoNotificationRegistrationInfo::<Impl, IMPL_OFFSET>, HandleNotification::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectNotifications as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRTImpl: Sized {
    fn APOProcess();
    fn CalcInputFrames();
    fn CalcOutputFrames();
}
impl IAudioProcessingObjectRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectRTVtbl {
        unsafe extern "system" fn APOProcess<Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalcInputFrames<Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalcOutputFrames<Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, APOProcess::<Impl, IMPL_OFFSET>, CalcInputFrames::<Impl, IMPL_OFFSET>, CalcOutputFrames::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRT as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRTQueueServiceImpl: Sized {
    fn GetRealTimeWorkQueue();
}
impl IAudioProcessingObjectRTQueueServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTQueueServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectRTQueueServiceVtbl {
        unsafe extern "system" fn GetRealTimeWorkQueue<Impl: IAudioProcessingObjectRTQueueServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRealTimeWorkQueue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRTQueueService as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectVBRImpl: Sized {
    fn CalcMaxInputFrames();
    fn CalcMaxOutputFrames();
}
impl IAudioProcessingObjectVBRVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectVBRImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectVBRVtbl {
        unsafe extern "system" fn CalcMaxInputFrames<Impl: IAudioProcessingObjectVBRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalcMaxOutputFrames<Impl: IAudioProcessingObjectVBRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CalcMaxInputFrames::<Impl, IMPL_OFFSET>, CalcMaxOutputFrames::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectVBR as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSystemEffectsImpl: Sized {}
impl IAudioSystemEffectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffectsVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects2Impl: Sized + IAudioSystemEffectsImpl {
    fn GetEffectsList();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffects2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffects2Vtbl {
        unsafe extern "system" fn GetEffectsList<Impl: IAudioSystemEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetEffectsList::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects3Impl: Sized + IAudioSystemEffects2Impl + IAudioSystemEffectsImpl {
    fn GetControllableSystemEffectsList();
    fn SetAudioSystemEffectState();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffects3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffects3Vtbl {
        unsafe extern "system" fn GetControllableSystemEffectsList<Impl: IAudioSystemEffects3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAudioSystemEffectState<Impl: IAudioSystemEffects3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetEffectsList::<Impl, IMPL_OFFSET>, GetControllableSystemEffectsList::<Impl, IMPL_OFFSET>, SetAudioSystemEffectState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffectsCustomFormatsImpl: Sized {
    fn GetFormatCount();
    fn GetFormat();
    fn GetFormatRepresentation();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffectsCustomFormatsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormatsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffectsCustomFormatsVtbl {
        unsafe extern "system" fn GetFormatCount<Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormat<Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormatRepresentation<Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFormatCount::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>, GetFormatRepresentation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsCustomFormats as ::windows::core::Interface>::IID
    }
}
