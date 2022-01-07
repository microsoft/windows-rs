#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StartProcessSilentlyAsync(&self, hostexepath: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>;
    fn StartProcessSilentlyWithTelemetryAsync(&self, hostexepath: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentStartProcessResult>>;
    fn ShareFolderAsync(&self, hostfolder: &::windows::core::HSTRING, requestoptions: &::core::option::Option<IsolatedWindowsEnvironmentShareFolderRequestOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>;
    fn ShareFolderWithTelemetryAsync(&self, hostfolder: &::windows::core::HSTRING, requestoptions: &::core::option::Option<IsolatedWindowsEnvironmentShareFolderRequestOptions>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFolderResult>>;
    fn LaunchFileWithUIAsync(&self, appexepath: &::windows::core::HSTRING, argumentstemplate: &::windows::core::HSTRING, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>;
    fn LaunchFileWithUIAndTelemetryAsync(&self, appexepath: &::windows::core::HSTRING, argumentstemplate: &::windows::core::HSTRING, filepath: &::windows::core::HSTRING, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentLaunchFileResult>>;
    fn TerminateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn TerminateWithTelemetryAsync(&self, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RegisterMessageReceiver(&self, receiverid: &::windows::core::GUID, messagereceivedcallback: &::core::option::Option<MessageReceivedCallback>) -> ::windows::core::Result<()>;
    fn UnregisterMessageReceiver(&self, receiverid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironment {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironment";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentVtbl {
        unsafe extern "system" fn Id<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartProcessSilentlyAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartProcessSilentlyAsync(&*(&hostexepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), activator) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartProcessSilentlyWithTelemetryAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, arguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activator: IsolatedWindowsEnvironmentActivator, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartProcessSilentlyWithTelemetryAsync(
                &*(&hostexepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                activator,
                &*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareFolderAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostfolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShareFolderAsync(&*(&hostfolder as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&requestoptions as *const <IsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareFolderWithTelemetryAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostfolder: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, requestoptions: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShareFolderWithTelemetryAsync(
                &*(&hostfolder as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&requestoptions as *const <IsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentShareFolderRequestOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFileWithUIAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFileWithUIAsync(
                &*(&appexepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&argumentstemplate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchFileWithUIAndTelemetryAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appexepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, argumentstemplate: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LaunchFileWithUIAndTelemetryAsync(
                &*(&appexepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&argumentstemplate as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminateAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateWithTelemetryAsync<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TerminateWithTelemetryAsync(&*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterMessageReceiver<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, messagereceivedcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterMessageReceiver(&*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&messagereceivedcallback as *const <MessageReceivedCallback as ::windows::core::Abi>::Abi as *const <MessageReceivedCallback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterMessageReceiver<Impl: IIsolatedWindowsEnvironmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterMessageReceiver(&*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironment>,
            base.5,
            Id::<Impl, OFFSET>,
            StartProcessSilentlyAsync::<Impl, OFFSET>,
            StartProcessSilentlyWithTelemetryAsync::<Impl, OFFSET>,
            ShareFolderAsync::<Impl, OFFSET>,
            ShareFolderWithTelemetryAsync::<Impl, OFFSET>,
            LaunchFileWithUIAsync::<Impl, OFFSET>,
            LaunchFileWithUIAndTelemetryAsync::<Impl, OFFSET>,
            TerminateAsync::<Impl, OFFSET>,
            TerminateWithTelemetryAsync::<Impl, OFFSET>,
            RegisterMessageReceiver::<Impl, OFFSET>,
            UnregisterMessageReceiver::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironment2Impl: Sized {
    fn PostMessageToReceiverAsync(&self, receiverid: &::windows::core::GUID, message: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>;
    fn PostMessageToReceiverWithTelemetryAsync(&self, receiverid: &::windows::core::GUID, message: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentPostMessageResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironment2 {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironment2";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironment2Vtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironment2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironment2Vtbl {
        unsafe extern "system" fn PostMessageToReceiverAsync<Impl: IIsolatedWindowsEnvironment2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PostMessageToReceiverAsync(&*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostMessageToReceiverWithTelemetryAsync<Impl: IIsolatedWindowsEnvironment2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PostMessageToReceiverWithTelemetryAsync(
                &*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&message as *const <super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
                &*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironment2>, base.5, PostMessageToReceiverAsync::<Impl, OFFSET>, PostMessageToReceiverWithTelemetryAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironment3Impl: Sized {
    fn GetUserInfo(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentUserInfo>;
    fn ShareFileAsync(&self, filepath: &::windows::core::HSTRING, options: &::core::option::Option<IsolatedWindowsEnvironmentShareFileRequestOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>;
    fn ShareFileWithTelemetryAsync(&self, filepath: &::windows::core::HSTRING, options: &::core::option::Option<IsolatedWindowsEnvironmentShareFileRequestOptions>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IsolatedWindowsEnvironmentShareFileResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironment3 {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironment3";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironment3Vtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironment3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironment3Vtbl {
        unsafe extern "system" fn GetUserInfo<Impl: IIsolatedWindowsEnvironment3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUserInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareFileAsync<Impl: IIsolatedWindowsEnvironment3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShareFileAsync(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <IsolatedWindowsEnvironmentShareFileRequestOptions as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentShareFileRequestOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareFileWithTelemetryAsync<Impl: IIsolatedWindowsEnvironment3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShareFileWithTelemetryAsync(
                &*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&options as *const <IsolatedWindowsEnvironmentShareFileRequestOptions as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentShareFileRequestOptions as ::windows::core::DefaultType>::DefaultType),
                &*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironment3>, base.5, GetUserInfo::<Impl, OFFSET>, ShareFileAsync::<Impl, OFFSET>, ShareFileWithTelemetryAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentCreateResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentCreateStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Environment(&self) -> ::windows::core::Result<IsolatedWindowsEnvironment>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentCreateResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentCreateResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentCreateResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentCreateResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentCreateResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentCreateStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentCreateResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Environment<Impl: IIsolatedWindowsEnvironmentCreateResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Environment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentCreateResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>, Environment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentFactoryImpl: Sized {
    fn CreateAsync(&self, options: &::core::option::Option<IsolatedWindowsEnvironmentOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>;
    fn CreateWithTelemetryAsync(&self, options: &::core::option::Option<IsolatedWindowsEnvironmentOptions>, telemetryparameters: &::core::option::Option<IsolatedWindowsEnvironmentTelemetryParameters>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IsolatedWindowsEnvironmentCreateResult, IsolatedWindowsEnvironmentCreateProgress>>;
    fn GetById(&self, environmentid: &::windows::core::HSTRING) -> ::windows::core::Result<IsolatedWindowsEnvironment>;
    fn FindByOwnerId(&self, environmentownerid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironment>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentFactory {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentFactoryVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentFactoryVtbl {
        unsafe extern "system" fn CreateAsync<Impl: IIsolatedWindowsEnvironmentFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateAsync(&*(&options as *const <IsolatedWindowsEnvironmentOptions as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithTelemetryAsync<Impl: IIsolatedWindowsEnvironmentFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, telemetryparameters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithTelemetryAsync(&*(&options as *const <IsolatedWindowsEnvironmentOptions as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentOptions as ::windows::core::DefaultType>::DefaultType), &*(&telemetryparameters as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentTelemetryParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetById<Impl: IIsolatedWindowsEnvironmentFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environmentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetById(&*(&environmentid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindByOwnerId<Impl: IIsolatedWindowsEnvironmentFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, environmentownerid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindByOwnerId(&*(&environmentownerid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentFactory>, base.5, CreateAsync::<Impl, OFFSET>, CreateWithTelemetryAsync::<Impl, OFFSET>, GetById::<Impl, OFFSET>, FindByOwnerId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentFileImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn HostPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentFile {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentFileVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentFileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentFileVtbl {
        unsafe extern "system" fn Id<Impl: IIsolatedWindowsEnvironmentFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostPath<Impl: IIsolatedWindowsEnvironmentFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HostPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IIsolatedWindowsEnvironmentFileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentFile>, base.5, Id::<Impl, OFFSET>, HostPath::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentFile2Impl: Sized {
    fn GuestPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentFile2 {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentFile2Vtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentFile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentFile2Vtbl {
        unsafe extern "system" fn GuestPath<Impl: IIsolatedWindowsEnvironmentFile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GuestPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IIsolatedWindowsEnvironmentFile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentFile2>, base.5, GuestPath::<Impl, OFFSET>, IsReadOnly::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentHostStaticsImpl: Sized {
    fn IsReady(&self) -> ::windows::core::Result<bool>;
    fn HostErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IsolatedWindowsEnvironmentHostError>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentHostStatics {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentHostStaticsVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentHostStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentHostStaticsVtbl {
        unsafe extern "system" fn IsReady<Impl: IIsolatedWindowsEnvironmentHostStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsReady() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HostErrors<Impl: IIsolatedWindowsEnvironmentHostStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HostErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentHostStatics>, base.5, IsReady::<Impl, OFFSET>, HostErrors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentLaunchFileResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentLaunchFileStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentLaunchFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentLaunchFileResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentLaunchFileResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentLaunchFileResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentLaunchFileResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentLaunchFileStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentLaunchFileResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn File<Impl: IIsolatedWindowsEnvironmentLaunchFileResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentLaunchFileResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>, File::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOptionsImpl: Sized {
    fn EnvironmentOwnerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnvironmentOwnerId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AllowedClipboardFormats(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAllowedClipboardFormats>;
    fn SetAllowedClipboardFormats(&self, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::Result<()>;
    fn ClipboardCopyPasteDirections(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentClipboardCopyPasteDirections>;
    fn SetClipboardCopyPasteDirections(&self, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::Result<()>;
    fn AvailablePrinters(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentAvailablePrinters>;
    fn SetAvailablePrinters(&self, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::Result<()>;
    fn SharedHostFolderPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SharedFolderNameInEnvironment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShareHostFolderForUntrustedItems(&self, sharedhostfolderpath: &::windows::core::HSTRING, sharefoldernameinenvironment: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PersistUserProfile(&self) -> ::windows::core::Result<bool>;
    fn SetPersistUserProfile(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowGraphicsHardwareAcceleration(&self) -> ::windows::core::Result<bool>;
    fn SetAllowGraphicsHardwareAcceleration(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCameraAndMicrophoneAccess(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCameraAndMicrophoneAccess(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentOptionsVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentOptionsVtbl {
        unsafe extern "system" fn EnvironmentOwnerId<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnvironmentOwnerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironmentOwnerId<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnvironmentOwnerId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowedClipboardFormats<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowedClipboardFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedClipboardFormats<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAllowedClipboardFormats) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowedClipboardFormats(value).into()
        }
        unsafe extern "system" fn ClipboardCopyPasteDirections<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClipboardCopyPasteDirections() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClipboardCopyPasteDirections<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentClipboardCopyPasteDirections) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClipboardCopyPasteDirections(value).into()
        }
        unsafe extern "system" fn AvailablePrinters<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AvailablePrinters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAvailablePrinters<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: IsolatedWindowsEnvironmentAvailablePrinters) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAvailablePrinters(value).into()
        }
        unsafe extern "system" fn SharedHostFolderPath<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SharedHostFolderPath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharedFolderNameInEnvironment<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SharedFolderNameInEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShareHostFolderForUntrustedItems<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharedhostfolderpath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharefoldernameinenvironment: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShareHostFolderForUntrustedItems(&*(&sharedhostfolderpath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sharefoldernameinenvironment as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PersistUserProfile<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PersistUserProfile() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPersistUserProfile<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPersistUserProfile(value).into()
        }
        unsafe extern "system" fn AllowGraphicsHardwareAcceleration<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowGraphicsHardwareAcceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowGraphicsHardwareAcceleration<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowGraphicsHardwareAcceleration(value).into()
        }
        unsafe extern "system" fn AllowCameraAndMicrophoneAccess<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowCameraAndMicrophoneAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowCameraAndMicrophoneAccess<Impl: IIsolatedWindowsEnvironmentOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowCameraAndMicrophoneAccess(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentOptions>,
            base.5,
            EnvironmentOwnerId::<Impl, OFFSET>,
            SetEnvironmentOwnerId::<Impl, OFFSET>,
            AllowedClipboardFormats::<Impl, OFFSET>,
            SetAllowedClipboardFormats::<Impl, OFFSET>,
            ClipboardCopyPasteDirections::<Impl, OFFSET>,
            SetClipboardCopyPasteDirections::<Impl, OFFSET>,
            AvailablePrinters::<Impl, OFFSET>,
            SetAvailablePrinters::<Impl, OFFSET>,
            SharedHostFolderPath::<Impl, OFFSET>,
            SharedFolderNameInEnvironment::<Impl, OFFSET>,
            ShareHostFolderForUntrustedItems::<Impl, OFFSET>,
            PersistUserProfile::<Impl, OFFSET>,
            SetPersistUserProfile::<Impl, OFFSET>,
            AllowGraphicsHardwareAcceleration::<Impl, OFFSET>,
            SetAllowGraphicsHardwareAcceleration::<Impl, OFFSET>,
            AllowCameraAndMicrophoneAccess::<Impl, OFFSET>,
            SetAllowCameraAndMicrophoneAccess::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOptions2Impl: Sized {
    fn WindowAnnotationOverride(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetWindowAnnotationOverride(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentOptions2 {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentOptions2Vtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentOptions2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentOptions2Vtbl {
        unsafe extern "system" fn WindowAnnotationOverride<Impl: IIsolatedWindowsEnvironmentOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowAnnotationOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowAnnotationOverride<Impl: IIsolatedWindowsEnvironmentOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWindowAnnotationOverride(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentOptions2>, base.5, WindowAnnotationOverride::<Impl, OFFSET>, SetWindowAnnotationOverride::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl: Sized {
    fn ShareableFolders(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProcessesRunnableAsSystem(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ProcessesRunnableAsUser(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ActivationFileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentOwnerRegistrationData {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentOwnerRegistrationDataVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentOwnerRegistrationDataVtbl {
        unsafe extern "system" fn ShareableFolders<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShareableFolders() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessesRunnableAsSystem<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessesRunnableAsSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessesRunnableAsUser<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessesRunnableAsUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationFileExtensions<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivationFileExtensions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentOwnerRegistrationData>, base.5, ShareableFolders::<Impl, OFFSET>, ProcessesRunnableAsSystem::<Impl, OFFSET>, ProcessesRunnableAsUser::<Impl, OFFSET>, ActivationFileExtensions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOwnerRegistrationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentOwnerRegistrationResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentOwnerRegistrationResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentOwnerRegistrationResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentOwnerRegistrationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentOwnerRegistrationResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentOwnerRegistrationStaticsImpl: Sized {
    fn Register(&self, ownername: &::windows::core::HSTRING, ownerregistrationdata: &::core::option::Option<IsolatedWindowsEnvironmentOwnerRegistrationData>) -> ::windows::core::Result<IsolatedWindowsEnvironmentOwnerRegistrationResult>;
    fn Unregister(&self, ownername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentOwnerRegistrationStatics {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentOwnerRegistrationStaticsVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentOwnerRegistrationStaticsVtbl {
        unsafe extern "system" fn Register<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ownername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, ownerregistrationdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Register(&*(&ownername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&ownerregistrationdata as *const <IsolatedWindowsEnvironmentOwnerRegistrationData as ::windows::core::Abi>::Abi as *const <IsolatedWindowsEnvironmentOwnerRegistrationData as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unregister<Impl: IIsolatedWindowsEnvironmentOwnerRegistrationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ownername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Unregister(&*(&ownername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentOwnerRegistrationStatics>, base.5, Register::<Impl, OFFSET>, Unregister::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentPostMessageResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentPostMessageStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentPostMessageResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentPostMessageResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentPostMessageResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentPostMessageResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentPostMessageResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentPostMessageStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentPostMessageResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentPostMessageResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentProcessImpl: Sized {
    fn State(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcessState>;
    fn ExitCode(&self) -> ::windows::core::Result<u32>;
    fn WaitForExit(&self) -> ::windows::core::Result<()>;
    fn WaitForExitWithTimeout(&self, timeoutmilliseconds: u32) -> ::windows::core::Result<()>;
    fn WaitForExitAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentProcess {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentProcessVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentProcessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentProcessVtbl {
        unsafe extern "system" fn State<Impl: IIsolatedWindowsEnvironmentProcessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentProcessState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitCode<Impl: IIsolatedWindowsEnvironmentProcessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForExit<Impl: IIsolatedWindowsEnvironmentProcessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WaitForExit().into()
        }
        unsafe extern "system" fn WaitForExitWithTimeout<Impl: IIsolatedWindowsEnvironmentProcessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeoutmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).WaitForExitWithTimeout(timeoutmilliseconds).into()
        }
        unsafe extern "system" fn WaitForExitAsync<Impl: IIsolatedWindowsEnvironmentProcessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WaitForExitAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentProcess>, base.5, State::<Impl, OFFSET>, ExitCode::<Impl, OFFSET>, WaitForExit::<Impl, OFFSET>, WaitForExitWithTimeout::<Impl, OFFSET>, WaitForExitAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFileRequestOptionsImpl: Sized {
    fn AllowWrite(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentShareFileRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentShareFileRequestOptionsVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentShareFileRequestOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentShareFileRequestOptionsVtbl {
        unsafe extern "system" fn AllowWrite<Impl: IIsolatedWindowsEnvironmentShareFileRequestOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowWrite<Impl: IIsolatedWindowsEnvironmentShareFileRequestOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowWrite(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentShareFileRequestOptions>, base.5, AllowWrite::<Impl, OFFSET>, SetAllowWrite::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFileResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFileStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn File(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentFile>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentShareFileResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentShareFileResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentShareFileResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentShareFileResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentShareFileResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFileStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentShareFileResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn File<Impl: IIsolatedWindowsEnvironmentShareFileResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).File() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentShareFileResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>, File::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFolderRequestOptionsImpl: Sized {
    fn AllowWrite(&self) -> ::windows::core::Result<bool>;
    fn SetAllowWrite(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentShareFolderRequestOptions {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentShareFolderRequestOptionsVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentShareFolderRequestOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentShareFolderRequestOptionsVtbl {
        unsafe extern "system" fn AllowWrite<Impl: IIsolatedWindowsEnvironmentShareFolderRequestOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowWrite<Impl: IIsolatedWindowsEnvironmentShareFolderRequestOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowWrite(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentShareFolderRequestOptions>, base.5, AllowWrite::<Impl, OFFSET>, SetAllowWrite::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentShareFolderResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentShareFolderStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentShareFolderResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentShareFolderResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentShareFolderResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentShareFolderResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentShareFolderResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentShareFolderStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentShareFolderResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentShareFolderResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentStartProcessResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentStartProcessStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Process(&self) -> ::windows::core::Result<IsolatedWindowsEnvironmentProcess>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentStartProcessResult {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentStartProcessResultVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentStartProcessResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentStartProcessResultVtbl {
        unsafe extern "system" fn Status<Impl: IIsolatedWindowsEnvironmentStartProcessResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IsolatedWindowsEnvironmentStartProcessStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedError<Impl: IIsolatedWindowsEnvironmentStartProcessResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Process<Impl: IIsolatedWindowsEnvironmentStartProcessResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Process() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentStartProcessResult>, base.5, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>, Process::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentTelemetryParametersImpl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetCorrelationId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentTelemetryParameters {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentTelemetryParametersVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentTelemetryParametersImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentTelemetryParametersVtbl {
        unsafe extern "system" fn CorrelationId<Impl: IIsolatedWindowsEnvironmentTelemetryParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCorrelationId<Impl: IIsolatedWindowsEnvironmentTelemetryParametersImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCorrelationId(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentTelemetryParameters>, base.5, CorrelationId::<Impl, OFFSET>, SetCorrelationId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsEnvironmentUserInfoImpl: Sized {
    fn EnvironmentUserSid(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnvironmentUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryWaitForSignInAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsEnvironmentUserInfo {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsEnvironmentUserInfoVtbl {
    pub const fn new<Impl: IIsolatedWindowsEnvironmentUserInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsEnvironmentUserInfoVtbl {
        unsafe extern "system" fn EnvironmentUserSid<Impl: IIsolatedWindowsEnvironmentUserInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnvironmentUserSid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnvironmentUserName<Impl: IIsolatedWindowsEnvironmentUserInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnvironmentUserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryWaitForSignInAsync<Impl: IIsolatedWindowsEnvironmentUserInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryWaitForSignInAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsEnvironmentUserInfo>, base.5, EnvironmentUserSid::<Impl, OFFSET>, EnvironmentUserName::<Impl, OFFSET>, TryWaitForSignInAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsHostMessengerStaticsImpl: Sized {
    fn PostMessageToReceiver(&self, receiverid: &::windows::core::GUID, message: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>) -> ::windows::core::Result<()>;
    fn GetFileId(&self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsHostMessengerStatics {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsHostMessengerStaticsVtbl {
    pub const fn new<Impl: IIsolatedWindowsHostMessengerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsHostMessengerStaticsVtbl {
        unsafe extern "system" fn PostMessageToReceiver<Impl: IIsolatedWindowsHostMessengerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, message: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PostMessageToReceiver(&*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&message as *const <super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetFileId<Impl: IIsolatedWindowsHostMessengerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFileId(&*(&filepath as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsHostMessengerStatics>, base.5, PostMessageToReceiver::<Impl, OFFSET>, GetFileId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IIsolatedWindowsHostMessengerStatics2Impl: Sized {
    fn RegisterHostMessageReceiver(&self, receiverid: &::windows::core::GUID, hostmessagereceivedcallback: &::core::option::Option<HostMessageReceivedCallback>) -> ::windows::core::Result<()>;
    fn UnregisterHostMessageReceiver(&self, receiverid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IIsolatedWindowsHostMessengerStatics2 {
    const NAME: &'static str = "Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IIsolatedWindowsHostMessengerStatics2Vtbl {
    pub const fn new<Impl: IIsolatedWindowsHostMessengerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIsolatedWindowsHostMessengerStatics2Vtbl {
        unsafe extern "system" fn RegisterHostMessageReceiver<Impl: IIsolatedWindowsHostMessengerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID, hostmessagereceivedcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterHostMessageReceiver(&*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&hostmessagereceivedcallback as *const <HostMessageReceivedCallback as ::windows::core::Abi>::Abi as *const <HostMessageReceivedCallback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UnregisterHostMessageReceiver<Impl: IIsolatedWindowsHostMessengerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, receiverid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterHostMessageReceiver(&*(&receiverid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIsolatedWindowsHostMessengerStatics2>, base.5, RegisterHostMessageReceiver::<Impl, OFFSET>, UnregisterHostMessageReceiver::<Impl, OFFSET>)
    }
}
