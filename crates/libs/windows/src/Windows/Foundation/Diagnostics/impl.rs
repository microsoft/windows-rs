pub trait IErrorReportingSettings_Impl: Sized {
    fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()>;
    fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions>;
}
impl ::windows::core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>() -> IErrorReportingSettings_Vtbl {
        unsafe extern "system" fn SetErrorOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetErrorOptions(value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IErrorReportingSettings, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&self, handler: &::core::option::Option<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for IFileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSession";
}
#[cfg(feature = "Storage")]
impl IFileLoggingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>() -> IFileLoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannel(::core::mem::transmute(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannelWithLevel(::core::mem::transmute(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLoggingChannel(::core::mem::transmute(&loggingchannel)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CloseAndSaveToFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFileGenerated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LogFileGenerated(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLogFileGenerated(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, IFileLoggingSession, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn Level(&self) -> ::windows::core::Result<LoggingLevel>;
    fn LogMessage(&self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogMessageWithLevel(&self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogValuePair(&self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()>;
    fn LogValuePairWithLevel(&self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LoggingEnabled(&self, handler: &::core::option::Option<super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel";
}
impl ILoggingChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>() -> ILoggingChannel_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Level() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogMessage(::core::mem::transmute(&eventstring)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogMessageWithLevel(::core::mem::transmute(&eventstring), level).into()
        }
        unsafe extern "system" fn LogValuePair<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogValuePair(::core::mem::transmute(&value1), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogValuePairWithLevel(::core::mem::transmute(&value1), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingEnabled(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLoggingEnabled(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingChannel, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaveToFileAsync(&self, folder: &::core::option::Option<super::super::Storage::IStorageFolder>, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows::core::RuntimeName for ILoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSession";
}
#[cfg(feature = "Storage")]
impl ILoggingSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>() -> ILoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToFileAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveToFileAsync(::core::mem::transmute(&folder), ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannel(::core::mem::transmute(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannelWithLevel(::core::mem::transmute(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLoggingChannel(::core::mem::transmute(&loggingchannel)).into()
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingSession, OFFSET>(),
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
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool>;
    fn LogEvent(&self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogEventWithFields(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndLevel(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndOptions(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
    fn StartActivity(&self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFields(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<LoggingActivity>;
}
impl ::windows::core::RuntimeName for ILoggingTarget {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingTarget";
}
impl ILoggingTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>() -> ILoggingTarget_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabledWithLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabledWithLevelAndKeywords(level, keywords) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEvent(::core::mem::transmute(&eventname)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEventWithFields(::core::mem::transmute(&eventname), ::core::mem::transmute(&fields)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEventWithFieldsAndLevel(::core::mem::transmute(&eventname), ::core::mem::transmute(&fields), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEventWithFieldsAndOptions(::core::mem::transmute(&eventname), ::core::mem::transmute(&fields), level, ::core::mem::transmute(&options)).into()
        }
        unsafe extern "system" fn StartActivity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivity(::core::mem::transmute(&starteventname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFields<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivityWithFields(::core::mem::transmute(&starteventname), ::core::mem::transmute(&fields)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivityWithFieldsAndLevel(::core::mem::transmute(&starteventname), ::core::mem::transmute(&fields), level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivityWithFieldsAndOptions(::core::mem::transmute(&starteventname), ::core::mem::transmute(&fields), level, ::core::mem::transmute(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ILoggingTarget, OFFSET>(),
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
