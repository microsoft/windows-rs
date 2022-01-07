pub trait IApoAcousticEchoCancellationImpl: Sized {}
impl ::windows::core::RuntimeName for IApoAcousticEchoCancellation {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IApoAcousticEchoCancellation";
}
impl IApoAcousticEchoCancellationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAcousticEchoCancellationImpl, const OFFSET: isize>() -> IApoAcousticEchoCancellationVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApoAcousticEchoCancellation>, ::windows::core::GetTrustLevel)
    }
}
pub trait IApoAuxiliaryInputConfigurationImpl: Sized {
    fn AddAuxiliaryInput();
    fn RemoveAuxiliaryInput();
    fn IsInputFormatSupported();
}
impl ::windows::core::RuntimeName for IApoAuxiliaryInputConfiguration {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IApoAuxiliaryInputConfiguration";
}
impl IApoAuxiliaryInputConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>() -> IApoAuxiliaryInputConfigurationVtbl {
        unsafe extern "system" fn AddAuxiliaryInput<Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, cbdatasize: u32, pbydata: *const u8, pinputconnection: *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAuxiliaryInput(dwinputid, cbdatasize, pbydata, &*(&pinputconnection as *const <APO_CONNECTION_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <APO_CONNECTION_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAuxiliaryInput<Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAuxiliaryInput(dwinputid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IApoAuxiliaryInputConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputFormatSupported(&*(&prequestedinputformat as *const <IAudioMediaType as ::windows::core::Abi>::Abi as *const <IAudioMediaType as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsupportedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApoAuxiliaryInputConfiguration>, ::windows::core::GetTrustLevel, AddAuxiliaryInput::<Impl, OFFSET>, RemoveAuxiliaryInput::<Impl, OFFSET>, IsInputFormatSupported::<Impl, OFFSET>)
    }
}
pub trait IApoAuxiliaryInputRTImpl: Sized {
    fn AcceptInput();
}
impl ::windows::core::RuntimeName for IApoAuxiliaryInputRT {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IApoAuxiliaryInputRT";
}
impl IApoAuxiliaryInputRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApoAuxiliaryInputRTImpl, const OFFSET: isize>() -> IApoAuxiliaryInputRTVtbl {
        unsafe extern "system" fn AcceptInput<Impl: IApoAuxiliaryInputRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputid: u32, pinputconnection: *const APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcceptInput(dwinputid, &*(&pinputconnection as *const <APO_CONNECTION_PROPERTY as ::windows::core::Abi>::Abi as *const <APO_CONNECTION_PROPERTY as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IApoAuxiliaryInputRT>, ::windows::core::GetTrustLevel, AcceptInput::<Impl, OFFSET>)
    }
}
pub trait IAudioDeviceModulesClientImpl: Sized {
    fn SetAudioDeviceModulesManager();
}
impl ::windows::core::RuntimeName for IAudioDeviceModulesClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioDeviceModulesClient";
}
impl IAudioDeviceModulesClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioDeviceModulesClientImpl, const OFFSET: isize>() -> IAudioDeviceModulesClientVtbl {
        unsafe extern "system" fn SetAudioDeviceModulesManager<Impl: IAudioDeviceModulesClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paudiodevicemodulesmanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAudioDeviceModulesManager(&*(&paudiodevicemodulesmanager as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioDeviceModulesClient>, ::windows::core::GetTrustLevel, SetAudioDeviceModulesManager::<Impl, OFFSET>)
    }
}
pub trait IAudioMediaTypeImpl: Sized {
    fn IsCompressedFormat();
    fn IsEqual();
    fn GetAudioFormat();
    fn GetUncompressedAudioFormat();
}
impl ::windows::core::RuntimeName for IAudioMediaType {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioMediaType";
}
impl IAudioMediaTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMediaTypeImpl, const OFFSET: isize>() -> IAudioMediaTypeVtbl {
        unsafe extern "system" fn IsCompressedFormat<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompressed: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCompressedFormat(::core::mem::transmute_copy(&pfcompressed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piaudiotype: ::windows::core::RawPtr, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&piaudiotype as *const <IAudioMediaType as ::windows::core::Abi>::Abi as *const <IAudioMediaType as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioFormat<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> *mut super::WAVEFORMATEX {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUncompressedAudioFormat<Impl: IAudioMediaTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puncompressedaudioformat: *mut UNCOMPRESSEDAUDIOFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUncompressedAudioFormat(::core::mem::transmute_copy(&puncompressedaudioformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioMediaType>, ::windows::core::GetTrustLevel, IsCompressedFormat::<Impl, OFFSET>, IsEqual::<Impl, OFFSET>, GetAudioFormat::<Impl, OFFSET>, GetUncompressedAudioFormat::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IAudioProcessingObject {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObject";
}
impl IAudioProcessingObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectImpl, const OFFSET: isize>() -> IAudioProcessingObjectVtbl {
        unsafe extern "system" fn Reset<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLatency<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLatency(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegistrationProperties<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppregprops: *mut *mut APO_REG_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistrationProperties(::core::mem::transmute_copy(&ppregprops)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdatasize: u32, pbydata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(cbdatasize, pbydata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInputFormatSupported<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedinputformat: ::windows::core::RawPtr, ppsupportedinputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInputFormatSupported(&*(&poppositeformat as *const <IAudioMediaType as ::windows::core::Abi>::Abi as *const <IAudioMediaType as ::windows::core::DefaultType>::DefaultType), &*(&prequestedinputformat as *const <IAudioMediaType as ::windows::core::Abi>::Abi as *const <IAudioMediaType as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsupportedinputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOutputFormatSupported<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poppositeformat: ::windows::core::RawPtr, prequestedoutputformat: ::windows::core::RawPtr, ppsupportedoutputformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOutputFormatSupported(&*(&poppositeformat as *const <IAudioMediaType as ::windows::core::Abi>::Abi as *const <IAudioMediaType as ::windows::core::DefaultType>::DefaultType), &*(&prequestedoutputformat as *const <IAudioMediaType as ::windows::core::Abi>::Abi as *const <IAudioMediaType as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsupportedoutputformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputChannelCount<Impl: IAudioProcessingObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputChannelCount(::core::mem::transmute_copy(&pu32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAudioProcessingObject>,
            ::windows::core::GetTrustLevel,
            Reset::<Impl, OFFSET>,
            GetLatency::<Impl, OFFSET>,
            GetRegistrationProperties::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            IsInputFormatSupported::<Impl, OFFSET>,
            IsOutputFormatSupported::<Impl, OFFSET>,
            GetInputChannelCount::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioProcessingObjectConfigurationImpl: Sized {
    fn LockForProcess();
    fn UnlockForProcess();
}
impl ::windows::core::RuntimeName for IAudioProcessingObjectConfiguration {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObjectConfiguration";
}
impl IAudioProcessingObjectConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectConfigurationImpl, const OFFSET: isize>() -> IAudioProcessingObjectConfigurationVtbl {
        unsafe extern "system" fn LockForProcess<Impl: IAudioProcessingObjectConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_DESCRIPTOR, u32numoutputconnections: u32, ppoutputconnections: *const *const APO_CONNECTION_DESCRIPTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockForProcess(u32numinputconnections, &*(&ppinputconnections as *const <APO_CONNECTION_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <APO_CONNECTION_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType), u32numoutputconnections, &*(&ppoutputconnections as *const <APO_CONNECTION_DESCRIPTOR as ::windows::core::Abi>::Abi as *const <APO_CONNECTION_DESCRIPTOR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnlockForProcess<Impl: IAudioProcessingObjectConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnlockForProcess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioProcessingObjectConfiguration>, ::windows::core::GetTrustLevel, LockForProcess::<Impl, OFFSET>, UnlockForProcess::<Impl, OFFSET>)
    }
}
pub trait IAudioProcessingObjectLoggingServiceImpl: Sized {
    fn ApoLog();
}
impl ::windows::core::RuntimeName for IAudioProcessingObjectLoggingService {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObjectLoggingService";
}
impl IAudioProcessingObjectLoggingServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectLoggingServiceImpl, const OFFSET: isize>() -> IAudioProcessingObjectLoggingServiceVtbl {
        unsafe extern "system" fn ApoLog<Impl: IAudioProcessingObjectLoggingServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: APO_LOG_LEVEL, format: super::super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApoLog(level, &*(&format as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioProcessingObjectLoggingService>, ::windows::core::GetTrustLevel, ApoLog::<Impl, OFFSET>)
    }
}
pub trait IAudioProcessingObjectNotificationsImpl: Sized {
    fn GetApoNotificationRegistrationInfo();
    fn HandleNotification();
}
impl ::windows::core::RuntimeName for IAudioProcessingObjectNotifications {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObjectNotifications";
}
impl IAudioProcessingObjectNotificationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectNotificationsImpl, const OFFSET: isize>() -> IAudioProcessingObjectNotificationsVtbl {
        unsafe extern "system" fn GetApoNotificationRegistrationInfo<Impl: IAudioProcessingObjectNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotifications: *mut *mut APO_NOTIFICATION_DESCRIPTOR, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApoNotificationRegistrationInfo(::core::mem::transmute_copy(&aponotifications), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HandleNotification<Impl: IAudioProcessingObjectNotificationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aponotification: *const APO_NOTIFICATION) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleNotification(&*(&aponotification as *const <APO_NOTIFICATION as ::windows::core::Abi>::Abi as *const <APO_NOTIFICATION as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioProcessingObjectNotifications>, ::windows::core::GetTrustLevel, GetApoNotificationRegistrationInfo::<Impl, OFFSET>, HandleNotification::<Impl, OFFSET>)
    }
}
pub trait IAudioProcessingObjectRTImpl: Sized {
    fn APOProcess();
    fn CalcInputFrames();
    fn CalcOutputFrames();
}
impl ::windows::core::RuntimeName for IAudioProcessingObjectRT {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObjectRT";
}
impl IAudioProcessingObjectRTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>() -> IAudioProcessingObjectRTVtbl {
        unsafe extern "system" fn APOProcess<Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32numinputconnections: u32, ppinputconnections: *const *const APO_CONNECTION_PROPERTY, u32numoutputconnections: u32, ppoutputconnections: *mut *mut APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).APOProcess(u32numinputconnections, &*(&ppinputconnections as *const <APO_CONNECTION_PROPERTY as ::windows::core::Abi>::Abi as *const <APO_CONNECTION_PROPERTY as ::windows::core::DefaultType>::DefaultType), u32numoutputconnections, &*(&ppoutputconnections as *const <APO_CONNECTION_PROPERTY as ::windows::core::Abi>::Abi as *const <APO_CONNECTION_PROPERTY as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CalcInputFrames<Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32outputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcInputFrames(u32outputframecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalcOutputFrames<Impl: IAudioProcessingObjectRTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32inputframecount: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcOutputFrames(u32inputframecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioProcessingObjectRT>, ::windows::core::GetTrustLevel, APOProcess::<Impl, OFFSET>, CalcInputFrames::<Impl, OFFSET>, CalcOutputFrames::<Impl, OFFSET>)
    }
}
pub trait IAudioProcessingObjectRTQueueServiceImpl: Sized {
    fn GetRealTimeWorkQueue();
}
impl ::windows::core::RuntimeName for IAudioProcessingObjectRTQueueService {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObjectRTQueueService";
}
impl IAudioProcessingObjectRTQueueServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectRTQueueServiceImpl, const OFFSET: isize>() -> IAudioProcessingObjectRTQueueServiceVtbl {
        unsafe extern "system" fn GetRealTimeWorkQueue<Impl: IAudioProcessingObjectRTQueueServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, workqueueid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRealTimeWorkQueue(::core::mem::transmute_copy(&workqueueid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioProcessingObjectRTQueueService>, ::windows::core::GetTrustLevel, GetRealTimeWorkQueue::<Impl, OFFSET>)
    }
}
pub trait IAudioProcessingObjectVBRImpl: Sized {
    fn CalcMaxInputFrames();
    fn CalcMaxOutputFrames();
}
impl ::windows::core::RuntimeName for IAudioProcessingObjectVBR {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioProcessingObjectVBR";
}
impl IAudioProcessingObjectVBRVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioProcessingObjectVBRImpl, const OFFSET: isize>() -> IAudioProcessingObjectVBRVtbl {
        unsafe extern "system" fn CalcMaxInputFrames<Impl: IAudioProcessingObjectVBRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxoutputframecount: u32, pu32inputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcMaxInputFrames(u32maxoutputframecount, ::core::mem::transmute_copy(&pu32inputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalcMaxOutputFrames<Impl: IAudioProcessingObjectVBRImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32maxinputframecount: u32, pu32outputframecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalcMaxOutputFrames(u32maxinputframecount, ::core::mem::transmute_copy(&pu32outputframecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioProcessingObjectVBR>, ::windows::core::GetTrustLevel, CalcMaxInputFrames::<Impl, OFFSET>, CalcMaxOutputFrames::<Impl, OFFSET>)
    }
}
pub trait IAudioSystemEffectsImpl: Sized {}
impl ::windows::core::RuntimeName for IAudioSystemEffects {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioSystemEffects";
}
impl IAudioSystemEffectsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsImpl, const OFFSET: isize>() -> IAudioSystemEffectsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSystemEffects>, ::windows::core::GetTrustLevel)
    }
}
pub trait IAudioSystemEffects2Impl: Sized + IAudioSystemEffectsImpl {
    fn GetEffectsList();
}
impl ::windows::core::RuntimeName for IAudioSystemEffects2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioSystemEffects2";
}
impl IAudioSystemEffects2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects2Impl, const OFFSET: isize>() -> IAudioSystemEffects2Vtbl {
        unsafe extern "system" fn GetEffectsList<Impl: IAudioSystemEffects2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppeffectsids: *mut *mut ::windows::core::GUID, pceffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectsList(::core::mem::transmute_copy(&ppeffectsids), ::core::mem::transmute_copy(&pceffects), &*(&event as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSystemEffects2>, ::windows::core::GetTrustLevel, GetEffectsList::<Impl, OFFSET>)
    }
}
pub trait IAudioSystemEffects3Impl: Sized + IAudioSystemEffects2Impl + IAudioSystemEffectsImpl {
    fn GetControllableSystemEffectsList();
    fn SetAudioSystemEffectState();
}
impl ::windows::core::RuntimeName for IAudioSystemEffects3 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioSystemEffects3";
}
impl IAudioSystemEffects3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffects3Impl, const OFFSET: isize>() -> IAudioSystemEffects3Vtbl {
        unsafe extern "system" fn GetControllableSystemEffectsList<Impl: IAudioSystemEffects3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_SYSTEMEFFECT, numeffects: *mut u32, event: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControllableSystemEffectsList(::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects), &*(&event as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioSystemEffectState<Impl: IAudioSystemEffects3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_SYSTEMEFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAudioSystemEffectState(&*(&effectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSystemEffects3>, ::windows::core::GetTrustLevel, GetControllableSystemEffectsList::<Impl, OFFSET>, SetAudioSystemEffectState::<Impl, OFFSET>)
    }
}
pub trait IAudioSystemEffectsCustomFormatsImpl: Sized {
    fn GetFormatCount();
    fn GetFormat();
    fn GetFormatRepresentation();
}
impl ::windows::core::RuntimeName for IAudioSystemEffectsCustomFormats {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Apo.IAudioSystemEffectsCustomFormats";
}
impl IAudioSystemEffectsCustomFormatsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>() -> IAudioSystemEffectsCustomFormatsVtbl {
        unsafe extern "system" fn GetFormatCount<Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatCount(::core::mem::transmute_copy(&pcformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppformat: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(nformat, ::core::mem::transmute_copy(&ppformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormatRepresentation<Impl: IAudioSystemEffectsCustomFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nformat: u32, ppwstrformatrep: *mut super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormatRepresentation(nformat, ::core::mem::transmute_copy(&ppwstrformatrep)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSystemEffectsCustomFormats>, ::windows::core::GetTrustLevel, GetFormatCount::<Impl, OFFSET>, GetFormat::<Impl, OFFSET>, GetFormatRepresentation::<Impl, OFFSET>)
    }
}
