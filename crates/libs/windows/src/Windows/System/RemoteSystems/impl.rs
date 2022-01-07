#[cfg(feature = "implement_exclusive")]
pub trait IKnownRemoteSystemCapabilitiesStaticsImpl: Sized {
    fn AppService(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteSession(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SpatialEntity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownRemoteSystemCapabilitiesStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownRemoteSystemCapabilitiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownRemoteSystemCapabilitiesStaticsImpl, const OFFSET: isize>() -> IKnownRemoteSystemCapabilitiesStaticsVtbl {
        unsafe extern "system" fn AppService<Impl: IKnownRemoteSystemCapabilitiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LaunchUri<Impl: IKnownRemoteSystemCapabilitiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteSession<Impl: IKnownRemoteSystemCapabilitiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SpatialEntity<Impl: IKnownRemoteSystemCapabilitiesStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpatialEntity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKnownRemoteSystemCapabilitiesStatics>, ::windows::core::GetTrustLevel, AppService::<Impl, OFFSET>, LaunchUri::<Impl, OFFSET>, RemoteSession::<Impl, OFFSET>, SpatialEntity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<RemoteSystemStatus>;
    fn IsAvailableByProximity(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemImpl, const OFFSET: isize>() -> IRemoteSystemVtbl {
        unsafe extern "system" fn DisplayName<Impl: IRemoteSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: IRemoteSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Kind<Impl: IRemoteSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IRemoteSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailableByProximity<Impl: IRemoteSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailableByProximity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystem>, ::windows::core::GetTrustLevel, DisplayName::<Impl, OFFSET>, Id::<Impl, OFFSET>, Kind::<Impl, OFFSET>, Status::<Impl, OFFSET>, IsAvailableByProximity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem2Impl: Sized {
    fn IsAvailableBySpatialProximity(&self) -> ::windows::core::Result<bool>;
    fn GetCapabilitySupportedAsync(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem2Impl, const OFFSET: isize>() -> IRemoteSystem2Vtbl {
        unsafe extern "system" fn IsAvailableBySpatialProximity<Impl: IRemoteSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailableBySpatialProximity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCapabilitySupportedAsync<Impl: IRemoteSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilitySupportedAsync(&*(&capabilityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystem2>, ::windows::core::GetTrustLevel, IsAvailableBySpatialProximity::<Impl, OFFSET>, GetCapabilitySupportedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem3Impl: Sized {
    fn ManufacturerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem3Impl, const OFFSET: isize>() -> IRemoteSystem3Vtbl {
        unsafe extern "system" fn ManufacturerDisplayName<Impl: IRemoteSystem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManufacturerDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModelDisplayName<Impl: IRemoteSystem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModelDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystem3>, ::windows::core::GetTrustLevel, ManufacturerDisplayName::<Impl, OFFSET>, ModelDisplayName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem4Impl: Sized {
    fn Platform(&self) -> ::windows::core::Result<RemoteSystemPlatform>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem4 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem4";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem4Impl, const OFFSET: isize>() -> IRemoteSystem4Vtbl {
        unsafe extern "system" fn Platform<Impl: IRemoteSystem4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemPlatform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Platform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystem4>, ::windows::core::GetTrustLevel, Platform::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem5Impl: Sized {
    fn Apps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem5 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem5";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem5Impl, const OFFSET: isize>() -> IRemoteSystem5Vtbl {
        unsafe extern "system" fn Apps<Impl: IRemoteSystem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Apps() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystem5>, ::windows::core::GetTrustLevel, Apps::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem6Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem6 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem6";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem6Impl, const OFFSET: isize>() -> IRemoteSystem6Vtbl {
        unsafe extern "system" fn User<Impl: IRemoteSystem6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystem6>, ::windows::core::GetTrustLevel, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAddedEventArgsImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAddedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemAddedEventArgsVtbl {
        unsafe extern "system" fn RemoteSystem<Impl: IRemoteSystemAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemAddedEventArgs>, ::windows::core::GetTrustLevel, RemoteSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsAvailableByProximity(&self) -> ::windows::core::Result<bool>;
    fn IsAvailableBySpatialProximity(&self) -> ::windows::core::Result<bool>;
    fn Attributes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemApp";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAppImpl, const OFFSET: isize>() -> IRemoteSystemAppVtbl {
        unsafe extern "system" fn Id<Impl: IRemoteSystemAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IRemoteSystemAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsAvailableByProximity<Impl: IRemoteSystemAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailableByProximity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailableBySpatialProximity<Impl: IRemoteSystemAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailableBySpatialProximity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IRemoteSystemAppImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemApp>, ::windows::core::GetTrustLevel, Id::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, IsAvailableByProximity::<Impl, OFFSET>, IsAvailableBySpatialProximity::<Impl, OFFSET>, Attributes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemApp2Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn ConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemApp2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemApp2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemApp2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemApp2Impl, const OFFSET: isize>() -> IRemoteSystemApp2Vtbl {
        unsafe extern "system" fn User<Impl: IRemoteSystemApp2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionToken<Impl: IRemoteSystemApp2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemApp2>, ::windows::core::GetTrustLevel, User::<Impl, OFFSET>, ConnectionToken::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppRegistrationImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn Attributes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAppRegistration";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAppRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAppRegistrationImpl, const OFFSET: isize>() -> IRemoteSystemAppRegistrationVtbl {
        unsafe extern "system" fn User<Impl: IRemoteSystemAppRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Impl: IRemoteSystemAppRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attributes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IRemoteSystemAppRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemAppRegistration>, ::windows::core::GetTrustLevel, User::<Impl, OFFSET>, Attributes::<Impl, OFFSET>, SaveAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppRegistrationStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<RemoteSystemAppRegistration>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<RemoteSystemAppRegistration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAppRegistrationStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAppRegistrationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAppRegistrationStaticsImpl, const OFFSET: isize>() -> IRemoteSystemAppRegistrationStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IRemoteSystemAppRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IRemoteSystemAppRegistrationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemAppRegistrationStatics>, ::windows::core::GetTrustLevel, GetDefault::<Impl, OFFSET>, GetForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAuthorizationKindFilterImpl: Sized {
    fn RemoteSystemAuthorizationKind(&self) -> ::windows::core::Result<RemoteSystemAuthorizationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAuthorizationKindFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAuthorizationKindFilterImpl, const OFFSET: isize>() -> IRemoteSystemAuthorizationKindFilterVtbl {
        unsafe extern "system" fn RemoteSystemAuthorizationKind<Impl: IRemoteSystemAuthorizationKindFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemAuthorizationKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemAuthorizationKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemAuthorizationKindFilter>, ::windows::core::GetTrustLevel, RemoteSystemAuthorizationKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAuthorizationKindFilterFactoryImpl: Sized {
    fn Create(&self, remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<RemoteSystemAuthorizationKindFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAuthorizationKindFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAuthorizationKindFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAuthorizationKindFilterFactoryImpl, const OFFSET: isize>() -> IRemoteSystemAuthorizationKindFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemAuthorizationKindFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemauthorizationkind: RemoteSystemAuthorizationKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(remotesystemauthorizationkind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemAuthorizationKindFilterFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionInfoImpl: Sized {
    fn IsProximal(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionInfoImpl, const OFFSET: isize>() -> IRemoteSystemConnectionInfoVtbl {
        unsafe extern "system" fn IsProximal<Impl: IRemoteSystemConnectionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsProximal() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionInfo>, ::windows::core::GetTrustLevel, IsProximal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionInfoStaticsImpl: Sized {
    fn TryCreateFromAppServiceConnection(&self, connection: &::core::option::Option<super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<RemoteSystemConnectionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionInfoStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionInfoStaticsImpl, const OFFSET: isize>() -> IRemoteSystemConnectionInfoStaticsVtbl {
        unsafe extern "system" fn TryCreateFromAppServiceConnection<Impl: IRemoteSystemConnectionInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateFromAppServiceConnection(&*(&connection as *const <super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionInfoStatics>, ::windows::core::GetTrustLevel, TryCreateFromAppServiceConnection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestImpl, const OFFSET: isize>() -> IRemoteSystemConnectionRequestVtbl {
        unsafe extern "system" fn RemoteSystem<Impl: IRemoteSystemConnectionRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionRequest>, ::windows::core::GetTrustLevel, RemoteSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequest2Impl: Sized {
    fn RemoteSystemApp(&self) -> ::windows::core::Result<RemoteSystemApp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequest2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequest2Impl, const OFFSET: isize>() -> IRemoteSystemConnectionRequest2Vtbl {
        unsafe extern "system" fn RemoteSystemApp<Impl: IRemoteSystemConnectionRequest2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemApp() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionRequest2>, ::windows::core::GetTrustLevel, RemoteSystemApp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequest3Impl: Sized {
    fn ConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequest3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequest3Impl, const OFFSET: isize>() -> IRemoteSystemConnectionRequest3Vtbl {
        unsafe extern "system" fn ConnectionToken<Impl: IRemoteSystemConnectionRequest3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionRequest3>, ::windows::core::GetTrustLevel, ConnectionToken::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestFactoryImpl: Sized {
    fn Create(&self, remotesystem: &::core::option::Option<RemoteSystem>) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequestFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestFactoryImpl, const OFFSET: isize>() -> IRemoteSystemConnectionRequestFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemConnectionRequestFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystem: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&remotesystem as *const <RemoteSystem as ::windows::core::Abi>::Abi as *const <RemoteSystem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionRequestFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestStaticsImpl: Sized {
    fn CreateForApp(&self, remotesystemapp: &::core::option::Option<RemoteSystemApp>) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequestStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestStaticsImpl, const OFFSET: isize>() -> IRemoteSystemConnectionRequestStaticsVtbl {
        unsafe extern "system" fn CreateForApp<Impl: IRemoteSystemConnectionRequestStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemapp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForApp(&*(&remotesystemapp as *const <RemoteSystemApp as ::windows::core::Abi>::Abi as *const <RemoteSystemApp as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionRequestStatics>, ::windows::core::GetTrustLevel, CreateForApp::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestStatics2Impl: Sized {
    fn CreateFromConnectionToken(&self, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
    fn CreateFromConnectionTokenForUser(&self, user: &::core::option::Option<super::User>, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequestStatics2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestStatics2Impl, const OFFSET: isize>() -> IRemoteSystemConnectionRequestStatics2Vtbl {
        unsafe extern "system" fn CreateFromConnectionToken<Impl: IRemoteSystemConnectionRequestStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectiontoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromConnectionToken(&*(&connectiontoken as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromConnectionTokenForUser<Impl: IRemoteSystemConnectionRequestStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, connectiontoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromConnectionTokenForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType), &*(&connectiontoken as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemConnectionRequestStatics2>, ::windows::core::GetTrustLevel, CreateFromConnectionToken::<Impl, OFFSET>, CreateFromConnectionTokenForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemDiscoveryTypeFilterImpl: Sized {
    fn RemoteSystemDiscoveryType(&self) -> ::windows::core::Result<RemoteSystemDiscoveryType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemDiscoveryTypeFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemDiscoveryTypeFilterImpl, const OFFSET: isize>() -> IRemoteSystemDiscoveryTypeFilterVtbl {
        unsafe extern "system" fn RemoteSystemDiscoveryType<Impl: IRemoteSystemDiscoveryTypeFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemDiscoveryType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemDiscoveryType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemDiscoveryTypeFilter>, ::windows::core::GetTrustLevel, RemoteSystemDiscoveryType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemDiscoveryTypeFilterFactoryImpl: Sized {
    fn Create(&self, discoverytype: RemoteSystemDiscoveryType) -> ::windows::core::Result<RemoteSystemDiscoveryTypeFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemDiscoveryTypeFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemDiscoveryTypeFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemDiscoveryTypeFilterFactoryImpl, const OFFSET: isize>() -> IRemoteSystemDiscoveryTypeFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemDiscoveryTypeFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discoverytype: RemoteSystemDiscoveryType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(discoverytype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemDiscoveryTypeFilterFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemEnumerationCompletedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemEnumerationCompletedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemEnumerationCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemEnumerationCompletedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemEnumerationCompletedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemEnumerationCompletedEventArgsVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemEnumerationCompletedEventArgs>, ::windows::core::GetTrustLevel)
    }
}
pub trait IRemoteSystemFilterImpl: Sized {}
impl ::windows::core::RuntimeName for IRemoteSystemFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemFilter";
}
impl IRemoteSystemFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemFilterImpl, const OFFSET: isize>() -> IRemoteSystemFilterVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemFilter>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindFilterImpl: Sized {
    fn RemoteSystemKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemKindFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindFilterImpl, const OFFSET: isize>() -> IRemoteSystemKindFilterVtbl {
        unsafe extern "system" fn RemoteSystemKinds<Impl: IRemoteSystemKindFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemKinds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemKindFilter>, ::windows::core::GetTrustLevel, RemoteSystemKinds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindFilterFactoryImpl: Sized {
    fn Create(&self, remotesystemkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<RemoteSystemKindFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemKindFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemKindFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindFilterFactoryImpl, const OFFSET: isize>() -> IRemoteSystemKindFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemKindFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemkinds: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&remotesystemkinds as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemKindFilterFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindStaticsImpl: Sized {
    fn Phone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hub(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Holographic(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Xbox(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemKindStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemKindStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindStaticsImpl, const OFFSET: isize>() -> IRemoteSystemKindStaticsVtbl {
        unsafe extern "system" fn Phone<Impl: IRemoteSystemKindStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Phone() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Hub<Impl: IRemoteSystemKindStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Hub() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Holographic<Impl: IRemoteSystemKindStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Holographic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Desktop<Impl: IRemoteSystemKindStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Desktop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Xbox<Impl: IRemoteSystemKindStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Xbox() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemKindStatics>, ::windows::core::GetTrustLevel, Phone::<Impl, OFFSET>, Hub::<Impl, OFFSET>, Holographic::<Impl, OFFSET>, Desktop::<Impl, OFFSET>, Xbox::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindStatics2Impl: Sized {
    fn Iot(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tablet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Laptop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemKindStatics2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemKindStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindStatics2Impl, const OFFSET: isize>() -> IRemoteSystemKindStatics2Vtbl {
        unsafe extern "system" fn Iot<Impl: IRemoteSystemKindStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Iot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tablet<Impl: IRemoteSystemKindStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tablet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Laptop<Impl: IRemoteSystemKindStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Laptop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemKindStatics2>, ::windows::core::GetTrustLevel, Iot::<Impl, OFFSET>, Tablet::<Impl, OFFSET>, Laptop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemRemovedEventArgsImpl: Sized {
    fn RemoteSystemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemRemovedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemRemovedEventArgsVtbl {
        unsafe extern "system" fn RemoteSystemId<Impl: IRemoteSystemRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemRemovedEventArgs>, ::windows::core::GetTrustLevel, RemoteSystemId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControllerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Disconnected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateParticipantWatcher(&self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcher>;
    fn SendInvitationAsync(&self, invitee: &::core::option::Option<RemoteSystem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSession";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionImpl, const OFFSET: isize>() -> IRemoteSystemSessionVtbl {
        unsafe extern "system" fn Id<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControllerDisplayName<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControllerDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnected<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnected(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisconnected<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisconnected(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateParticipantWatcher<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateParticipantWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendInvitationAsync<Impl: IRemoteSystemSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, invitee: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendInvitationAsync(&*(&invitee as *const <RemoteSystem as ::windows::core::Abi>::Abi as *const <RemoteSystem as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IRemoteSystemSession>,
            ::windows::core::GetTrustLevel,
            Id::<Impl, OFFSET>,
            DisplayName::<Impl, OFFSET>,
            ControllerDisplayName::<Impl, OFFSET>,
            Disconnected::<Impl, OFFSET>,
            RemoveDisconnected::<Impl, OFFSET>,
            CreateParticipantWatcher::<Impl, OFFSET>,
            SendInvitationAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionAddedEventArgsImpl: Sized {
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionAddedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionAddedEventArgsVtbl {
        unsafe extern "system" fn SessionInfo<Impl: IRemoteSystemSessionAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionAddedEventArgs>, ::windows::core::GetTrustLevel, SessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionControllerImpl: Sized {
    fn JoinRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveJoinRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoveParticipantAsync(&self, pparticipant: &::core::option::Option<RemoteSystemSessionParticipant>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn CreateSessionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionController";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionControllerImpl, const OFFSET: isize>() -> IRemoteSystemSessionControllerVtbl {
        unsafe extern "system" fn JoinRequested<Impl: IRemoteSystemSessionControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JoinRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveJoinRequested<Impl: IRemoteSystemSessionControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveJoinRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveParticipantAsync<Impl: IRemoteSystemSessionControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparticipant: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveParticipantAsync(&*(&pparticipant as *const <RemoteSystemSessionParticipant as ::windows::core::Abi>::Abi as *const <RemoteSystemSessionParticipant as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSessionAsync<Impl: IRemoteSystemSessionControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSessionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionController>, ::windows::core::GetTrustLevel, JoinRequested::<Impl, OFFSET>, RemoveJoinRequested::<Impl, OFFSET>, RemoveParticipantAsync::<Impl, OFFSET>, CreateSessionAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionControllerFactoryImpl: Sized {
    fn CreateController(&self, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionController>;
    fn CreateControllerWithSessionOptions(&self, displayname: &::windows::core::HSTRING, options: &::core::option::Option<RemoteSystemSessionOptions>) -> ::windows::core::Result<RemoteSystemSessionController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionControllerFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionControllerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionControllerFactoryImpl, const OFFSET: isize>() -> IRemoteSystemSessionControllerFactoryVtbl {
        unsafe extern "system" fn CreateController<Impl: IRemoteSystemSessionControllerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateController(&*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateControllerWithSessionOptions<Impl: IRemoteSystemSessionControllerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateControllerWithSessionOptions(&*(&displayname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&options as *const <RemoteSystemSessionOptions as ::windows::core::Abi>::Abi as *const <RemoteSystemSessionOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionControllerFactory>, ::windows::core::GetTrustLevel, CreateController::<Impl, OFFSET>, CreateControllerWithSessionOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionCreationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionCreationStatus>;
    fn Session(&self) -> ::windows::core::Result<RemoteSystemSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionCreationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionCreationResultImpl, const OFFSET: isize>() -> IRemoteSystemSessionCreationResultVtbl {
        unsafe extern "system" fn Status<Impl: IRemoteSystemSessionCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionCreationStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IRemoteSystemSessionCreationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionCreationResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Session::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionDisconnectedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<RemoteSystemSessionDisconnectedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionDisconnectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionDisconnectedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionDisconnectedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IRemoteSystemSessionDisconnectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionDisconnectedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionDisconnectedEventArgs>, ::windows::core::GetTrustLevel, Reason::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControllerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JoinAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInfoImpl, const OFFSET: isize>() -> IRemoteSystemSessionInfoVtbl {
        unsafe extern "system" fn DisplayName<Impl: IRemoteSystemSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ControllerDisplayName<Impl: IRemoteSystemSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControllerDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinAsync<Impl: IRemoteSystemSessionInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JoinAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionInfo>, ::windows::core::GetTrustLevel, DisplayName::<Impl, OFFSET>, ControllerDisplayName::<Impl, OFFSET>, JoinAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationImpl: Sized {
    fn Sender(&self) -> ::windows::core::Result<RemoteSystem>;
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInvitation";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionInvitationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInvitationImpl, const OFFSET: isize>() -> IRemoteSystemSessionInvitationVtbl {
        unsafe extern "system" fn Sender<Impl: IRemoteSystemSessionInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionInfo<Impl: IRemoteSystemSessionInvitationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionInvitation>, ::windows::core::GetTrustLevel, Sender::<Impl, OFFSET>, SessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationListenerImpl: Sized {
    fn InvitationReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvitationReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionInvitationListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInvitationListenerImpl, const OFFSET: isize>() -> IRemoteSystemSessionInvitationListenerVtbl {
        unsafe extern "system" fn InvitationReceived<Impl: IRemoteSystemSessionInvitationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InvitationReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInvitationReceived<Impl: IRemoteSystemSessionInvitationListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInvitationReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionInvitationListener>, ::windows::core::GetTrustLevel, InvitationReceived::<Impl, OFFSET>, RemoveInvitationReceived::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationReceivedEventArgsImpl: Sized {
    fn Invitation(&self) -> ::windows::core::Result<RemoteSystemSessionInvitation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionInvitationReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInvitationReceivedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionInvitationReceivedEventArgsVtbl {
        unsafe extern "system" fn Invitation<Impl: IRemoteSystemSessionInvitationReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invitation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionInvitationReceivedEventArgs>, ::windows::core::GetTrustLevel, Invitation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinRequestImpl: Sized {
    fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionJoinRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionJoinRequestImpl, const OFFSET: isize>() -> IRemoteSystemSessionJoinRequestVtbl {
        unsafe extern "system" fn Participant<Impl: IRemoteSystemSessionJoinRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Accept<Impl: IRemoteSystemSessionJoinRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Accept().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionJoinRequest>, ::windows::core::GetTrustLevel, Participant::<Impl, OFFSET>, Accept::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinRequestedEventArgsImpl: Sized {
    fn JoinRequest(&self) -> ::windows::core::Result<RemoteSystemSessionJoinRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionJoinRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionJoinRequestedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionJoinRequestedEventArgsVtbl {
        unsafe extern "system" fn JoinRequest<Impl: IRemoteSystemSessionJoinRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).JoinRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IRemoteSystemSessionJoinRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionJoinRequestedEventArgs>, ::windows::core::GetTrustLevel, JoinRequest::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionJoinStatus>;
    fn Session(&self) -> ::windows::core::Result<RemoteSystemSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionJoinResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionJoinResultImpl, const OFFSET: isize>() -> IRemoteSystemSessionJoinResultVtbl {
        unsafe extern "system" fn Status<Impl: IRemoteSystemSessionJoinResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionJoinStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Session<Impl: IRemoteSystemSessionJoinResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionJoinResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, Session::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionMessageChannelImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<RemoteSystemSession>;
    fn BroadcastValueSetAsync(&self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SendValueSetAsync(&self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>, participant: &::core::option::Option<RemoteSystemSessionParticipant>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SendValueSetToParticipantsAsync(&self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>, participants: &::core::option::Option<super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ValueSetReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueSetReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionMessageChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>() -> IRemoteSystemSessionMessageChannelVtbl {
        unsafe extern "system" fn Session<Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Session() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BroadcastValueSetAsync<Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagedata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BroadcastValueSetAsync(&*(&messagedata as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendValueSetAsync<Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagedata: ::windows::core::RawPtr, participant: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendValueSetAsync(&*(&messagedata as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType), &*(&participant as *const <RemoteSystemSessionParticipant as ::windows::core::Abi>::Abi as *const <RemoteSystemSessionParticipant as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendValueSetToParticipantsAsync<Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagedata: ::windows::core::RawPtr, participants: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendValueSetToParticipantsAsync(
                &*(&messagedata as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
                &*(&participants as *const <super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant> as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueSetReceived<Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueSetReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueSetReceived<Impl: IRemoteSystemSessionMessageChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValueSetReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionMessageChannel>,
            ::windows::core::GetTrustLevel,
            Session::<Impl, OFFSET>,
            BroadcastValueSetAsync::<Impl, OFFSET>,
            SendValueSetAsync::<Impl, OFFSET>,
            SendValueSetToParticipantsAsync::<Impl, OFFSET>,
            ValueSetReceived::<Impl, OFFSET>,
            RemoveValueSetReceived::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionMessageChannelFactoryImpl: Sized {
    fn Create(&self, session: &::core::option::Option<RemoteSystemSession>, channelname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionMessageChannel>;
    fn CreateWithReliability(&self, session: &::core::option::Option<RemoteSystemSession>, channelname: &::windows::core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability) -> ::windows::core::Result<RemoteSystemSessionMessageChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionMessageChannelFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionMessageChannelFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionMessageChannelFactoryImpl, const OFFSET: isize>() -> IRemoteSystemSessionMessageChannelFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemSessionMessageChannelFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, channelname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&session as *const <RemoteSystemSession as ::windows::core::Abi>::Abi as *const <RemoteSystemSession as ::windows::core::DefaultType>::DefaultType), &*(&channelname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithReliability<Impl: IRemoteSystemSessionMessageChannelFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, channelname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, reliability: RemoteSystemSessionMessageChannelReliability, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithReliability(&*(&session as *const <RemoteSystemSession as ::windows::core::Abi>::Abi as *const <RemoteSystemSession as ::windows::core::DefaultType>::DefaultType), &*(&channelname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), reliability) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionMessageChannelFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>, CreateWithReliability::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionOptionsImpl: Sized {
    fn IsInviteOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsInviteOnly(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionOptionsImpl, const OFFSET: isize>() -> IRemoteSystemSessionOptionsVtbl {
        unsafe extern "system" fn IsInviteOnly<Impl: IRemoteSystemSessionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInviteOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsInviteOnly<Impl: IRemoteSystemSessionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsInviteOnly(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionOptions>, ::windows::core::GetTrustLevel, IsInviteOnly::<Impl, OFFSET>, SetIsInviteOnly::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
    fn GetHostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipant";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionParticipantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantImpl, const OFFSET: isize>() -> IRemoteSystemSessionParticipantVtbl {
        unsafe extern "system" fn RemoteSystem<Impl: IRemoteSystemSessionParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostNames<Impl: IRemoteSystemSessionParticipantImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostNames() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionParticipant>, ::windows::core::GetTrustLevel, RemoteSystem::<Impl, OFFSET>, GetHostNames::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantAddedEventArgsImpl: Sized {
    fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionParticipantAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantAddedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionParticipantAddedEventArgsVtbl {
        unsafe extern "system" fn Participant<Impl: IRemoteSystemSessionParticipantAddedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionParticipantAddedEventArgs>, ::windows::core::GetTrustLevel, Participant::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantRemovedEventArgsImpl: Sized {
    fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionParticipantRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantRemovedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionParticipantRemovedEventArgsVtbl {
        unsafe extern "system" fn Participant<Impl: IRemoteSystemSessionParticipantRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Participant() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionParticipantRemovedEventArgs>, ::windows::core::GetTrustLevel, Participant::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcherStatus>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionParticipantWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>() -> IRemoteSystemSessionParticipantWatcherVtbl {
        unsafe extern "system" fn Start<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Status<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionParticipantWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Added<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnumerationCompleted<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IRemoteSystemSessionParticipantWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionParticipantWatcher>,
            ::windows::core::GetTrustLevel,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            Added::<Impl, OFFSET>,
            RemoveAdded::<Impl, OFFSET>,
            Removed::<Impl, OFFSET>,
            RemoveRemoved::<Impl, OFFSET>,
            EnumerationCompleted::<Impl, OFFSET>,
            RemoveEnumerationCompleted::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionRemovedEventArgsImpl: Sized {
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionRemovedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionRemovedEventArgsVtbl {
        unsafe extern "system" fn SessionInfo<Impl: IRemoteSystemSessionRemovedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionRemovedEventArgs>, ::windows::core::GetTrustLevel, SessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<RemoteSystemSessionWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionStaticsImpl, const OFFSET: isize>() -> IRemoteSystemSessionStaticsVtbl {
        unsafe extern "system" fn CreateWatcher<Impl: IRemoteSystemSessionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionStatics>, ::windows::core::GetTrustLevel, CreateWatcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionUpdatedEventArgsImpl: Sized {
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionUpdatedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionUpdatedEventArgsVtbl {
        unsafe extern "system" fn SessionInfo<Impl: IRemoteSystemSessionUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionUpdatedEventArgs>, ::windows::core::GetTrustLevel, SessionInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionValueSetReceivedEventArgsImpl: Sized {
    fn Sender(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionValueSetReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionValueSetReceivedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemSessionValueSetReceivedEventArgsVtbl {
        unsafe extern "system" fn Sender<Impl: IRemoteSystemSessionValueSetReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sender() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IRemoteSystemSessionValueSetReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionValueSetReceivedEventArgs>, ::windows::core::GetTrustLevel, Sender::<Impl, OFFSET>, Message::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionWatcherStatus>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>() -> IRemoteSystemSessionWatcherVtbl {
        unsafe extern "system" fn Start<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Status<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemSessionWatcherStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Added<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Added(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAdded<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Updated<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Updated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdated<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Removed<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoved<Impl: IRemoteSystemSessionWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRemoteSystemSessionWatcher>,
            ::windows::core::GetTrustLevel,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Status::<Impl, OFFSET>,
            Added::<Impl, OFFSET>,
            RemoveAdded::<Impl, OFFSET>,
            Updated::<Impl, OFFSET>,
            RemoveUpdated::<Impl, OFFSET>,
            Removed::<Impl, OFFSET>,
            RemoveRemoved::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStaticsImpl: Sized {
    fn FindByHostNameAsync(&self, hostname: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>>;
    fn CreateWatcher(&self) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn CreateWatcherWithFilters(&self, filters: &::core::option::Option<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStaticsImpl, const OFFSET: isize>() -> IRemoteSystemStaticsVtbl {
        unsafe extern "system" fn FindByHostNameAsync<Impl: IRemoteSystemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindByHostNameAsync(&*(&hostname as *const <super::super::Networking::HostName as ::windows::core::Abi>::Abi as *const <super::super::Networking::HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcher<Impl: IRemoteSystemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcherWithFilters<Impl: IRemoteSystemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherWithFilters(&*(&filters as *const <super::super::Foundation::Collections::IIterable<IRemoteSystemFilter> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<IRemoteSystemFilter> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestAccessAsync<Impl: IRemoteSystemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestAccessAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemStatics>, ::windows::core::GetTrustLevel, FindByHostNameAsync::<Impl, OFFSET>, CreateWatcher::<Impl, OFFSET>, CreateWatcherWithFilters::<Impl, OFFSET>, RequestAccessAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatics2Impl: Sized {
    fn IsAuthorizationKindEnabled(&self, kind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatics2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatics2Impl, const OFFSET: isize>() -> IRemoteSystemStatics2Vtbl {
        unsafe extern "system" fn IsAuthorizationKindEnabled<Impl: IRemoteSystemStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: RemoteSystemAuthorizationKind, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAuthorizationKindEnabled(kind) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemStatics2>, ::windows::core::GetTrustLevel, IsAuthorizationKindEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatics3Impl: Sized {
    fn CreateWatcherForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn CreateWatcherWithFiltersForUser(&self, user: &::core::option::Option<super::User>, filters: &::core::option::Option<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>) -> ::windows::core::Result<RemoteSystemWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatics3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatics3Impl, const OFFSET: isize>() -> IRemoteSystemStatics3Vtbl {
        unsafe extern "system" fn CreateWatcherForUser<Impl: IRemoteSystemStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWatcherWithFiltersForUser<Impl: IRemoteSystemStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, filters: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWatcherWithFiltersForUser(&*(&user as *const <super::User as ::windows::core::Abi>::Abi as *const <super::User as ::windows::core::DefaultType>::DefaultType), &*(&filters as *const <super::super::Foundation::Collections::IIterable<IRemoteSystemFilter> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<IRemoteSystemFilter> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemStatics3>, ::windows::core::GetTrustLevel, CreateWatcherForUser::<Impl, OFFSET>, CreateWatcherWithFiltersForUser::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatusTypeFilterImpl: Sized {
    fn RemoteSystemStatusType(&self) -> ::windows::core::Result<RemoteSystemStatusType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatusTypeFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatusTypeFilterImpl, const OFFSET: isize>() -> IRemoteSystemStatusTypeFilterVtbl {
        unsafe extern "system" fn RemoteSystemStatusType<Impl: IRemoteSystemStatusTypeFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemStatusType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemStatusType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemStatusTypeFilter>, ::windows::core::GetTrustLevel, RemoteSystemStatusType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatusTypeFilterFactoryImpl: Sized {
    fn Create(&self, remotesystemstatustype: RemoteSystemStatusType) -> ::windows::core::Result<RemoteSystemStatusTypeFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatusTypeFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatusTypeFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatusTypeFilterFactoryImpl, const OFFSET: isize>() -> IRemoteSystemStatusTypeFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemStatusTypeFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemstatustype: RemoteSystemStatusType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(remotesystemstatustype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemStatusTypeFilterFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemUpdatedEventArgsImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemUpdatedEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemUpdatedEventArgsVtbl {
        unsafe extern "system" fn RemoteSystem<Impl: IRemoteSystemUpdatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemUpdatedEventArgs>, ::windows::core::GetTrustLevel, RemoteSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn RemoteSystemAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoteSystemUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoteSystemRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcher";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>() -> IRemoteSystemWatcherVtbl {
        unsafe extern "system" fn Start<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RemoteSystemAdded<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemAdded(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoteSystemAdded<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoteSystemAdded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteSystemUpdated<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemUpdated(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoteSystemUpdated<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoteSystemUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoteSystemRemoved<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteSystemRemoved(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRemoteSystemRemoved<Impl: IRemoteSystemWatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRemoteSystemRemoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRemoteSystemWatcher>,
            ::windows::core::GetTrustLevel,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            RemoteSystemAdded::<Impl, OFFSET>,
            RemoveRemoteSystemAdded::<Impl, OFFSET>,
            RemoteSystemUpdated::<Impl, OFFSET>,
            RemoveRemoteSystemUpdated::<Impl, OFFSET>,
            RemoteSystemRemoved::<Impl, OFFSET>,
            RemoveRemoteSystemRemoved::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcher2Impl: Sized {
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWatcher2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcher2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcher2Impl, const OFFSET: isize>() -> IRemoteSystemWatcher2Vtbl {
        unsafe extern "system" fn EnumerationCompleted<Impl: IRemoteSystemWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnumerationCompleted<Impl: IRemoteSystemWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnumerationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorOccurred<Impl: IRemoteSystemWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorOccurred(&*(&handler as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveErrorOccurred<Impl: IRemoteSystemWatcher2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveErrorOccurred(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemWatcher2>, ::windows::core::GetTrustLevel, EnumerationCompleted::<Impl, OFFSET>, RemoveEnumerationCompleted::<Impl, OFFSET>, ErrorOccurred::<Impl, OFFSET>, RemoveErrorOccurred::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcher3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWatcher3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcher3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWatcher3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcher3Impl, const OFFSET: isize>() -> IRemoteSystemWatcher3Vtbl {
        unsafe extern "system" fn User<Impl: IRemoteSystemWatcher3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemWatcher3>, ::windows::core::GetTrustLevel, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcherErrorOccurredEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<RemoteSystemWatcherError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWatcherErrorOccurredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcherErrorOccurredEventArgsImpl, const OFFSET: isize>() -> IRemoteSystemWatcherErrorOccurredEventArgsVtbl {
        unsafe extern "system" fn Error<Impl: IRemoteSystemWatcherErrorOccurredEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RemoteSystemWatcherError) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemWatcherErrorOccurredEventArgs>, ::windows::core::GetTrustLevel, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWebAccountFilterImpl: Sized {
    fn Account(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWebAccountFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWebAccountFilterImpl, const OFFSET: isize>() -> IRemoteSystemWebAccountFilterVtbl {
        unsafe extern "system" fn Account<Impl: IRemoteSystemWebAccountFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Account() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemWebAccountFilter>, ::windows::core::GetTrustLevel, Account::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWebAccountFilterFactoryImpl: Sized {
    fn Create(&self, account: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<RemoteSystemWebAccountFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWebAccountFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWebAccountFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWebAccountFilterFactoryImpl, const OFFSET: isize>() -> IRemoteSystemWebAccountFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IRemoteSystemWebAccountFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&account as *const <super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteSystemWebAccountFilterFactory>, ::windows::core::GetTrustLevel, Create::<Impl, OFFSET>)
    }
}
