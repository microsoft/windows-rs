#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
pub trait IPerceptionFrameProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn FrameProviderInfo(&self) -> ::windows::core::Result<PerceptionFrameProviderInfo>;
    fn Available(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn SetProperty(&self, value: &::core::option::Option<PerceptionPropertyChangeRequest>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProvider {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
}
#[cfg(all(feature = "Foundation_Collections", feature = "deprecated"))]
impl IPerceptionFrameProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>() -> IPerceptionFrameProvider_Vtbl {
        unsafe extern "system" fn FrameProviderInfo<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FrameProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProvider, OFFSET>(),
            FrameProviderInfo: FrameProviderInfo::<Identity, Impl, OFFSET>,
            Available: Available::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderManager_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn GetFrameProvider(&self, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<IPerceptionFrameProvider>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderManager {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl IPerceptionFrameProviderManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProviderManager_Impl, const OFFSET: isize>() -> IPerceptionFrameProviderManager_Vtbl {
        unsafe extern "system" fn GetFrameProvider<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProviderManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameProvider(::core::mem::transmute(&frameproviderinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProviderManager, OFFSET>(),
            GetFrameProvider: GetFrameProvider::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProviderManager as ::windows::core::Interface>::IID
    }
}
