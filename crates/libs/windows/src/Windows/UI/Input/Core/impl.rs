#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IRadialControllerIndependentInputSource_Impl: Sized {
    fn Controller(&mut self) -> ::windows::core::Result<super::RadialController>;
    fn Dispatcher(&mut self) -> ::windows::core::Result<super::super::Core::CoreDispatcher>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.IRadialControllerIndependentInputSource";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl IRadialControllerIndependentInputSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerIndependentInputSource_Vtbl {
        unsafe extern "system" fn Controller<Impl: IRadialControllerIndependentInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: IRadialControllerIndependentInputSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Dispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerIndependentInputSource, BASE_OFFSET>(),
            Controller: Controller::<Impl, IMPL_OFFSET>,
            Dispatcher: Dispatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IRadialControllerIndependentInputSource2_Impl: Sized {
    fn DispatcherQueue(&mut self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSource2 {
    const NAME: &'static str = "Windows.UI.Input.Core.IRadialControllerIndependentInputSource2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IRadialControllerIndependentInputSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSource2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerIndependentInputSource2_Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: IRadialControllerIndependentInputSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerIndependentInputSource2, BASE_OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "implement_exclusive"))]
pub trait IRadialControllerIndependentInputSourceStatics_Impl: Sized {
    fn CreateForView(&mut self, view: &::core::option::Option<super::super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<RadialControllerIndependentInputSource>;
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSourceStatics {
    const NAME: &'static str = "Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics";
}
#[cfg(all(feature = "ApplicationModel_Core", feature = "implement_exclusive"))]
impl IRadialControllerIndependentInputSourceStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadialControllerIndependentInputSourceStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRadialControllerIndependentInputSourceStatics_Vtbl {
        unsafe extern "system" fn CreateForView<Impl: IRadialControllerIndependentInputSourceStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateForView(&*(&view as *const <super::super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::Abi>::Abi as *const <super::super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRadialControllerIndependentInputSourceStatics, BASE_OFFSET>(),
            CreateForView: CreateForView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRadialControllerIndependentInputSourceStatics as ::windows::core::Interface>::IID
    }
}
