#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPairImpl: Sized {
    fn LocalHostName(&self) -> ::windows::core::Result<HostName>;
    fn SetLocalHostName(&self, value: &::core::option::Option<HostName>) -> ::windows::core::Result<()>;
    fn LocalServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLocalServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoteHostName(&self) -> ::windows::core::Result<HostName>;
    fn SetRemoteHostName(&self, value: &::core::option::Option<HostName>) -> ::windows::core::Result<()>;
    fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteServiceName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEndpointPair {
    const NAME: &'static str = "Windows.Networking.IEndpointPair";
}
#[cfg(feature = "implement_exclusive")]
impl IEndpointPairVtbl {
    pub const fn new<Impl: IEndpointPairImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEndpointPairVtbl {
        unsafe extern "system" fn LocalHostName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalHostName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocalHostName(&*(&value as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LocalServiceName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalServiceName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLocalServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteHostName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteHostName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRemoteHostName(&*(&value as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteServiceName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteServiceName<Impl: IEndpointPairImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRemoteServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEndpointPair>, base.5, LocalHostName::<Impl, OFFSET>, SetLocalHostName::<Impl, OFFSET>, LocalServiceName::<Impl, OFFSET>, SetLocalServiceName::<Impl, OFFSET>, RemoteHostName::<Impl, OFFSET>, SetRemoteHostName::<Impl, OFFSET>, RemoteServiceName::<Impl, OFFSET>, SetRemoteServiceName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEndpointPairFactoryImpl: Sized {
    fn CreateEndpointPair(&self, localhostname: &::core::option::Option<HostName>, localservicename: &::windows::core::HSTRING, remotehostname: &::core::option::Option<HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<EndpointPair>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEndpointPairFactory {
    const NAME: &'static str = "Windows.Networking.IEndpointPairFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IEndpointPairFactoryVtbl {
    pub const fn new<Impl: IEndpointPairFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEndpointPairFactoryVtbl {
        unsafe extern "system" fn CreateEndpointPair<Impl: IEndpointPairFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEndpointPairFactory>, base.5, CreateEndpointPair::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameImpl: Sized {
    fn IPInformation(&self) -> ::windows::core::Result<Connectivity::IPInformation>;
    fn RawName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanonicalName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Type(&self) -> ::windows::core::Result<HostNameType>;
    fn IsEqual(&self, hostname: &::core::option::Option<HostName>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostName {
    const NAME: &'static str = "Windows.Networking.IHostName";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameVtbl {
    pub const fn new<Impl: IHostNameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHostNameVtbl {
        unsafe extern "system" fn IPInformation<Impl: IHostNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IPInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawName<Impl: IHostNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RawName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IHostNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanonicalName<Impl: IHostNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanonicalName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IHostNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HostNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Impl: IHostNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEqual(&*(&hostname as *const <HostName as ::windows::core::Abi>::Abi as *const <HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHostName>, base.5, IPInformation::<Impl, OFFSET>, RawName::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, CanonicalName::<Impl, OFFSET>, Type::<Impl, OFFSET>, IsEqual::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameFactoryImpl: Sized {
    fn CreateHostName(&self, hostname: &::windows::core::HSTRING) -> ::windows::core::Result<HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostNameFactory {
    const NAME: &'static str = "Windows.Networking.IHostNameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameFactoryVtbl {
    pub const fn new<Impl: IHostNameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHostNameFactoryVtbl {
        unsafe extern "system" fn CreateHostName<Impl: IHostNameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateHostName(&*(&hostname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHostNameFactory>, base.5, CreateHostName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHostNameStaticsImpl: Sized {
    fn Compare(&self, value1: &::windows::core::HSTRING, value2: &::windows::core::HSTRING) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHostNameStatics {
    const NAME: &'static str = "Windows.Networking.IHostNameStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IHostNameStaticsVtbl {
    pub const fn new<Impl: IHostNameStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHostNameStaticsVtbl {
        unsafe extern "system" fn Compare<Impl: IHostNameStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value1: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, value2: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&value1 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&value2 as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHostNameStatics>, base.5, Compare::<Impl, OFFSET>)
    }
}
