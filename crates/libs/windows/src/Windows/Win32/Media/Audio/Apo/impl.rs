pub trait IApoAcousticEchoCancellation_Impl: Sized {}
impl IApoAcousticEchoCancellation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAcousticEchoCancellation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApoAcousticEchoCancellation_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAcousticEchoCancellation as ::windows::core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputConfiguration_Impl: Sized {
    fn AddAuxiliaryInput(&mut self, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn RemoveAuxiliaryInput(&mut self, dwinputid: u32) -> ::windows::core::Result<()>;
    fn IsInputFormatSupported(&mut self, prequestedinputformat: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<IAudioMediaType>;
}
impl IApoAuxiliaryInputConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApoAuxiliaryInputConfiguration_Vtbl {
        unsafe extern "system" fn AddAuxiliaryInput<Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAuxiliaryInput(::core::mem::transmute_copy(&dwinputid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pbydata), ::core::mem::transmute_copy(&pinputconnection)).into()
        }
        unsafe extern "system" fn RemoveAuxiliaryInput<Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAuxiliaryInput(::core::mem::transmute_copy(&dwinputid)).into()
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputFormatSupported(::core::mem::transmute(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedinputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddAuxiliaryInput: AddAuxiliaryInput::<Impl, IMPL_OFFSET>,
            RemoveAuxiliaryInput: RemoveAuxiliaryInput::<Impl, IMPL_OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputConfiguration as ::windows::core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputRT_Impl: Sized {
    fn AcceptInput(&mut self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY);
}
impl IApoAuxiliaryInputRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputRT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApoAuxiliaryInputRT_Vtbl {
        unsafe extern "system" fn AcceptInput<Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptInput(::core::mem::transmute_copy(&dwinputid), ::core::mem::transmute_copy(&pinputconnection))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AcceptInput: AcceptInput::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputRT as ::windows::core::Interface>::IID
    }
}
pub trait IAudioDeviceModulesClient_Impl: Sized {
    fn SetAudioDeviceModulesManager(&mut self, paudiodevicemodulesmanager: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IAudioDeviceModulesClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioDeviceModulesClient_Vtbl {
        unsafe extern "system" fn SetAudioDeviceModulesManager<Impl: IAudioDeviceModulesClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioDeviceModulesManager(::core::mem::transmute(&paudiodevicemodulesmanager)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAudioDeviceModulesManager: SetAudioDeviceModulesManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModulesClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMediaType_Impl: Sized {
    fn IsCompressedFormat(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn IsEqual(&mut self, piaudiotype: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<u32>;
    fn GetAudioFormat(&mut self) -> *mut super::WAVEFORMATEX;
    fn GetUncompressedAudioFormat(&mut self) -> ::windows::core::Result<UNCOMPRESSEDAUDIOFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioMediaType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMediaType_Vtbl {
        unsafe extern "system" fn IsCompressedFormat<Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompressedFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcompressed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piaudiotype: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&piaudiotype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioFormat<Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAudioFormat()
        }
        unsafe extern "system" fn GetUncompressedAudioFormat<Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUncompressedAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *puncompressedaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsCompressedFormat: IsCompressedFormat::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
            GetAudioFormat: GetAudioFormat::<Impl, IMPL_OFFSET>,
            GetUncompressedAudioFormat: GetUncompressedAudioFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMediaType as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObject_Impl: Sized {
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn GetLatency(&mut self) -> ::windows::core::Result<i64>;
    fn GetRegistrationProperties(&mut self) -> ::windows::core::Result<*mut APO_REG_PROPERTIES>;
    fn Initialize(&mut self, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::Result<()>;
    fn IsInputFormatSupported(&mut self, poppositeformat: &::core::option::Option<IAudioMediaType>, prequestedinputformat: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<IAudioMediaType>;
    fn IsOutputFormatSupported(&mut self, poppositeformat: &::core::option::Option<IAudioMediaType>, prequestedoutputformat: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<IAudioMediaType>;
    fn GetInputChannelCount(&mut self) -> ::windows::core::Result<u32>;
}
impl IAudioProcessingObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObject_Vtbl {
        unsafe extern "system" fn Reset<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetLatency<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationProperties<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistrationProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppregprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pbydata)).into()
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputFormatSupported(::core::mem::transmute(&poppositeformat), ::core::mem::transmute(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedinputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedoutputformat: ::windows::core::RawPtr, ppsupportedoutputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOutputFormatSupported(::core::mem::transmute(&poppositeformat), ::core::mem::transmute(&prequestedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedoutputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputChannelCount<Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pu32channelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reset: Reset::<Impl, IMPL_OFFSET>,
            GetLatency: GetLatency::<Impl, IMPL_OFFSET>,
            GetRegistrationProperties: GetRegistrationProperties::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Impl, IMPL_OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Impl, IMPL_OFFSET>,
            GetInputChannelCount: GetInputChannelCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObject as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectConfiguration_Impl: Sized {
    fn LockForProcess(&mut self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn UnlockForProcess(&mut self) -> ::windows::core::Result<()>;
}
impl IAudioProcessingObjectConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectConfiguration_Vtbl {
        unsafe extern "system" fn LockForProcess<Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockForProcess(::core::mem::transmute_copy(&u32numinputconnections), ::core::mem::transmute_copy(&ppinputconnections), ::core::mem::transmute_copy(&u32numoutputconnections), ::core::mem::transmute_copy(&ppoutputconnections)).into()
        }
        unsafe extern "system" fn UnlockForProcess<Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnlockForProcess().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            LockForProcess: LockForProcess::<Impl, IMPL_OFFSET>,
            UnlockForProcess: UnlockForProcess::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioProcessingObjectLoggingService_Impl: Sized {
    fn ApoLog(&mut self, level: APO_LOG_LEVEL, format: super::super::super::Foundation::PWSTR);
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioProcessingObjectLoggingService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectLoggingService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectLoggingService_Vtbl {
        unsafe extern "system" fn ApoLog<Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: super::super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApoLog(::core::mem::transmute_copy(&level), ::core::mem::transmute_copy(&format))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ApoLog: ApoLog::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectLoggingService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAudioProcessingObjectNotifications_Impl: Sized {
    fn GetApoNotificationRegistrationInfo(&mut self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::Result<()>;
    fn HandleNotification(&mut self, aponotification: *const APO_NOTIFICATION);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IAudioProcessingObjectNotifications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectNotifications_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectNotifications_Vtbl {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo<Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApoNotificationRegistrationInfo(::core::mem::transmute_copy(&aponotifications), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn HandleNotification<Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleNotification(::core::mem::transmute_copy(&aponotification))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetApoNotificationRegistrationInfo: GetApoNotificationRegistrationInfo::<Impl, IMPL_OFFSET>,
            HandleNotification: HandleNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectNotifications as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRT_Impl: Sized {
    fn APOProcess(&mut self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY);
    fn CalcInputFrames(&mut self, u32outputframecount: u32) -> u32;
    fn CalcOutputFrames(&mut self, u32inputframecount: u32) -> u32;
}
impl IAudioProcessingObjectRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectRT_Vtbl {
        unsafe extern "system" fn APOProcess<Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).APOProcess(::core::mem::transmute_copy(&u32numinputconnections), ::core::mem::transmute_copy(&ppinputconnections), ::core::mem::transmute_copy(&u32numoutputconnections), ::core::mem::transmute_copy(&ppoutputconnections))
        }
        unsafe extern "system" fn CalcInputFrames<Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CalcInputFrames(::core::mem::transmute_copy(&u32outputframecount))
        }
        unsafe extern "system" fn CalcOutputFrames<Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CalcOutputFrames(::core::mem::transmute_copy(&u32inputframecount))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            APOProcess: APOProcess::<Impl, IMPL_OFFSET>,
            CalcInputFrames: CalcInputFrames::<Impl, IMPL_OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRT as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRTQueueService_Impl: Sized {
    fn GetRealTimeWorkQueue(&mut self) -> ::windows::core::Result<u32>;
}
impl IAudioProcessingObjectRTQueueService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTQueueService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectRTQueueService_Vtbl {
        unsafe extern "system" fn GetRealTimeWorkQueue<Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRealTimeWorkQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *workqueueid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetRealTimeWorkQueue: GetRealTimeWorkQueue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRTQueueService as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectVBR_Impl: Sized {
    fn CalcMaxInputFrames(&mut self, u32maxoutputframecount: u32) -> ::windows::core::Result<u32>;
    fn CalcMaxOutputFrames(&mut self, u32maxinputframecount: u32) -> ::windows::core::Result<u32>;
}
impl IAudioProcessingObjectVBR_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectVBR_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioProcessingObjectVBR_Vtbl {
        unsafe extern "system" fn CalcMaxInputFrames<Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcMaxInputFrames(::core::mem::transmute_copy(&u32maxoutputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pu32inputframecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalcMaxOutputFrames<Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcMaxOutputFrames(::core::mem::transmute_copy(&u32maxinputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pu32outputframecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CalcMaxInputFrames: CalcMaxInputFrames::<Impl, IMPL_OFFSET>,
            CalcMaxOutputFrames: CalcMaxOutputFrames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectVBR as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSystemEffects_Impl: Sized {}
impl IAudioSystemEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffects_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects2_Impl: Sized + IAudioSystemEffects_Impl {
    fn GetEffectsList(&mut self, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffects2_Vtbl {
        unsafe extern "system" fn GetEffectsList<Impl: IAudioSystemEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEffectsList(::core::mem::transmute_copy(&ppeffectsids), ::core::mem::transmute_copy(&pceffects), ::core::mem::transmute_copy(&event)).into()
        }
        Self { base: IAudioSystemEffects_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetEffectsList: GetEffectsList::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects3_Impl: Sized + IAudioSystemEffects_Impl + IAudioSystemEffects2_Impl {
    fn GetControllableSystemEffectsList(&mut self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetAudioSystemEffectState(&mut self, effectid: &::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffects3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffects3_Vtbl {
        unsafe extern "system" fn GetControllableSystemEffectsList<Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetControllableSystemEffectsList(::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects), ::core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn SetAudioSystemEffectState<Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioSystemEffectState(::core::mem::transmute_copy(&effectid), ::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: IAudioSystemEffects2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetControllableSystemEffectsList: GetControllableSystemEffectsList::<Impl, IMPL_OFFSET>,
            SetAudioSystemEffectState: SetAudioSystemEffectState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffectsCustomFormats_Impl: Sized {
    fn GetFormatCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetFormat(&mut self, nformat: u32) -> ::windows::core::Result<IAudioMediaType>;
    fn GetFormatRepresentation(&mut self, nformat: u32) -> ::windows::core::Result<super::super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffectsCustomFormats_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormats_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffectsCustomFormats_Vtbl {
        unsafe extern "system" fn GetFormatCount<Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&nformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRepresentation<Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatRepresentation(::core::mem::transmute_copy(&nformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrformatrep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFormatCount: GetFormatCount::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
            GetFormatRepresentation: GetFormatRepresentation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsCustomFormats as ::windows::core::Interface>::IID
    }
}
