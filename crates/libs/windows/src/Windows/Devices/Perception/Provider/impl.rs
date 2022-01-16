#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
pub trait IPerceptionFrameProvider_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn FrameProviderInfo(&mut self) -> ::windows::core::Result<PerceptionFrameProviderInfo>;
    fn Available(&mut self) -> ::windows::core::Result<bool>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, value: &::core::option::Option<PerceptionPropertyChangeRequest>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProvider {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProvider";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "deprecated"))]
impl IPerceptionFrameProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameProvider_Vtbl {
        unsafe extern "system" fn FrameProviderInfo<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FrameProviderInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Available<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Available() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Properties<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn SetProperty<Impl: IPerceptionFrameProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(&*(&value as *const <PerceptionPropertyChangeRequest as ::windows::core::Abi>::Abi as *const <PerceptionPropertyChangeRequest as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProvider, BASE_OFFSET>(),
            FrameProviderInfo: FrameProviderInfo::<Impl, IMPL_OFFSET>,
            Available: Available::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderManager_Impl: Sized + super::super::super::Foundation::IClosable_Impl {
    fn GetFrameProvider(&mut self, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<IPerceptionFrameProvider>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for IPerceptionFrameProviderManager {
    const NAME: &'static str = "Windows.Devices.Perception.Provider.IPerceptionFrameProviderManager";
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
impl IPerceptionFrameProviderManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerceptionFrameProviderManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerceptionFrameProviderManager_Vtbl {
        unsafe extern "system" fn GetFrameProvider<Impl: IPerceptionFrameProviderManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameproviderinfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameProvider(&*(&frameproviderinfo as *const <PerceptionFrameProviderInfo as ::windows::core::Abi>::Abi as *const <PerceptionFrameProviderInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPerceptionFrameProviderManager, BASE_OFFSET>(),
            GetFrameProvider: GetFrameProvider::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerceptionFrameProviderManager as ::windows::core::Interface>::IID
    }
}
