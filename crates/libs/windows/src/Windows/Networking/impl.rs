#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPairImpl: Sized {
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
impl IEndpointPairVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEndpointPairImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEndpointPairVtbl {
        unsafe extern "system" fn LocalHostName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocalHostName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalHostName(&*(&value as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalServiceName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLocalServiceName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteHostName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteHostName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteHostName(&*(&value as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteServiceName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRemoteServiceName<Impl: IEndpointPairImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IEndpointPairFactoryImpl: Sized {
    fn CreateEndpointPair(&mut self, localhostname: &::core::option::Option<HostName>, localservicename: &::windows::core::HSTRING, remotehostname: &::core::option::Option<HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<EndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEndpointPairFactory {
    const NAME: &'static str = "Windows.Networking.IEndpointPairFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEndpointPairFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEndpointPairFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEndpointPairFactoryVtbl {
        unsafe extern "system" fn CreateEndpointPair<Impl: IEndpointPairFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IHostNameImpl: Sized {
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
impl IHostNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostNameVtbl {
        unsafe extern "system" fn IPInformation<Impl: IHostNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RawName<Impl: IHostNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisplayName<Impl: IHostNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanonicalName<Impl: IHostNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IHostNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEqual<Impl: IHostNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IHostNameFactoryImpl: Sized {
    fn CreateHostName(&mut self, hostname: &::windows::core::HSTRING) -> ::windows::core::Result<HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostNameFactory {
    const NAME: &'static str = "Windows.Networking.IHostNameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostNameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostNameFactoryVtbl {
        unsafe extern "system" fn CreateHostName<Impl: IHostNameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IHostNameStaticsImpl: Sized {
    fn Compare(&mut self, value1: &::windows::core::HSTRING, value2: &::windows::core::HSTRING) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostNameStatics {
    const NAME: &'static str = "Windows.Networking.IHostNameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHostNameStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHostNameStaticsVtbl {
        unsafe extern "system" fn Compare<Impl: IHostNameStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT {
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
