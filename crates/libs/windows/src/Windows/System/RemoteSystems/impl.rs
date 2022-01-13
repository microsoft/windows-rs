#[cfg(feature = "implement_exclusive")]
pub trait IKnownRemoteSystemCapabilitiesStaticsImpl: Sized {
    fn AppService(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchUri(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteSession(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SpatialEntity(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKnownRemoteSystemCapabilitiesStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IKnownRemoteSystemCapabilitiesStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKnownRemoteSystemCapabilitiesStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKnownRemoteSystemCapabilitiesStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKnownRemoteSystemCapabilitiesStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKnownRemoteSystemCapabilitiesStatics, BASE_OFFSET>(),
            AppService: AppService::<Impl, IMPL_OFFSET>,
            LaunchUri: LaunchUri::<Impl, IMPL_OFFSET>,
            RemoteSession: RemoteSession::<Impl, IMPL_OFFSET>,
            SpatialEntity: SpatialEntity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKnownRemoteSystemCapabilitiesStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&mut self) -> ::windows::core::Result<RemoteSystemStatus>;
    fn IsAvailableByProximity(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystem, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            IsAvailableByProximity: IsAvailableByProximity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystem2Impl: Sized {
    fn IsAvailableBySpatialProximity(&mut self) -> ::windows::core::Result<bool>;
    fn GetCapabilitySupportedAsync(&mut self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystem2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystem2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystem2, BASE_OFFSET>(),
            IsAvailableBySpatialProximity: IsAvailableBySpatialProximity::<Impl, IMPL_OFFSET>,
            GetCapabilitySupportedAsync: GetCapabilitySupportedAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem3Impl: Sized {
    fn ManufacturerDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystem3Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystem3, BASE_OFFSET>(),
            ManufacturerDisplayName: ManufacturerDisplayName::<Impl, IMPL_OFFSET>,
            ModelDisplayName: ModelDisplayName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem4Impl: Sized {
    fn Platform(&mut self) -> ::windows::core::Result<RemoteSystemPlatform>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem4 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem4";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystem4Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystem4, BASE_OFFSET>(), Platform: Platform::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystem4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystem5Impl: Sized {
    fn Apps(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystem5 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystem5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystem5Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystem5, BASE_OFFSET>(), Apps: Apps::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystem5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem6Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystem6 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystem6";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystem6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystem6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystem6Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystem6, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystem6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAddedEventArgsImpl: Sized {
    fn RemoteSystem(&mut self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAddedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemAddedEventArgs, BASE_OFFSET>(),
            RemoteSystem: RemoteSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemAppImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsAvailableByProximity(&mut self) -> ::windows::core::Result<bool>;
    fn IsAvailableBySpatialProximity(&mut self) -> ::windows::core::Result<bool>;
    fn Attributes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemApp";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemAppVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAppImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAppVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemApp, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            IsAvailableByProximity: IsAvailableByProximity::<Impl, IMPL_OFFSET>,
            IsAvailableBySpatialProximity: IsAvailableBySpatialProximity::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemApp as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemApp2Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::User>;
    fn ConnectionToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemApp2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemApp2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemApp2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemApp2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemApp2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemApp2, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            ConnectionToken: ConnectionToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemApp2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemAppRegistrationImpl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::User>;
    fn Attributes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAppRegistration";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemAppRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAppRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAppRegistrationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemAppRegistration, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            Attributes: Attributes::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAppRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppRegistrationStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<RemoteSystemAppRegistration>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<RemoteSystemAppRegistration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAppRegistrationStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAppRegistrationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAppRegistrationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAppRegistrationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAppRegistrationStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemAppRegistrationStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAppRegistrationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAuthorizationKindFilterImpl: Sized {
    fn RemoteSystemAuthorizationKind(&mut self) -> ::windows::core::Result<RemoteSystemAuthorizationKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAuthorizationKindFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAuthorizationKindFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAuthorizationKindFilterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemAuthorizationKindFilter, BASE_OFFSET>(),
            RemoteSystemAuthorizationKind: RemoteSystemAuthorizationKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAuthorizationKindFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAuthorizationKindFilterFactoryImpl: Sized {
    fn Create(&mut self, remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<RemoteSystemAuthorizationKindFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemAuthorizationKindFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemAuthorizationKindFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemAuthorizationKindFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemAuthorizationKindFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemAuthorizationKindFilterFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemAuthorizationKindFilterFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemAuthorizationKindFilterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionInfoImpl: Sized {
    fn IsProximal(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionInfo, BASE_OFFSET>(),
            IsProximal: IsProximal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
pub trait IRemoteSystemConnectionInfoStaticsImpl: Sized {
    fn TryCreateFromAppServiceConnection(&mut self, connection: &::core::option::Option<super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<RemoteSystemConnectionInfo>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionInfoStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionInfoStatics";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl IRemoteSystemConnectionInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionInfoStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionInfoStatics, BASE_OFFSET>(),
            TryCreateFromAppServiceConnection: TryCreateFromAppServiceConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionInfoStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestImpl: Sized {
    fn RemoteSystem(&mut self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionRequestVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionRequest, BASE_OFFSET>(),
            RemoteSystem: RemoteSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequest2Impl: Sized {
    fn RemoteSystemApp(&mut self) -> ::windows::core::Result<RemoteSystemApp>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequest2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequest2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequest2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequest2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionRequest2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionRequest2, BASE_OFFSET>(),
            RemoteSystemApp: RemoteSystemApp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionRequest2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequest3Impl: Sized {
    fn ConnectionToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequest3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequest3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequest3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequest3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionRequest3Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionRequest3, BASE_OFFSET>(),
            ConnectionToken: ConnectionToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionRequest3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestFactoryImpl: Sized {
    fn Create(&mut self, remotesystem: &::core::option::Option<RemoteSystem>) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequestFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequestFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionRequestFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionRequestFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionRequestFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestStaticsImpl: Sized {
    fn CreateForApp(&mut self, remotesystemapp: &::core::option::Option<RemoteSystemApp>) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequestStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionRequestStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionRequestStatics, BASE_OFFSET>(),
            CreateForApp: CreateForApp::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionRequestStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestStatics2Impl: Sized {
    fn CreateFromConnectionToken(&mut self, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
    fn CreateFromConnectionTokenForUser(&mut self, user: &::core::option::Option<super::User>, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemConnectionRequestStatics2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemConnectionRequestStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemConnectionRequestStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemConnectionRequestStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemConnectionRequestStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemConnectionRequestStatics2, BASE_OFFSET>(),
            CreateFromConnectionToken: CreateFromConnectionToken::<Impl, IMPL_OFFSET>,
            CreateFromConnectionTokenForUser: CreateFromConnectionTokenForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemConnectionRequestStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemDiscoveryTypeFilterImpl: Sized {
    fn RemoteSystemDiscoveryType(&mut self) -> ::windows::core::Result<RemoteSystemDiscoveryType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemDiscoveryTypeFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemDiscoveryTypeFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemDiscoveryTypeFilterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemDiscoveryTypeFilter, BASE_OFFSET>(),
            RemoteSystemDiscoveryType: RemoteSystemDiscoveryType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemDiscoveryTypeFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemDiscoveryTypeFilterFactoryImpl: Sized {
    fn Create(&mut self, discoverytype: RemoteSystemDiscoveryType) -> ::windows::core::Result<RemoteSystemDiscoveryTypeFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemDiscoveryTypeFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemDiscoveryTypeFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemDiscoveryTypeFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemDiscoveryTypeFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemDiscoveryTypeFilterFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemDiscoveryTypeFilterFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemDiscoveryTypeFilterFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemEnumerationCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemEnumerationCompletedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemEnumerationCompletedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemEnumerationCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IRemoteSystemFilterImpl: Sized {}
impl ::windows::core::RuntimeName for IRemoteSystemFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemFilter";
}
impl IRemoteSystemFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemFilterVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemFilter, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemKindFilterImpl: Sized {
    fn RemoteSystemKinds(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemKindFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemKindFilterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemKindFilter, BASE_OFFSET>(),
            RemoteSystemKinds: RemoteSystemKinds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemKindFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemKindFilterFactoryImpl: Sized {
    fn Create(&mut self, remotesystemkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<RemoteSystemKindFilter>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemKindFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindFilterFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemKindFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemKindFilterFactoryVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemKindFilterFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemKindFilterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindStaticsImpl: Sized {
    fn Phone(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hub(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Holographic(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Xbox(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemKindStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemKindStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemKindStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemKindStatics, BASE_OFFSET>(),
            Phone: Phone::<Impl, IMPL_OFFSET>,
            Hub: Hub::<Impl, IMPL_OFFSET>,
            Holographic: Holographic::<Impl, IMPL_OFFSET>,
            Desktop: Desktop::<Impl, IMPL_OFFSET>,
            Xbox: Xbox::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemKindStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindStatics2Impl: Sized {
    fn Iot(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tablet(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Laptop(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemKindStatics2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemKindStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemKindStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemKindStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemKindStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemKindStatics2, BASE_OFFSET>(),
            Iot: Iot::<Impl, IMPL_OFFSET>,
            Tablet: Tablet::<Impl, IMPL_OFFSET>,
            Laptop: Laptop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemKindStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemRemovedEventArgsImpl: Sized {
    fn RemoteSystemId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemRemovedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemRemovedEventArgs, BASE_OFFSET>(),
            RemoteSystemId: RemoteSystemId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControllerDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Disconnected(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateParticipantWatcher(&mut self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcher>;
    fn SendInvitationAsync(&mut self, invitee: &::core::option::Option<RemoteSystem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSession, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            ControllerDisplayName: ControllerDisplayName::<Impl, IMPL_OFFSET>,
            Disconnected: Disconnected::<Impl, IMPL_OFFSET>,
            RemoveDisconnected: RemoveDisconnected::<Impl, IMPL_OFFSET>,
            CreateParticipantWatcher: CreateParticipantWatcher::<Impl, IMPL_OFFSET>,
            SendInvitationAsync: SendInvitationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionAddedEventArgsImpl: Sized {
    fn SessionInfo(&mut self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionAddedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionAddedEventArgs, BASE_OFFSET>(),
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionControllerImpl: Sized {
    fn JoinRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveJoinRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoveParticipantAsync(&mut self, pparticipant: &::core::option::Option<RemoteSystemSessionParticipant>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn CreateSessionAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionController";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionControllerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionController, BASE_OFFSET>(),
            JoinRequested: JoinRequested::<Impl, IMPL_OFFSET>,
            RemoveJoinRequested: RemoveJoinRequested::<Impl, IMPL_OFFSET>,
            RemoveParticipantAsync: RemoveParticipantAsync::<Impl, IMPL_OFFSET>,
            CreateSessionAsync: CreateSessionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionControllerFactoryImpl: Sized {
    fn CreateController(&mut self, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionController>;
    fn CreateControllerWithSessionOptions(&mut self, displayname: &::windows::core::HSTRING, options: &::core::option::Option<RemoteSystemSessionOptions>) -> ::windows::core::Result<RemoteSystemSessionController>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionControllerFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionControllerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionControllerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionControllerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionControllerFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionControllerFactory, BASE_OFFSET>(),
            CreateController: CreateController::<Impl, IMPL_OFFSET>,
            CreateControllerWithSessionOptions: CreateControllerWithSessionOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionControllerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionCreationResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<RemoteSystemSessionCreationStatus>;
    fn Session(&mut self) -> ::windows::core::Result<RemoteSystemSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionCreationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionCreationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionCreationResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionCreationResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionCreationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Session: Session::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionCreationResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionDisconnectedEventArgsImpl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<RemoteSystemSessionDisconnectedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionDisconnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionDisconnectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionDisconnectedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionDisconnectedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionDisconnectedEventArgs, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionDisconnectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionInfoImpl: Sized {
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControllerDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JoinAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInfo";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionInfoVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionInfo, BASE_OFFSET>(),
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            ControllerDisplayName: ControllerDisplayName::<Impl, IMPL_OFFSET>,
            JoinAsync: JoinAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationImpl: Sized {
    fn Sender(&mut self) -> ::windows::core::Result<RemoteSystem>;
    fn SessionInfo(&mut self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInvitation";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionInvitationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInvitationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionInvitationVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionInvitation, BASE_OFFSET>(),
            Sender: Sender::<Impl, IMPL_OFFSET>,
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionInvitation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionInvitationListenerImpl: Sized {
    fn InvitationReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvitationReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInvitationListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionInvitationListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInvitationListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionInvitationListenerVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionInvitationListener, BASE_OFFSET>(),
            InvitationReceived: InvitationReceived::<Impl, IMPL_OFFSET>,
            RemoveInvitationReceived: RemoveInvitationReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionInvitationListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationReceivedEventArgsImpl: Sized {
    fn Invitation(&mut self) -> ::windows::core::Result<RemoteSystemSessionInvitation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionInvitationReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionInvitationReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionInvitationReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionInvitationReceivedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionInvitationReceivedEventArgs, BASE_OFFSET>(),
            Invitation: Invitation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionInvitationReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinRequestImpl: Sized {
    fn Participant(&mut self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
    fn Accept(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionJoinRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionJoinRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionJoinRequestVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionJoinRequest, BASE_OFFSET>(),
            Participant: Participant::<Impl, IMPL_OFFSET>,
            Accept: Accept::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionJoinRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionJoinRequestedEventArgsImpl: Sized {
    fn JoinRequest(&mut self) -> ::windows::core::Result<RemoteSystemSessionJoinRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionJoinRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionJoinRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionJoinRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionJoinRequestedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionJoinRequestedEventArgs, BASE_OFFSET>(),
            JoinRequest: JoinRequest::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionJoinRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinResultImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<RemoteSystemSessionJoinStatus>;
    fn Session(&mut self) -> ::windows::core::Result<RemoteSystemSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionJoinResult";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionJoinResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionJoinResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionJoinResultVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionJoinResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            Session: Session::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionJoinResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionMessageChannelImpl: Sized {
    fn Session(&mut self) -> ::windows::core::Result<RemoteSystemSession>;
    fn BroadcastValueSetAsync(&mut self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SendValueSetAsync(&mut self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>, participant: &::core::option::Option<RemoteSystemSessionParticipant>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SendValueSetToParticipantsAsync(&mut self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>, participants: &::core::option::Option<super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ValueSetReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueSetReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannel";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemSessionMessageChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionMessageChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionMessageChannelVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionMessageChannel, BASE_OFFSET>(),
            Session: Session::<Impl, IMPL_OFFSET>,
            BroadcastValueSetAsync: BroadcastValueSetAsync::<Impl, IMPL_OFFSET>,
            SendValueSetAsync: SendValueSetAsync::<Impl, IMPL_OFFSET>,
            SendValueSetToParticipantsAsync: SendValueSetToParticipantsAsync::<Impl, IMPL_OFFSET>,
            ValueSetReceived: ValueSetReceived::<Impl, IMPL_OFFSET>,
            RemoveValueSetReceived: RemoveValueSetReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionMessageChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionMessageChannelFactoryImpl: Sized {
    fn Create(&mut self, session: &::core::option::Option<RemoteSystemSession>, channelname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionMessageChannel>;
    fn CreateWithReliability(&mut self, session: &::core::option::Option<RemoteSystemSession>, channelname: &::windows::core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability) -> ::windows::core::Result<RemoteSystemSessionMessageChannel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionMessageChannelFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionMessageChannelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionMessageChannelFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionMessageChannelFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionMessageChannelFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionMessageChannelFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
            CreateWithReliability: CreateWithReliability::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionMessageChannelFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionOptionsImpl: Sized {
    fn IsInviteOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsInviteOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionOptionsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionOptions, BASE_OFFSET>(),
            IsInviteOnly: IsInviteOnly::<Impl, IMPL_OFFSET>,
            SetIsInviteOnly: SetIsInviteOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionParticipantImpl: Sized {
    fn RemoteSystem(&mut self) -> ::windows::core::Result<RemoteSystem>;
    fn GetHostNames(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipant";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl IRemoteSystemSessionParticipantVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionParticipantVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionParticipant, BASE_OFFSET>(),
            RemoteSystem: RemoteSystem::<Impl, IMPL_OFFSET>,
            GetHostNames: GetHostNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionParticipant as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantAddedEventArgsImpl: Sized {
    fn Participant(&mut self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipantAddedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionParticipantAddedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantAddedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionParticipantAddedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionParticipantAddedEventArgs, BASE_OFFSET>(),
            Participant: Participant::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionParticipantAddedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantRemovedEventArgsImpl: Sized {
    fn Participant(&mut self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipantRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionParticipantRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionParticipantRemovedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionParticipantRemovedEventArgs, BASE_OFFSET>(),
            Participant: Participant::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionParticipantRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionParticipantWatcherImpl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcherStatus>;
    fn Added(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionParticipantWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionParticipantWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionParticipantWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionParticipantWatcherVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionParticipantWatcher, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionParticipantWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionRemovedEventArgsImpl: Sized {
    fn SessionInfo(&mut self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionRemovedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionRemovedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionRemovedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionRemovedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionRemovedEventArgs, BASE_OFFSET>(),
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionRemovedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionStaticsImpl: Sized {
    fn CreateWatcher(&mut self) -> ::windows::core::Result<RemoteSystemSessionWatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionStatics, BASE_OFFSET>(),
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionUpdatedEventArgsImpl: Sized {
    fn SessionInfo(&mut self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemSessionUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionUpdatedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionUpdatedEventArgs, BASE_OFFSET>(),
            SessionInfo: SessionInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionValueSetReceivedEventArgsImpl: Sized {
    fn Sender(&mut self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionValueSetReceivedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemSessionValueSetReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionValueSetReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionValueSetReceivedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionValueSetReceivedEventArgs, BASE_OFFSET>(),
            Sender: Sender::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionValueSetReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemSessionWatcherImpl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<RemoteSystemSessionWatcherStatus>;
    fn Added(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemSessionWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemSessionWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemSessionWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemSessionWatcherVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemSessionWatcher, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Added: Added::<Impl, IMPL_OFFSET>,
            RemoveAdded: RemoveAdded::<Impl, IMPL_OFFSET>,
            Updated: Updated::<Impl, IMPL_OFFSET>,
            RemoveUpdated: RemoveUpdated::<Impl, IMPL_OFFSET>,
            Removed: Removed::<Impl, IMPL_OFFSET>,
            RemoveRemoved: RemoveRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemSessionWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
pub trait IRemoteSystemStaticsImpl: Sized {
    fn FindByHostNameAsync(&mut self, hostname: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>>;
    fn CreateWatcher(&mut self) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn CreateWatcherWithFilters(&mut self, filters: &::core::option::Option<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn RequestAccessAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemStatics {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Networking", feature = "implement_exclusive"))]
impl IRemoteSystemStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemStaticsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemStatics, BASE_OFFSET>(),
            FindByHostNameAsync: FindByHostNameAsync::<Impl, IMPL_OFFSET>,
            CreateWatcher: CreateWatcher::<Impl, IMPL_OFFSET>,
            CreateWatcherWithFilters: CreateWatcherWithFilters::<Impl, IMPL_OFFSET>,
            RequestAccessAsync: RequestAccessAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatics2Impl: Sized {
    fn IsAuthorizationKindEnabled(&mut self, kind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatics2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemStatics2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemStatics2, BASE_OFFSET>(),
            IsAuthorizationKindEnabled: IsAuthorizationKindEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRemoteSystemStatics3Impl: Sized {
    fn CreateWatcherForUser(&mut self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn CreateWatcherWithFiltersForUser(&mut self, user: &::core::option::Option<super::User>, filters: &::core::option::Option<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>) -> ::windows::core::Result<RemoteSystemWatcher>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemStatics3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatics3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRemoteSystemStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemStatics3Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemStatics3, BASE_OFFSET>(),
            CreateWatcherForUser: CreateWatcherForUser::<Impl, IMPL_OFFSET>,
            CreateWatcherWithFiltersForUser: CreateWatcherWithFiltersForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatusTypeFilterImpl: Sized {
    fn RemoteSystemStatusType(&mut self) -> ::windows::core::Result<RemoteSystemStatusType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatusTypeFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatusTypeFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemStatusTypeFilterVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemStatusTypeFilter, BASE_OFFSET>(),
            RemoteSystemStatusType: RemoteSystemStatusType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemStatusTypeFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatusTypeFilterFactoryImpl: Sized {
    fn Create(&mut self, remotesystemstatustype: RemoteSystemStatusType) -> ::windows::core::Result<RemoteSystemStatusTypeFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemStatusTypeFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemStatusTypeFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemStatusTypeFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemStatusTypeFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemStatusTypeFilterFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemStatusTypeFilterFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemStatusTypeFilterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemUpdatedEventArgsImpl: Sized {
    fn RemoteSystem(&mut self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemUpdatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemUpdatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemUpdatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemUpdatedEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemUpdatedEventArgs, BASE_OFFSET>(),
            RemoteSystem: RemoteSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemUpdatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemWatcherImpl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn RemoteSystemAdded(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemAdded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoteSystemUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoteSystemRemoved(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemRemoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcher";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemWatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemWatcherVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemWatcher, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            RemoteSystemAdded: RemoteSystemAdded::<Impl, IMPL_OFFSET>,
            RemoveRemoteSystemAdded: RemoveRemoteSystemAdded::<Impl, IMPL_OFFSET>,
            RemoteSystemUpdated: RemoteSystemUpdated::<Impl, IMPL_OFFSET>,
            RemoveRemoteSystemUpdated: RemoveRemoteSystemUpdated::<Impl, IMPL_OFFSET>,
            RemoteSystemRemoved: RemoteSystemRemoved::<Impl, IMPL_OFFSET>,
            RemoveRemoteSystemRemoved: RemoveRemoteSystemRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemWatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteSystemWatcher2Impl: Sized {
    fn EnumerationCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemWatcher2 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcher2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteSystemWatcher2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcher2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemWatcher2Vtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemWatcher2, BASE_OFFSET>(),
            EnumerationCompleted: EnumerationCompleted::<Impl, IMPL_OFFSET>,
            RemoveEnumerationCompleted: RemoveEnumerationCompleted::<Impl, IMPL_OFFSET>,
            ErrorOccurred: ErrorOccurred::<Impl, IMPL_OFFSET>,
            RemoveErrorOccurred: RemoveErrorOccurred::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemWatcher2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcher3Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWatcher3 {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcher3";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWatcher3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcher3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemWatcher3Vtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemWatcher3, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemWatcher3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcherErrorOccurredEventArgsImpl: Sized {
    fn Error(&mut self) -> ::windows::core::Result<RemoteSystemWatcherError>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWatcherErrorOccurredEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteSystemWatcherErrorOccurredEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWatcherErrorOccurredEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemWatcherErrorOccurredEventArgsVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemWatcherErrorOccurredEventArgs, BASE_OFFSET>(),
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemWatcherErrorOccurredEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IRemoteSystemWebAccountFilterImpl: Sized {
    fn Account(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWebAccountFilter";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IRemoteSystemWebAccountFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWebAccountFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemWebAccountFilterVtbl {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemWebAccountFilter, BASE_OFFSET>(), Account: Account::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemWebAccountFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IRemoteSystemWebAccountFilterFactoryImpl: Sized {
    fn Create(&mut self, account: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<RemoteSystemWebAccountFilter>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteSystemWebAccountFilterFactory {
    const NAME: &'static str = "Windows.System.RemoteSystems.IRemoteSystemWebAccountFilterFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IRemoteSystemWebAccountFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteSystemWebAccountFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteSystemWebAccountFilterFactoryVtbl {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteSystemWebAccountFilterFactory, BASE_OFFSET>(),
            Create: Create::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteSystemWebAccountFilterFactory as ::windows::core::Interface>::IID
    }
}
