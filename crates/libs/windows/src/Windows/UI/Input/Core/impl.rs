#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerIndependentInputSourceImpl: Sized {
    fn Controller(&self) -> ::windows::core::Result<super::RadialController>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::super::Core::CoreDispatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.IRadialControllerIndependentInputSource";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerIndependentInputSourceVtbl {
    pub const fn new<Impl: IRadialControllerIndependentInputSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerIndependentInputSourceVtbl {
        unsafe extern "system" fn Controller<Impl: IRadialControllerIndependentInputSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Controller() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: IRadialControllerIndependentInputSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Dispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerIndependentInputSource>, base.5, Controller::<Impl, OFFSET>, Dispatcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerIndependentInputSource2Impl: Sized {
    fn DispatcherQueue(&self) -> ::windows::core::Result<super::super::super::System::DispatcherQueue>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSource2 {
    const NAME: &'static str = "Windows.UI.Input.Core.IRadialControllerIndependentInputSource2";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerIndependentInputSource2Vtbl {
    pub const fn new<Impl: IRadialControllerIndependentInputSource2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerIndependentInputSource2Vtbl {
        unsafe extern "system" fn DispatcherQueue<Impl: IRadialControllerIndependentInputSource2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DispatcherQueue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerIndependentInputSource2>, base.5, DispatcherQueue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadialControllerIndependentInputSourceStaticsImpl: Sized {
    fn CreateForView(&self, view: &::core::option::Option<super::super::super::ApplicationModel::Core::CoreApplicationView>) -> ::windows::core::Result<RadialControllerIndependentInputSource>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadialControllerIndependentInputSourceStatics {
    const NAME: &'static str = "Windows.UI.Input.Core.IRadialControllerIndependentInputSourceStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRadialControllerIndependentInputSourceStaticsVtbl {
    pub const fn new<Impl: IRadialControllerIndependentInputSourceStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadialControllerIndependentInputSourceStaticsVtbl {
        unsafe extern "system" fn CreateForView<Impl: IRadialControllerIndependentInputSourceStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForView(&*(&view as *const <super::super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::Abi>::Abi as *const <super::super::super::ApplicationModel::Core::CoreApplicationView as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadialControllerIndependentInputSourceStatics>, base.5, CreateForView::<Impl, OFFSET>)
    }
}
