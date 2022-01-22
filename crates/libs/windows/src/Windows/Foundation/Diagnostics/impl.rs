pub trait IErrorReportingSettings_Impl: Sized {
    fn SetErrorOptions(&mut self, value: ErrorOptions) -> ::windows::core::Result<()>;
    fn GetErrorOptions(&mut self) -> ::windows::core::Result<ErrorOptions>;
}
impl ::windows::core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>() -> IErrorReportingSettings_Vtbl {
        unsafe extern "system" fn SetErrorOptions<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetErrorOptions(value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Identity: ::windows::core::IUnknownImpl, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IErrorReportingSettings, OFFSET>(),
            SetErrorOptions: SetErrorOptions::<Identity, Impl, OFFSET>,
            GetErrorOptions: GetErrorOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorReportingSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait IFileLoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn CloseAndSaveToFileAsync(&mut self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&mut self, handler: &::core::option::Option<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&mut self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for IFileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSession";
}
#[cfg(feature = "Storage")]
impl IFileLoggingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>() -> IFileLoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLoggingChannelWithLevel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CloseAndSaveToFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFileGenerated<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LogFileGenerated(&*(&handler as *const <super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs> as ::windows::core::Abi>::Abi as *const <super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Identity: ::windows::core::IUnknownImpl, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveLogFileGenerated(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFileLoggingSession, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
            CloseAndSaveToFileAsync: CloseAndSaveToFileAsync::<Identity, Impl, OFFSET>,
            LogFileGenerated: LogFileGenerated::<Identity, Impl, OFFSET>,
            RemoveLogFileGenerated: RemoveLogFileGenerated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileLoggingSession as ::windows::core::Interface>::IID
    }
}
pub trait ILoggingChannel_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Enabled(&mut self) -> ::windows::core::Result<bool>;
    fn Level(&mut self) -> ::windows::core::Result<LoggingLevel>;
    fn LogMessage(&mut self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogMessageWithLevel(&mut self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogValuePair(&mut self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()>;
    fn LogValuePairWithLevel(&mut self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LoggingEnabled(&mut self, handler: &::core::option::Option<super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&mut self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel";
}
impl ILoggingChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>() -> ILoggingChannel_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Level() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogMessage<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogMessage(&*(&eventstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogMessageWithLevel(&*(&eventstring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn LogValuePair<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogValuePair(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogValuePairWithLevel(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoggingEnabled(&*(&handler as *const <super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveLoggingEnabled(&*(&token as *const <super::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannel, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            Level: Level::<Identity, Impl, OFFSET>,
            LogMessage: LogMessage::<Identity, Impl, OFFSET>,
            LogMessageWithLevel: LogMessageWithLevel::<Identity, Impl, OFFSET>,
            LogValuePair: LogValuePair::<Identity, Impl, OFFSET>,
            LogValuePairWithLevel: LogValuePairWithLevel::<Identity, Impl, OFFSET>,
            LoggingEnabled: LoggingEnabled::<Identity, Impl, OFFSET>,
            RemoveLoggingEnabled: RemoveLoggingEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Storage")]
pub trait ILoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaveToFileAsync(&mut self, folder: &::core::option::Option<super::super::Storage::IStorageFolder>, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&mut self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for ILoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSession";
}
#[cfg(feature = "Storage")]
impl ILoggingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const OFFSET: isize>() -> ILoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToFileAsync<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SaveToFileAsync(&*(&folder as *const <super::super::Storage::IStorageFolder as ::windows::core::Abi>::Abi as *const <super::super::Storage::IStorageFolder as ::windows::core::DefaultType>::DefaultType), &*(&filename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLoggingChannelWithLevel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveLoggingChannel(&*(&loggingchannel as *const <ILoggingChannel as ::windows::core::Abi>::Abi as *const <ILoggingChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingSession, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SaveToFileAsync: SaveToFileAsync::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingSession as ::windows::core::Interface>::IID
    }
}
pub trait ILoggingTarget_Impl: Sized {
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevel(&mut self, level: LoggingLevel) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&mut self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool>;
    fn LogEvent(&mut self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogEventWithFields(&mut self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndLevel(&mut self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndOptions(&mut self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
    fn StartActivity(&mut self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFields(&mut self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&mut self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&mut self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<LoggingActivity>;
}
impl ::windows::core::RuntimeName for ILoggingTarget {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingTarget";
}
impl ILoggingTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>() -> ILoggingTarget_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEnabledWithLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsEnabledWithLevelAndKeywords(level, keywords) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEvent<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogEvent(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogEventWithFields(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).LogEventWithFieldsAndLevel(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this)
                .LogEventWithFieldsAndOptions(&*(&eventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level, &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType))
                .into()
        }
        unsafe extern "system" fn StartActivity<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartActivity(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFields<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartActivityWithFields(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartActivityWithFieldsAndLevel(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Identity: ::windows::core::IUnknownImpl, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: ::windows::core::RawPtr, level: LoggingLevel, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).StartActivityWithFieldsAndOptions(&*(&starteventname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&fields as *const <LoggingFields as ::windows::core::Abi>::Abi as *const <LoggingFields as ::windows::core::DefaultType>::DefaultType), level, &*(&options as *const <LoggingOptions as ::windows::core::Abi>::Abi as *const <LoggingOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingTarget, OFFSET>(),
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            IsEnabledWithLevel: IsEnabledWithLevel::<Identity, Impl, OFFSET>,
            IsEnabledWithLevelAndKeywords: IsEnabledWithLevelAndKeywords::<Identity, Impl, OFFSET>,
            LogEvent: LogEvent::<Identity, Impl, OFFSET>,
            LogEventWithFields: LogEventWithFields::<Identity, Impl, OFFSET>,
            LogEventWithFieldsAndLevel: LogEventWithFieldsAndLevel::<Identity, Impl, OFFSET>,
            LogEventWithFieldsAndOptions: LogEventWithFieldsAndOptions::<Identity, Impl, OFFSET>,
            StartActivity: StartActivity::<Identity, Impl, OFFSET>,
            StartActivityWithFields: StartActivityWithFields::<Identity, Impl, OFFSET>,
            StartActivityWithFieldsAndLevel: StartActivityWithFieldsAndLevel::<Identity, Impl, OFFSET>,
            StartActivityWithFieldsAndOptions: StartActivityWithFieldsAndOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoggingTarget as ::windows::core::Interface>::IID
    }
}
