#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPair_Impl: Sized {
    fn LocalHostName(&mut self) -> ::windows::core::Result<HostName>;
    fn SetLocalHostName(&mut self, value: &::core::option::Option<HostName>) -> ::windows::core::Result<()>;
    fn LocalServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalServiceName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteHostName(&mut self) -> ::windows::core::Result<HostName>;
    fn SetRemoteHostName(&mut self, value: &::core::option::Option<HostName>) -> ::windows::core::Result<()>;
    fn RemoteServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteServiceName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEndpointPair {
    const NAME: &'static str = "Windows.Networking.IEndpointPair";
}
#[cfg(feature = "implement_exclusive")]
impl IEndpointPair_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEndpointPair_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEndpointPair_Vtbl {
        unsafe extern "system" fn LocalHostName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalHostName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalHostName(&*(&value as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalServiceName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalServiceName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteHostName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteHostName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteHostName(&*(&value as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteServiceName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteServiceName<Impl: IEndpointPair_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEndpointPair, BASE_OFFSET>(),
            LocalHostName: LocalHostName::<Impl, IMPL_OFFSET>,
            SetLocalHostName: SetLocalHostName::<Impl, IMPL_OFFSET>,
            LocalServiceName: LocalServiceName::<Impl, IMPL_OFFSET>,
            SetLocalServiceName: SetLocalServiceName::<Impl, IMPL_OFFSET>,
            RemoteHostName: RemoteHostName::<Impl, IMPL_OFFSET>,
            SetRemoteHostName: SetRemoteHostName::<Impl, IMPL_OFFSET>,
            RemoteServiceName: RemoteServiceName::<Impl, IMPL_OFFSET>,
            SetRemoteServiceName: SetRemoteServiceName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEndpointPair as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPairFactory_Impl: Sized {
    fn CreateEndpointPair(&mut self, localhostname: &::core::option::Option<HostName>, localservicename: &::windows::core::HSTRING, remotehostname: &::core::option::Option<HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<EndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEndpointPairFactory {
    const NAME: &'static str = "Windows.Networking.IEndpointPairFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEndpointPairFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEndpointPairFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEndpointPairFactory_Vtbl {
        unsafe extern "system" fn CreateEndpointPair<Impl: IEndpointPairFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEndpointPair(
                &*(&localhostname as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType),
                &*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&remotehostname as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType),
                &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEndpointPairFactory, BASE_OFFSET>(),
            CreateEndpointPair: CreateEndpointPair::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEndpointPairFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IHostName_Impl: Sized {
    fn IPInformation(&mut self) -> ::windows::core::Result<Connectivity::IPInformation>;
    fn RawName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanonicalName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Type(&mut self) -> ::windows::core::Result<HostNameType>;
    fn IsEqual(&mut self, hostname: &::core::option::Option<HostName>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHostName {
    const NAME: &'static str = "Windows.Networking.IHostName";
}
#[cfg(all(feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IHostName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostName_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostName_Vtbl {
        unsafe extern "system" fn IPInformation<Impl: IHostName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IPInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawName<Impl: IHostName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IHostName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanonicalName<Impl: IHostName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanonicalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IHostName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IHostName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&hostname as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHostName, BASE_OFFSET>(),
            IPInformation: IPInformation::<Impl, IMPL_OFFSET>,
            RawName: RawName::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            CanonicalName: CanonicalName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            IsEqual: IsEqual::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostName as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameFactory_Impl: Sized {
    fn CreateHostName(&mut self, hostname: &::windows::core::HSTRING) -> ::windows::core::Result<HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostNameFactory {
    const NAME: &'static str = "Windows.Networking.IHostNameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostNameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostNameFactory_Vtbl {
        unsafe extern "system" fn CreateHostName<Impl: IHostNameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateHostName(&*(&hostname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHostNameFactory, BASE_OFFSET>(), CreateHostName: CreateHostName::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostNameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameStatics_Impl: Sized {
    fn Compare(&mut self, value1: &::windows::core::HSTRING, value2: &::windows::core::HSTRING) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostNameStatics {
    const NAME: &'static str = "Windows.Networking.IHostNameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostNameStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostNameStatics_Vtbl {
        unsafe extern "system" fn Compare<Impl: IHostNameStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value2 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IHostNameStatics, BASE_OFFSET>(), Compare: Compare::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHostNameStatics as ::windows::core::Interface>::IID
    }
}
