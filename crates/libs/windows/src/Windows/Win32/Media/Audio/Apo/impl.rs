pub trait IApoAcousticEchoCancellation_Impl: Sized {}
impl IApoAcousticEchoCancellation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAcousticEchoCancellation_Impl, const OFFSET: isize>() -> IApoAcousticEchoCancellation_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAcousticEchoCancellation as ::windows::core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputConfiguration_Impl: Sized {
    fn AddAuxiliaryInput(&self, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn RemoveAuxiliaryInput(&self, dwinputid: u32) -> ::windows::core::Result<()>;
    fn IsInputFormatSupported(&self, prequestedinputformat: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<IAudioMediaType>;
}
impl IApoAuxiliaryInputConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>() -> IApoAuxiliaryInputConfiguration_Vtbl {
        unsafe extern "system" fn AddAuxiliaryInput<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAuxiliaryInput(::core::mem::transmute_copy(&dwinputid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pbydata), ::core::mem::transmute_copy(&pinputconnection)).into()
        }
        unsafe extern "system" fn RemoveAuxiliaryInput<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAuxiliaryInput(::core::mem::transmute_copy(&dwinputid)).into()
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInputFormatSupported(::core::mem::transmute(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedinputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddAuxiliaryInput: AddAuxiliaryInput::<Identity, Impl, OFFSET>,
            RemoveAuxiliaryInput: RemoveAuxiliaryInput::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputConfiguration as ::windows::core::Interface>::IID
    }
}
pub trait IApoAuxiliaryInputRT_Impl: Sized {
    fn AcceptInput(&self, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY);
}
impl IApoAuxiliaryInputRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: isize>() -> IApoAuxiliaryInputRT_Vtbl {
        unsafe extern "system" fn AcceptInput<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcceptInput(::core::mem::transmute_copy(&dwinputid), ::core::mem::transmute_copy(&pinputconnection))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AcceptInput: AcceptInput::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApoAuxiliaryInputRT as ::windows::core::Interface>::IID
    }
}
pub trait IAudioDeviceModulesClient_Impl: Sized {
    fn SetAudioDeviceModulesManager(&self, paudiodevicemodulesmanager: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IAudioDeviceModulesClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesClient_Impl, const OFFSET: isize>() -> IAudioDeviceModulesClient_Vtbl {
        unsafe extern "system" fn SetAudioDeviceModulesManager<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAudioDeviceModulesManager(::core::mem::transmute(&paudiodevicemodulesmanager)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAudioDeviceModulesManager: SetAudioDeviceModulesManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioDeviceModulesClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMediaType_Impl: Sized {
    fn IsCompressedFormat(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn IsEqual(&self, piaudiotype: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<u32>;
    fn GetAudioFormat(&self) -> *mut super::WAVEFORMATEX;
    fn GetUncompressedAudioFormat(&self) -> ::windows::core::Result<UNCOMPRESSEDAUDIOFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioMediaType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaType_Impl, const OFFSET: isize>() -> IAudioMediaType_Vtbl {
        unsafe extern "system" fn IsCompressedFormat<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsCompressedFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcompressed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piaudiotype: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEqual(::core::mem::transmute(&piaudiotype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAudioFormat()
        }
        unsafe extern "system" fn GetUncompressedAudioFormat<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUncompressedAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *puncompressedaudioformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsCompressedFormat: IsCompressedFormat::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            GetAudioFormat: GetAudioFormat::<Identity, Impl, OFFSET>,
            GetUncompressedAudioFormat: GetUncompressedAudioFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMediaType as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObject_Impl: Sized {
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn GetLatency(&self) -> ::windows::core::Result<i64>;
    fn GetRegistrationProperties(&self) -> ::windows::core::Result<*mut APO_REG_PROPERTIES>;
    fn Initialize(&self, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::Result<()>;
    fn IsInputFormatSupported(&self, poppositeformat: &::core::option::Option<IAudioMediaType>, prequestedinputformat: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<IAudioMediaType>;
    fn IsOutputFormatSupported(&self, poppositeformat: &::core::option::Option<IAudioMediaType>, prequestedoutputformat: &::core::option::Option<IAudioMediaType>) -> ::windows::core::Result<IAudioMediaType>;
    fn GetInputChannelCount(&self) -> ::windows::core::Result<u32>;
}
impl IAudioProcessingObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>() -> IAudioProcessingObject_Vtbl {
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn GetLatency<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationProperties<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegistrationProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *ppregprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pbydata)).into()
        }
        unsafe extern "system" fn IsInputFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInputFormatSupported(::core::mem::transmute(&poppositeformat), ::core::mem::transmute(&prequestedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedinputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedoutputformat: ::windows::core::RawPtr, ppsupportedoutputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOutputFormatSupported(::core::mem::transmute(&poppositeformat), ::core::mem::transmute(&prequestedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsupportedoutputformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputChannelCount<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pu32channelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, Impl, OFFSET>,
            GetLatency: GetLatency::<Identity, Impl, OFFSET>,
            GetRegistrationProperties: GetRegistrationProperties::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            IsInputFormatSupported: IsInputFormatSupported::<Identity, Impl, OFFSET>,
            IsOutputFormatSupported: IsOutputFormatSupported::<Identity, Impl, OFFSET>,
            GetInputChannelCount: GetInputChannelCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObject as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectConfiguration_Impl: Sized {
    fn LockForProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::Result<()>;
    fn UnlockForProcess(&self) -> ::windows::core::Result<()>;
}
impl IAudioProcessingObjectConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>() -> IAudioProcessingObjectConfiguration_Vtbl {
        unsafe extern "system" fn LockForProcess<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LockForProcess(::core::mem::transmute_copy(&u32numinputconnections), ::core::mem::transmute_copy(&ppinputconnections), ::core::mem::transmute_copy(&u32numoutputconnections), ::core::mem::transmute_copy(&ppoutputconnections)).into()
        }
        unsafe extern "system" fn UnlockForProcess<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnlockForProcess().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LockForProcess: LockForProcess::<Identity, Impl, OFFSET>,
            UnlockForProcess: UnlockForProcess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectConfiguration as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectLoggingService_Impl: Sized {
    fn ApoLog(&self, level: APO_LOG_LEVEL, format: &::windows::core::PCWSTR);
}
impl IAudioProcessingObjectLoggingService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: isize>() -> IAudioProcessingObjectLoggingService_Vtbl {
        unsafe extern "system" fn ApoLog<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectLoggingService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: ::windows::core::PCWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ApoLog(::core::mem::transmute_copy(&level), ::core::mem::transmute(&format))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ApoLog: ApoLog::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectLoggingService as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IAudioProcessingObjectNotifications_Impl: Sized {
    fn GetApoNotificationRegistrationInfo(&self, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::Result<()>;
    fn HandleNotification(&self, aponotification: *const APO_NOTIFICATION);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IAudioProcessingObjectNotifications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>() -> IAudioProcessingObjectNotifications_Vtbl {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetApoNotificationRegistrationInfo(::core::mem::transmute_copy(&aponotifications), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn HandleNotification<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectNotifications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HandleNotification(::core::mem::transmute_copy(&aponotification))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetApoNotificationRegistrationInfo: GetApoNotificationRegistrationInfo::<Identity, Impl, OFFSET>,
            HandleNotification: HandleNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectNotifications as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRT_Impl: Sized {
    fn APOProcess(&self, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY);
    fn CalcInputFrames(&self, u32outputframecount: u32) -> u32;
    fn CalcOutputFrames(&self, u32inputframecount: u32) -> u32;
}
impl IAudioProcessingObjectRT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>() -> IAudioProcessingObjectRT_Vtbl {
        unsafe extern "system" fn APOProcess<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).APOProcess(::core::mem::transmute_copy(&u32numinputconnections), ::core::mem::transmute_copy(&ppinputconnections), ::core::mem::transmute_copy(&u32numoutputconnections), ::core::mem::transmute_copy(&ppoutputconnections))
        }
        unsafe extern "system" fn CalcInputFrames<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CalcInputFrames(::core::mem::transmute_copy(&u32outputframecount))
        }
        unsafe extern "system" fn CalcOutputFrames<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CalcOutputFrames(::core::mem::transmute_copy(&u32inputframecount))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            APOProcess: APOProcess::<Identity, Impl, OFFSET>,
            CalcInputFrames: CalcInputFrames::<Identity, Impl, OFFSET>,
            CalcOutputFrames: CalcOutputFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRT as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectRTQueueService_Impl: Sized {
    fn GetRealTimeWorkQueue(&self) -> ::windows::core::Result<u32>;
}
impl IAudioProcessingObjectRTQueueService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: isize>() -> IAudioProcessingObjectRTQueueService_Vtbl {
        unsafe extern "system" fn GetRealTimeWorkQueue<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTQueueService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRealTimeWorkQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *workqueueid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetRealTimeWorkQueue: GetRealTimeWorkQueue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectRTQueueService as ::windows::core::Interface>::IID
    }
}
pub trait IAudioProcessingObjectVBR_Impl: Sized {
    fn CalcMaxInputFrames(&self, u32maxoutputframecount: u32) -> ::windows::core::Result<u32>;
    fn CalcMaxOutputFrames(&self, u32maxinputframecount: u32) -> ::windows::core::Result<u32>;
}
impl IAudioProcessingObjectVBR_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>() -> IAudioProcessingObjectVBR_Vtbl {
        unsafe extern "system" fn CalcMaxInputFrames<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CalcMaxInputFrames(::core::mem::transmute_copy(&u32maxoutputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pu32inputframecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalcMaxOutputFrames<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectVBR_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CalcMaxOutputFrames(::core::mem::transmute_copy(&u32maxinputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pu32outputframecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CalcMaxInputFrames: CalcMaxInputFrames::<Identity, Impl, OFFSET>,
            CalcMaxOutputFrames: CalcMaxOutputFrames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioProcessingObjectVBR as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSystemEffects_Impl: Sized {}
impl IAudioSystemEffects_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects_Impl, const OFFSET: isize>() -> IAudioSystemEffects_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects2_Impl: Sized + IAudioSystemEffects_Impl {
    fn GetEffectsList(&self, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffects2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects2_Impl, const OFFSET: isize>() -> IAudioSystemEffects2_Vtbl {
        unsafe extern "system" fn GetEffectsList<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetEffectsList(::core::mem::transmute_copy(&ppeffectsids), ::core::mem::transmute_copy(&pceffects), ::core::mem::transmute_copy(&event)).into()
        }
        Self { base: IAudioSystemEffects_Vtbl::new::<Identity, Impl, OFFSET>(), GetEffectsList: GetEffectsList::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects2 as ::windows::core::Interface>::IID || iid == &<IAudioSystemEffects as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSystemEffects3_Impl: Sized + IAudioSystemEffects_Impl + IAudioSystemEffects2_Impl {
    fn GetControllableSystemEffectsList(&self, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn SetAudioSystemEffectState(&self, effectid: &::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSystemEffects3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>() -> IAudioSystemEffects3_Vtbl {
        unsafe extern "system" fn GetControllableSystemEffectsList<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetControllableSystemEffectsList(::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects), ::core::mem::transmute_copy(&event)).into()
        }
        unsafe extern "system" fn SetAudioSystemEffectState<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAudioSystemEffectState(::core::mem::transmute(&effectid), ::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: IAudioSystemEffects2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetControllableSystemEffectsList: GetControllableSystemEffectsList::<Identity, Impl, OFFSET>,
            SetAudioSystemEffectState: SetAudioSystemEffectState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffects3 as ::windows::core::Interface>::IID || iid == &<IAudioSystemEffects as ::windows::core::Interface>::IID || iid == &<IAudioSystemEffects2 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSystemEffectsCustomFormats_Impl: Sized {
    fn GetFormatCount(&self) -> ::windows::core::Result<u32>;
    fn GetFormat(&self, nformat: u32) -> ::windows::core::Result<IAudioMediaType>;
    fn GetFormatRepresentation(&self, nformat: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl IAudioSystemEffectsCustomFormats_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>() -> IAudioSystemEffectsCustomFormats_Vtbl {
        unsafe extern "system" fn GetFormatCount<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&nformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRepresentation<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFormatRepresentation(::core::mem::transmute_copy(&nformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrformatrep = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFormatCount: GetFormatCount::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetFormatRepresentation: GetFormatRepresentation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsCustomFormats as ::windows::core::Interface>::IID
    }
}
