#[cfg(feature = "implement_exclusive")]
pub trait IRemoteTextConnectionImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn RegisterThread(&self, threadid: u32) -> ::windows::core::Result<()>;
    fn UnregisterThread(&self, threadid: u32) -> ::windows::core::Result<()>;
    fn ReportDataReceived(&self, pdudata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteTextConnection {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Input.IRemoteTextConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteTextConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteTextConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteTextConnectionVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IRemoteTextConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IRemoteTextConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn RegisterThread<Impl: IRemoteTextConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterThread(threadid).into()
        }
        unsafe extern "system" fn UnregisterThread<Impl: IRemoteTextConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterThread(threadid).into()
        }
        unsafe extern "system" fn ReportDataReceived<Impl: IRemoteTextConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportDataReceived(::core::slice::from_raw_parts(::core::mem::transmute_copy(&pdudata), pduData_array_size as _)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteTextConnection, BASE_OFFSET>(),
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            RegisterThread: RegisterThread::<Impl, IMPL_OFFSET>,
            UnregisterThread: UnregisterThread::<Impl, IMPL_OFFSET>,
            ReportDataReceived: ReportDataReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteTextConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteTextConnectionFactoryImpl: Sized {
    fn CreateInstance(&self, connectionid: &::windows::core::GUID, pduforwarder: &::core::option::Option<RemoteTextConnectionDataHandler>) -> ::windows::core::Result<RemoteTextConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteTextConnectionFactory {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Input.IRemoteTextConnectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteTextConnectionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteTextConnectionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteTextConnectionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRemoteTextConnectionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, pduforwarder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&connectionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pduforwarder as *const <RemoteTextConnectionDataHandler as ::windows::core::Abi>::Abi as *const <RemoteTextConnectionDataHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteTextConnectionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteTextConnectionFactory as ::windows::core::Interface>::IID
    }
}
