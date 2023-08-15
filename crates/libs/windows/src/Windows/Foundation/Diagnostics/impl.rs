#[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"implement\"`*"]
pub trait IErrorReportingSettings_Impl: Sized {
    fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows_core::Result<()>;
    fn GetErrorOptions(&self) -> ::windows_core::Result<ErrorOptions>;
}
impl ::windows_core::RuntimeName for IErrorReportingSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IErrorReportingSettings";
}
impl IErrorReportingSettings_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>() -> IErrorReportingSettings_Vtbl {
        unsafe extern "system" fn SetErrorOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ErrorOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetErrorOptions(value).into()
        }
        unsafe extern "system" fn GetErrorOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IErrorReportingSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ErrorOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IErrorReportingSettings, OFFSET>(),
            SetErrorOptions: SetErrorOptions::<Identity, Impl, OFFSET>,
            GetErrorOptions: GetErrorOptions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IErrorReportingSettings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`, `\"implement\"`*"]
#[cfg(feature = "Storage")]
pub trait IFileLoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AddLoggingChannel(&self, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: ::core::option::Option<&ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows_core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
    fn CloseAndSaveToFileAsync(&self) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&self, handler: ::core::option::Option<&super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows_core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&self, token: &super::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows_core::RuntimeName for IFileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.IFileLoggingSession";
}
#[cfg(feature = "Storage")]
impl IFileLoggingSession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>() -> IFileLoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannel(::windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannelWithLevel(::windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLoggingChannel(::windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn CloseAndSaveToFileAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CloseAndSaveToFileAsync() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogFileGenerated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LogFileGenerated(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLogFileGenerated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileLoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLogFileGenerated(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IFileLoggingSession, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
            CloseAndSaveToFileAsync: CloseAndSaveToFileAsync::<Identity, Impl, OFFSET>,
            LogFileGenerated: LogFileGenerated::<Identity, Impl, OFFSET>,
            RemoveLogFileGenerated: RemoveLogFileGenerated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFileLoggingSession as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"implement\"`*"]
pub trait ILoggingChannel_Impl: Sized + super::IClosable_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Enabled(&self) -> ::windows_core::Result<bool>;
    fn Level(&self) -> ::windows_core::Result<LoggingLevel>;
    fn LogMessage(&self, eventstring: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LogMessageWithLevel(&self, eventstring: &::windows_core::HSTRING, level: LoggingLevel) -> ::windows_core::Result<()>;
    fn LogValuePair(&self, value1: &::windows_core::HSTRING, value2: i32) -> ::windows_core::Result<()>;
    fn LogValuePairWithLevel(&self, value1: &::windows_core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows_core::Result<()>;
    fn LoggingEnabled(&self, handler: ::core::option::Option<&super::TypedEventHandler<ILoggingChannel, ::windows_core::IInspectable>>) -> ::windows_core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&self, token: &super::EventRegistrationToken) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ILoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingChannel";
}
impl ILoggingChannel_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>() -> ILoggingChannel_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Level<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut LoggingLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Level() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogMessage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogMessage(::core::mem::transmute(&eventstring)).into()
        }
        unsafe extern "system" fn LogMessageWithLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventstring: ::std::mem::MaybeUninit<::windows_core::HSTRING>, level: LoggingLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogMessageWithLevel(::core::mem::transmute(&eventstring), level).into()
        }
        unsafe extern "system" fn LogValuePair<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value2: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogValuePair(::core::mem::transmute(&value1), value2).into()
        }
        unsafe extern "system" fn LogValuePairWithLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogValuePairWithLevel(::core::mem::transmute(&value1), value2, level).into()
        }
        unsafe extern "system" fn LoggingEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoggingEnabled(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoggingEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLoggingEnabled(::core::mem::transmute(&token)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILoggingChannel, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILoggingChannel as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"Storage\"`, `\"implement\"`*"]
#[cfg(feature = "Storage")]
pub trait ILoggingSession_Impl: Sized + super::IClosable_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SaveToFileAsync(&self, folder: ::core::option::Option<&super::super::Storage::IStorageFolder>, filename: &::windows_core::HSTRING) -> ::windows_core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&self, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: ::core::option::Option<&ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows_core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: ::core::option::Option<&ILoggingChannel>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Storage")]
impl ::windows_core::RuntimeName for ILoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingSession";
}
#[cfg(feature = "Storage")]
impl ILoggingSession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>() -> ILoggingSession_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToFileAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveToFileAsync(::windows_core::from_raw_borrowed(&folder), ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLoggingChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannel(::windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        unsafe extern "system" fn AddLoggingChannelWithLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void, maxlevel: LoggingLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingChannelWithLevel(::windows_core::from_raw_borrowed(&loggingchannel), maxlevel).into()
        }
        unsafe extern "system" fn RemoveLoggingChannel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loggingchannel: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveLoggingChannel(::windows_core::from_raw_borrowed(&loggingchannel)).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILoggingSession, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SaveToFileAsync: SaveToFileAsync::<Identity, Impl, OFFSET>,
            AddLoggingChannel: AddLoggingChannel::<Identity, Impl, OFFSET>,
            AddLoggingChannelWithLevel: AddLoggingChannelWithLevel::<Identity, Impl, OFFSET>,
            RemoveLoggingChannel: RemoveLoggingChannel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILoggingSession as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Foundation_Diagnostics\"`, `\"implement\"`*"]
pub trait ILoggingTarget_Impl: Sized {
    fn IsEnabled(&self) -> ::windows_core::Result<bool>;
    fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows_core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows_core::Result<bool>;
    fn LogEvent(&self, eventname: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LogEventWithFields(&self, eventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>) -> ::windows_core::Result<()>;
    fn LogEventWithFieldsAndLevel(&self, eventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel) -> ::windows_core::Result<()>;
    fn LogEventWithFieldsAndOptions(&self, eventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel, options: ::core::option::Option<&LoggingOptions>) -> ::windows_core::Result<()>;
    fn StartActivity(&self, starteventname: &::windows_core::HSTRING) -> ::windows_core::Result<LoggingActivity>;
    fn StartActivityWithFields(&self, starteventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>) -> ::windows_core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel) -> ::windows_core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows_core::HSTRING, fields: ::core::option::Option<&LoggingFields>, level: LoggingLevel, options: ::core::option::Option<&LoggingOptions>) -> ::windows_core::Result<LoggingActivity>;
}
impl ::windows_core::RuntimeName for ILoggingTarget {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ILoggingTarget";
}
impl ILoggingTarget_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>() -> ILoggingTarget_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabledWithLevel(level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledWithLevelAndKeywords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEnabledWithLevelAndKeywords(level, keywords) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LogEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEvent(::core::mem::transmute(&eventname)).into()
        }
        unsafe extern "system" fn LogEventWithFields<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEventWithFields(::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&fields)).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEventWithFieldsAndLevel(::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&fields), level).into()
        }
        unsafe extern "system" fn LogEventWithFieldsAndOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LogEventWithFieldsAndOptions(::core::mem::transmute(&eventname), ::windows_core::from_raw_borrowed(&fields), level, ::windows_core::from_raw_borrowed(&options)).into()
        }
        unsafe extern "system" fn StartActivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivity(::core::mem::transmute(&starteventname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFields<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivityWithFields(::core::mem::transmute(&starteventname), ::windows_core::from_raw_borrowed(&fields)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivityWithFieldsAndLevel(::core::mem::transmute(&starteventname), ::windows_core::from_raw_borrowed(&fields), level) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartActivityWithFieldsAndOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILoggingTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starteventname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, fields: *mut ::core::ffi::c_void, level: LoggingLevel, options: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartActivityWithFieldsAndOptions(::core::mem::transmute(&starteventname), ::windows_core::from_raw_borrowed(&fields), level, ::windows_core::from_raw_borrowed(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ILoggingTarget, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILoggingTarget as ::windows_core::ComInterface>::IID
    }
}
