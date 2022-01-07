#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerImpl: Sized {
    fn MinWindowWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinWindowWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinWindowHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMinWindowHeight(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.IAdaptiveTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveTriggerVtbl {
    pub const fn new<Impl: IAdaptiveTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveTriggerVtbl {
        unsafe extern "system" fn MinWindowWidth<Impl: IAdaptiveTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinWindowWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinWindowWidth<Impl: IAdaptiveTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinWindowWidth(value).into()
        }
        unsafe extern "system" fn MinWindowHeight<Impl: IAdaptiveTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinWindowHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinWindowHeight<Impl: IAdaptiveTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinWindowHeight(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveTrigger>, base.5, MinWindowWidth::<Impl, OFFSET>, SetMinWindowWidth::<Impl, OFFSET>, MinWindowHeight::<Impl, OFFSET>, SetMinWindowHeight::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AdaptiveTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveTriggerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IAdaptiveTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveTriggerFactoryVtbl {
    pub const fn new<Impl: IAdaptiveTriggerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveTriggerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAdaptiveTriggerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveTriggerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerStaticsImpl: Sized {
    fn MinWindowWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MinWindowHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveTriggerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IAdaptiveTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveTriggerStaticsVtbl {
    pub const fn new<Impl: IAdaptiveTriggerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAdaptiveTriggerStaticsVtbl {
        unsafe extern "system" fn MinWindowWidthProperty<Impl: IAdaptiveTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinWindowWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinWindowHeightProperty<Impl: IAdaptiveTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinWindowHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAdaptiveTriggerStatics>, base.5, MinWindowWidthProperty::<Impl, OFFSET>, MinWindowHeightProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationImpl: Sized {
    fn Resources(&self) -> ::windows::core::Result<ResourceDictionary>;
    fn SetResources(&self, value: &::core::option::Option<ResourceDictionary>) -> ::windows::core::Result<()>;
    fn DebugSettings(&self) -> ::windows::core::Result<DebugSettings>;
    fn RequestedTheme(&self) -> ::windows::core::Result<ApplicationTheme>;
    fn SetRequestedTheme(&self, value: ApplicationTheme) -> ::windows::core::Result<()>;
    fn UnhandledException(&self, handler: &::core::option::Option<UnhandledExceptionEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledException(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Suspending(&self, handler: &::core::option::Option<SuspendingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Exit(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplication {
    const NAME: &'static str = "Windows.UI.Xaml.IApplication";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationVtbl {
    pub const fn new<Impl: IApplicationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationVtbl {
        unsafe extern "system" fn Resources<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResources<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResources(&*(&value as *const <ResourceDictionary as ::windows::core::Abi>::Abi as *const <ResourceDictionary as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DebugSettings<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DebugSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedTheme<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedTheme<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequestedTheme(value).into()
        }
        unsafe extern "system" fn UnhandledException<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnhandledException(&*(&handler as *const <UnhandledExceptionEventHandler as ::windows::core::Abi>::Abi as *const <UnhandledExceptionEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnhandledException<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUnhandledException(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Suspending<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Suspending(&*(&handler as *const <SuspendingEventHandler as ::windows::core::Abi>::Abi as *const <SuspendingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspending<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSuspending(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Resuming<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resuming(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResuming<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveResuming(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Exit<Impl: IApplicationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Exit().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplication>, base.5, Resources::<Impl, OFFSET>, SetResources::<Impl, OFFSET>, DebugSettings::<Impl, OFFSET>, RequestedTheme::<Impl, OFFSET>, SetRequestedTheme::<Impl, OFFSET>, UnhandledException::<Impl, OFFSET>, RemoveUnhandledException::<Impl, OFFSET>, Suspending::<Impl, OFFSET>, RemoveSuspending::<Impl, OFFSET>, Resuming::<Impl, OFFSET>, RemoveResuming::<Impl, OFFSET>, Exit::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplication2Impl: Sized {
    fn FocusVisualKind(&self) -> ::windows::core::Result<FocusVisualKind>;
    fn SetFocusVisualKind(&self, value: FocusVisualKind) -> ::windows::core::Result<()>;
    fn RequiresPointerMode(&self) -> ::windows::core::Result<ApplicationRequiresPointerMode>;
    fn SetRequiresPointerMode(&self, value: ApplicationRequiresPointerMode) -> ::windows::core::Result<()>;
    fn LeavingBackground(&self, handler: &::core::option::Option<LeavingBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&self, handler: &::core::option::Option<EnteredBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplication2 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplication2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplication2Vtbl {
    pub const fn new<Impl: IApplication2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplication2Vtbl {
        unsafe extern "system" fn FocusVisualKind<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusVisualKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualKind<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FocusVisualKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusVisualKind(value).into()
        }
        unsafe extern "system" fn RequiresPointerMode<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationRequiresPointerMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequiresPointerMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequiresPointerMode<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ApplicationRequiresPointerMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequiresPointerMode(value).into()
        }
        unsafe extern "system" fn LeavingBackground<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LeavingBackground(&*(&handler as *const <LeavingBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <LeavingBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLeavingBackground<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLeavingBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnteredBackground<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnteredBackground(&*(&handler as *const <EnteredBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <EnteredBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnteredBackground<Impl: IApplication2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEnteredBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplication2>, base.5, FocusVisualKind::<Impl, OFFSET>, SetFocusVisualKind::<Impl, OFFSET>, RequiresPointerMode::<Impl, OFFSET>, SetRequiresPointerMode::<Impl, OFFSET>, LeavingBackground::<Impl, OFFSET>, RemoveLeavingBackground::<Impl, OFFSET>, EnteredBackground::<Impl, OFFSET>, RemoveEnteredBackground::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplication3Impl: Sized {
    fn HighContrastAdjustment(&self) -> ::windows::core::Result<ApplicationHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&self, value: ApplicationHighContrastAdjustment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplication3 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplication3";
}
#[cfg(feature = "implement_exclusive")]
impl IApplication3Vtbl {
    pub const fn new<Impl: IApplication3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplication3Vtbl {
        unsafe extern "system" fn HighContrastAdjustment<Impl: IApplication3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighContrastAdjustment<Impl: IApplication3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ApplicationHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHighContrastAdjustment(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplication3>, base.5, HighContrastAdjustment::<Impl, OFFSET>, SetHighContrastAdjustment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Application>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationFactoryVtbl {
    pub const fn new<Impl: IApplicationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IApplicationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationInitializationCallbackParamsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationInitializationCallbackParams {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationInitializationCallbackParams";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationInitializationCallbackParamsVtbl {
    pub const fn new<Impl: IApplicationInitializationCallbackParamsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationInitializationCallbackParamsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationInitializationCallbackParams>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationOverridesImpl: Sized {
    fn OnActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnLaunched(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::LaunchActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnSearchActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::SearchActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnShareTargetActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileOpenPickerActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileSavePickerActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnCachedFileUpdaterActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnWindowCreated(&self, args: &::core::option::Option<WindowCreatedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationOverridesVtbl {
    pub const fn new<Impl: IApplicationOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationOverridesVtbl {
        unsafe extern "system" fn OnActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnActivated(&*(&args as *const <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLaunched<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnLaunched(&*(&args as *const <super::super::ApplicationModel::Activation::LaunchActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::LaunchActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnFileActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnSearchActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnSearchActivated(&*(&args as *const <super::super::ApplicationModel::Activation::SearchActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::SearchActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnShareTargetActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnShareTargetActivated(&*(&args as *const <super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileOpenPickerActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnFileOpenPickerActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileSavePickerActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnFileSavePickerActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnCachedFileUpdaterActivated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnCachedFileUpdaterActivated(&*(&args as *const <super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnWindowCreated<Impl: IApplicationOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnWindowCreated(&*(&args as *const <WindowCreatedEventArgs as ::windows::core::Abi>::Abi as *const <WindowCreatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationOverrides>, base.5, OnActivated::<Impl, OFFSET>, OnLaunched::<Impl, OFFSET>, OnFileActivated::<Impl, OFFSET>, OnSearchActivated::<Impl, OFFSET>, OnShareTargetActivated::<Impl, OFFSET>, OnFileOpenPickerActivated::<Impl, OFFSET>, OnFileSavePickerActivated::<Impl, OFFSET>, OnCachedFileUpdaterActivated::<Impl, OFFSET>, OnWindowCreated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationOverrides2Impl: Sized {
    fn OnBackgroundActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationOverrides2Vtbl {
    pub const fn new<Impl: IApplicationOverrides2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationOverrides2Vtbl {
        unsafe extern "system" fn OnBackgroundActivated<Impl: IApplicationOverrides2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnBackgroundActivated(&*(&args as *const <super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationOverrides2>, base.5, OnBackgroundActivated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<Application>;
    fn Start(&self, callback: &::core::option::Option<ApplicationInitializationCallback>) -> ::windows::core::Result<()>;
    fn LoadComponent(&self, component: &::core::option::Option<::windows::core::IInspectable>, resourcelocator: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LoadComponentWithResourceLocation(&self, component: &::core::option::Option<::windows::core::IInspectable>, resourcelocator: &::core::option::Option<super::super::Foundation::Uri>, componentresourcelocation: Controls::Primitives::ComponentResourceLocation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationStaticsVtbl {
    pub const fn new<Impl: IApplicationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApplicationStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IApplicationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IApplicationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start(&*(&callback as *const <ApplicationInitializationCallback as ::windows::core::Abi>::Abi as *const <ApplicationInitializationCallback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadComponent<Impl: IApplicationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, component: *mut ::core::ffi::c_void, resourcelocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadComponent(&*(&component as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&resourcelocator as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadComponentWithResourceLocation<Impl: IApplicationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, component: *mut ::core::ffi::c_void, resourcelocator: ::windows::core::RawPtr, componentresourcelocation: Controls::Primitives::ComponentResourceLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LoadComponentWithResourceLocation(&*(&component as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&resourcelocator as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), componentresourcelocation).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApplicationStatics>, base.5, Current::<Impl, OFFSET>, Start::<Impl, OFFSET>, LoadComponent::<Impl, OFFSET>, LoadComponentWithResourceLocation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingFailedEventArgsImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IBindingFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingFailedEventArgsVtbl {
    pub const fn new<Impl: IBindingFailedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBindingFailedEventArgsVtbl {
        unsafe extern "system" fn Message<Impl: IBindingFailedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBindingFailedEventArgs>, base.5, Message::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewOptionsImpl: Sized {
    fn AnimationDesired(&self) -> ::windows::core::Result<bool>;
    fn SetAnimationDesired(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetRect(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
    fn SetTargetRect(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBringIntoViewOptions {
    const NAME: &'static str = "Windows.UI.Xaml.IBringIntoViewOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IBringIntoViewOptionsVtbl {
    pub const fn new<Impl: IBringIntoViewOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBringIntoViewOptionsVtbl {
        unsafe extern "system" fn AnimationDesired<Impl: IBringIntoViewOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnimationDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationDesired<Impl: IBringIntoViewOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAnimationDesired(value).into()
        }
        unsafe extern "system" fn TargetRect<Impl: IBringIntoViewOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetRect<Impl: IBringIntoViewOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetRect(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBringIntoViewOptions>, base.5, AnimationDesired::<Impl, OFFSET>, SetAnimationDesired::<Impl, OFFSET>, TargetRect::<Impl, OFFSET>, SetTargetRect::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewOptions2Impl: Sized {
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalAlignmentRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalAlignmentRatio(&self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBringIntoViewOptions2 {
    const NAME: &'static str = "Windows.UI.Xaml.IBringIntoViewOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IBringIntoViewOptions2Vtbl {
    pub const fn new<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBringIntoViewOptions2Vtbl {
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignmentRatio<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignmentRatio<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IBringIntoViewOptions2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBringIntoViewOptions2>, base.5, HorizontalAlignmentRatio::<Impl, OFFSET>, SetHorizontalAlignmentRatio::<Impl, OFFSET>, VerticalAlignmentRatio::<Impl, OFFSET>, SetVerticalAlignmentRatio::<Impl, OFFSET>, HorizontalOffset::<Impl, OFFSET>, SetHorizontalOffset::<Impl, OFFSET>, VerticalOffset::<Impl, OFFSET>, SetVerticalOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewRequestedEventArgsImpl: Sized {
    fn TargetElement(&self) -> ::windows::core::Result<UIElement>;
    fn SetTargetElement(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn AnimationDesired(&self) -> ::windows::core::Result<bool>;
    fn SetAnimationDesired(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetRect(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetTargetRect(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn HorizontalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn VerticalAlignmentRatio(&self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBringIntoViewRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IBringIntoViewRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBringIntoViewRequestedEventArgsVtbl {
    pub const fn new<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBringIntoViewRequestedEventArgsVtbl {
        unsafe extern "system" fn TargetElement<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetElement<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetElement(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AnimationDesired<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AnimationDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationDesired<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAnimationDesired(value).into()
        }
        unsafe extern "system" fn TargetRect<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetRect<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetRect(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        unsafe extern "system" fn Handled<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IBringIntoViewRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IBringIntoViewRequestedEventArgs>,
            base.5,
            TargetElement::<Impl, OFFSET>,
            SetTargetElement::<Impl, OFFSET>,
            AnimationDesired::<Impl, OFFSET>,
            SetAnimationDesired::<Impl, OFFSET>,
            TargetRect::<Impl, OFFSET>,
            SetTargetRect::<Impl, OFFSET>,
            HorizontalAlignmentRatio::<Impl, OFFSET>,
            VerticalAlignmentRatio::<Impl, OFFSET>,
            HorizontalOffset::<Impl, OFFSET>,
            SetHorizontalOffset::<Impl, OFFSET>,
            VerticalOffset::<Impl, OFFSET>,
            SetVerticalOffset::<Impl, OFFSET>,
            Handled::<Impl, OFFSET>,
            SetHandled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushTransitionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushTransition {
    const NAME: &'static str = "Windows.UI.Xaml.IBrushTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushTransitionVtbl {
    pub const fn new<Impl: IBrushTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrushTransitionVtbl {
        unsafe extern "system" fn Duration<Impl: IBrushTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IBrushTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrushTransition>, base.5, Duration::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushTransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BrushTransition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushTransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IBrushTransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushTransitionFactoryVtbl {
    pub const fn new<Impl: IBrushTransitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBrushTransitionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBrushTransitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBrushTransitionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPaletteResourcesImpl: Sized {
    fn AltHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMediumHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMediumHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMediumHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMediumHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeAltLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeAltLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeDisabledHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeDisabledHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeDisabledLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeDisabledLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeHigh(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeHigh(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeMediumLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeMediumLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeWhite(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeWhite(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeGray(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeGray(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ListLow(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetListLow(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ListMedium(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetListMedium(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ErrorText(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetErrorText(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn Accent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAccent(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPaletteResources {
    const NAME: &'static str = "Windows.UI.Xaml.IColorPaletteResources";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPaletteResourcesVtbl {
    pub const fn new<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPaletteResourcesVtbl {
        unsafe extern "system" fn AltHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAltHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAltLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAltMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltMediumHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltMediumHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltMediumHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAltMediumHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AltMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAltMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBaseHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBaseLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBaseMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseMediumHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseMediumHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMediumHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBaseMediumHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBaseMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeAltLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeAltLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeAltLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeAltLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeBlackHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeBlackHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeBlackLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeBlackLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeBlackMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeBlackMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeBlackMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeBlackMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeDisabledHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeDisabledHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeDisabledHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeDisabledHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeDisabledLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeDisabledLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeDisabledLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeDisabledLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeHigh<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeMediumLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeWhite<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeWhite<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeWhite(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeGray<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChromeGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeGray<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChromeGray(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ListLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLow<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ListMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListMedium<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorText<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorText<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetErrorText(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Accent<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Accent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccent<Impl: IColorPaletteResourcesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAccent(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IColorPaletteResources>,
            base.5,
            AltHigh::<Impl, OFFSET>,
            SetAltHigh::<Impl, OFFSET>,
            AltLow::<Impl, OFFSET>,
            SetAltLow::<Impl, OFFSET>,
            AltMedium::<Impl, OFFSET>,
            SetAltMedium::<Impl, OFFSET>,
            AltMediumHigh::<Impl, OFFSET>,
            SetAltMediumHigh::<Impl, OFFSET>,
            AltMediumLow::<Impl, OFFSET>,
            SetAltMediumLow::<Impl, OFFSET>,
            BaseHigh::<Impl, OFFSET>,
            SetBaseHigh::<Impl, OFFSET>,
            BaseLow::<Impl, OFFSET>,
            SetBaseLow::<Impl, OFFSET>,
            BaseMedium::<Impl, OFFSET>,
            SetBaseMedium::<Impl, OFFSET>,
            BaseMediumHigh::<Impl, OFFSET>,
            SetBaseMediumHigh::<Impl, OFFSET>,
            BaseMediumLow::<Impl, OFFSET>,
            SetBaseMediumLow::<Impl, OFFSET>,
            ChromeAltLow::<Impl, OFFSET>,
            SetChromeAltLow::<Impl, OFFSET>,
            ChromeBlackHigh::<Impl, OFFSET>,
            SetChromeBlackHigh::<Impl, OFFSET>,
            ChromeBlackLow::<Impl, OFFSET>,
            SetChromeBlackLow::<Impl, OFFSET>,
            ChromeBlackMediumLow::<Impl, OFFSET>,
            SetChromeBlackMediumLow::<Impl, OFFSET>,
            ChromeBlackMedium::<Impl, OFFSET>,
            SetChromeBlackMedium::<Impl, OFFSET>,
            ChromeDisabledHigh::<Impl, OFFSET>,
            SetChromeDisabledHigh::<Impl, OFFSET>,
            ChromeDisabledLow::<Impl, OFFSET>,
            SetChromeDisabledLow::<Impl, OFFSET>,
            ChromeHigh::<Impl, OFFSET>,
            SetChromeHigh::<Impl, OFFSET>,
            ChromeLow::<Impl, OFFSET>,
            SetChromeLow::<Impl, OFFSET>,
            ChromeMedium::<Impl, OFFSET>,
            SetChromeMedium::<Impl, OFFSET>,
            ChromeMediumLow::<Impl, OFFSET>,
            SetChromeMediumLow::<Impl, OFFSET>,
            ChromeWhite::<Impl, OFFSET>,
            SetChromeWhite::<Impl, OFFSET>,
            ChromeGray::<Impl, OFFSET>,
            SetChromeGray::<Impl, OFFSET>,
            ListLow::<Impl, OFFSET>,
            SetListLow::<Impl, OFFSET>,
            ListMedium::<Impl, OFFSET>,
            SetListMedium::<Impl, OFFSET>,
            ErrorText::<Impl, OFFSET>,
            SetErrorText::<Impl, OFFSET>,
            Accent::<Impl, OFFSET>,
            SetAccent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPaletteResourcesFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPaletteResources>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPaletteResourcesFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IColorPaletteResourcesFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPaletteResourcesFactoryVtbl {
    pub const fn new<Impl: IColorPaletteResourcesFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPaletteResourcesFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorPaletteResourcesFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorPaletteResourcesFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICornerRadiusHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICornerRadiusHelper {
    const NAME: &'static str = "Windows.UI.Xaml.ICornerRadiusHelper";
}
#[cfg(feature = "implement_exclusive")]
impl ICornerRadiusHelperVtbl {
    pub const fn new<Impl: ICornerRadiusHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICornerRadiusHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICornerRadiusHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICornerRadiusHelperStaticsImpl: Sized {
    fn FromRadii(&self, topleft: f64, topright: f64, bottomright: f64, bottomleft: f64) -> ::windows::core::Result<CornerRadius>;
    fn FromUniformRadius(&self, uniformradius: f64) -> ::windows::core::Result<CornerRadius>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICornerRadiusHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.ICornerRadiusHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICornerRadiusHelperStaticsVtbl {
    pub const fn new<Impl: ICornerRadiusHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICornerRadiusHelperStaticsVtbl {
        unsafe extern "system" fn FromRadii<Impl: ICornerRadiusHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, topleft: f64, topright: f64, bottomright: f64, bottomleft: f64, result__: *mut CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromRadii(topleft, topright, bottomright, bottomleft) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromUniformRadius<Impl: ICornerRadiusHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uniformradius: f64, result__: *mut CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromUniformRadius(uniformradius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICornerRadiusHelperStatics>, base.5, FromRadii::<Impl, OFFSET>, FromUniformRadius::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataContextChangedEventArgsImpl: Sized {
    fn NewValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataContextChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDataContextChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDataContextChangedEventArgsVtbl {
    pub const fn new<Impl: IDataContextChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataContextChangedEventArgsVtbl {
        unsafe extern "system" fn NewValue<Impl: IDataContextChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IDataContextChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDataContextChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataContextChangedEventArgs>, base.5, NewValue::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateImpl: Sized {
    fn LoadContent(&self) -> ::windows::core::Result<DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplate";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateVtbl {
    pub const fn new<Impl: IDataTemplateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataTemplateVtbl {
        unsafe extern "system" fn LoadContent<Impl: IDataTemplateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LoadContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataTemplate>, base.5, LoadContent::<Impl, OFFSET>)
    }
}
pub trait IDataTemplateExtensionImpl: Sized {
    fn ResetTemplate(&self) -> ::windows::core::Result<()>;
    fn ProcessBinding(&self, phase: u32) -> ::windows::core::Result<bool>;
    fn ProcessBindings(&self, arg: &::core::option::Option<Controls::ContainerContentChangingEventArgs>) -> ::windows::core::Result<i32>;
}
impl ::windows::core::RuntimeName for IDataTemplateExtension {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateExtension";
}
impl IDataTemplateExtensionVtbl {
    pub const fn new<Impl: IDataTemplateExtensionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataTemplateExtensionVtbl {
        unsafe extern "system" fn ResetTemplate<Impl: IDataTemplateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ResetTemplate().into()
        }
        unsafe extern "system" fn ProcessBinding<Impl: IDataTemplateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phase: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessBinding(phase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessBindings<Impl: IDataTemplateExtensionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, arg: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessBindings(&*(&arg as *const <Controls::ContainerContentChangingEventArgs as ::windows::core::Abi>::Abi as *const <Controls::ContainerContentChangingEventArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataTemplateExtension>, base.5, ResetTemplate::<Impl, OFFSET>, ProcessBinding::<Impl, OFFSET>, ProcessBindings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateFactoryVtbl {
    pub const fn new<Impl: IDataTemplateFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataTemplateFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDataTemplateFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataTemplateFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateKeyImpl: Sized {
    fn DataType(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDataType(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateKey {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateKey";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateKeyVtbl {
    pub const fn new<Impl: IDataTemplateKeyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataTemplateKeyVtbl {
        unsafe extern "system" fn DataType<Impl: IDataTemplateKeyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataType<Impl: IDataTemplateKeyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDataType(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataTemplateKey>, base.5, DataType::<Impl, OFFSET>, SetDataType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateKeyFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateKey>;
    fn CreateInstanceWithType(&self, datatype: &::core::option::Option<::windows::core::IInspectable>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateKey>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateKeyFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateKeyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateKeyFactoryVtbl {
    pub const fn new<Impl: IDataTemplateKeyFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataTemplateKeyFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDataTemplateKeyFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithType<Impl: IDataTemplateKeyFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatype: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithType(&*(&datatype as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataTemplateKeyFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateInstanceWithType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateStatics2Impl: Sized {
    fn ExtensionInstanceProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn GetExtensionInstance(&self, element: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<IDataTemplateExtension>;
    fn SetExtensionInstance(&self, element: &::core::option::Option<FrameworkElement>, value: &::core::option::Option<IDataTemplateExtension>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateStatics2Vtbl {
    pub const fn new<Impl: IDataTemplateStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDataTemplateStatics2Vtbl {
        unsafe extern "system" fn ExtensionInstanceProperty<Impl: IDataTemplateStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtensionInstanceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtensionInstance<Impl: IDataTemplateStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetExtensionInstance(&*(&element as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionInstance<Impl: IDataTemplateStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExtensionInstance(&*(&element as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IDataTemplateExtension as ::windows::core::Abi>::Abi as *const <IDataTemplateExtension as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDataTemplateStatics2>, base.5, ExtensionInstanceProperty::<Impl, OFFSET>, GetExtensionInstance::<Impl, OFFSET>, SetExtensionInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettingsImpl: Sized {
    fn EnableFrameRateCounter(&self) -> ::windows::core::Result<bool>;
    fn SetEnableFrameRateCounter(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsBindingTracingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBindingTracingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOverdrawHeatMapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOverdrawHeatMapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn BindingFailed(&self, handler: &::core::option::Option<BindingFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBindingFailed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettingsVtbl {
    pub const fn new<Impl: IDebugSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDebugSettingsVtbl {
        unsafe extern "system" fn EnableFrameRateCounter<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableFrameRateCounter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFrameRateCounter<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableFrameRateCounter(value).into()
        }
        unsafe extern "system" fn IsBindingTracingEnabled<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsBindingTracingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBindingTracingEnabled<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsBindingTracingEnabled(value).into()
        }
        unsafe extern "system" fn IsOverdrawHeatMapEnabled<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOverdrawHeatMapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverdrawHeatMapEnabled<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsOverdrawHeatMapEnabled(value).into()
        }
        unsafe extern "system" fn BindingFailed<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BindingFailed(&*(&handler as *const <BindingFailedEventHandler as ::windows::core::Abi>::Abi as *const <BindingFailedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBindingFailed<Impl: IDebugSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBindingFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDebugSettings>, base.5, EnableFrameRateCounter::<Impl, OFFSET>, SetEnableFrameRateCounter::<Impl, OFFSET>, IsBindingTracingEnabled::<Impl, OFFSET>, SetIsBindingTracingEnabled::<Impl, OFFSET>, IsOverdrawHeatMapEnabled::<Impl, OFFSET>, SetIsOverdrawHeatMapEnabled::<Impl, OFFSET>, BindingFailed::<Impl, OFFSET>, RemoveBindingFailed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings2Impl: Sized {
    fn EnableRedrawRegions(&self) -> ::windows::core::Result<bool>;
    fn SetEnableRedrawRegions(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettings2Vtbl {
    pub const fn new<Impl: IDebugSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDebugSettings2Vtbl {
        unsafe extern "system" fn EnableRedrawRegions<Impl: IDebugSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnableRedrawRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableRedrawRegions<Impl: IDebugSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnableRedrawRegions(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDebugSettings2>, base.5, EnableRedrawRegions::<Impl, OFFSET>, SetEnableRedrawRegions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings3Impl: Sized {
    fn IsTextPerformanceVisualizationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTextPerformanceVisualizationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings3 {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettings3Vtbl {
    pub const fn new<Impl: IDebugSettings3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDebugSettings3Vtbl {
        unsafe extern "system" fn IsTextPerformanceVisualizationEnabled<Impl: IDebugSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTextPerformanceVisualizationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextPerformanceVisualizationEnabled<Impl: IDebugSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTextPerformanceVisualizationEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDebugSettings3>, base.5, IsTextPerformanceVisualizationEnabled::<Impl, OFFSET>, SetIsTextPerformanceVisualizationEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings4Impl: Sized {
    fn FailFastOnErrors(&self) -> ::windows::core::Result<bool>;
    fn SetFailFastOnErrors(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings4 {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettings4Vtbl {
    pub const fn new<Impl: IDebugSettings4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDebugSettings4Vtbl {
        unsafe extern "system" fn FailFastOnErrors<Impl: IDebugSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FailFastOnErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFailFastOnErrors<Impl: IDebugSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFailFastOnErrors(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDebugSettings4>, base.5, FailFastOnErrors::<Impl, OFFSET>, SetFailFastOnErrors::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectImpl: Sized {
    fn GetValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, dp: &::core::option::Option<DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ClearValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<()>;
    fn ReadLocalValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnimationBaseValue(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyObject {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObject";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyObjectVtbl {
    pub const fn new<Impl: IDependencyObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyObjectVtbl {
        unsafe extern "system" fn GetValue<Impl: IDependencyObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IDependencyObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearValue<Impl: IDependencyObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ClearValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadLocalValue<Impl: IDependencyObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadLocalValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnimationBaseValue<Impl: IDependencyObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnimationBaseValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: IDependencyObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyObject>, base.5, GetValue::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, ClearValue::<Impl, OFFSET>, ReadLocalValue::<Impl, OFFSET>, GetAnimationBaseValue::<Impl, OFFSET>, Dispatcher::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObject2Impl: Sized {
    fn RegisterPropertyChangedCallback(&self, dp: &::core::option::Option<DependencyProperty>, callback: &::core::option::Option<DependencyPropertyChangedCallback>) -> ::windows::core::Result<i64>;
    fn UnregisterPropertyChangedCallback(&self, dp: &::core::option::Option<DependencyProperty>, token: i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyObject2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObject2";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyObject2Vtbl {
    pub const fn new<Impl: IDependencyObject2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyObject2Vtbl {
        unsafe extern "system" fn RegisterPropertyChangedCallback<Impl: IDependencyObject2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, callback: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterPropertyChangedCallback(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&callback as *const <DependencyPropertyChangedCallback as ::windows::core::Abi>::Abi as *const <DependencyPropertyChangedCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterPropertyChangedCallback<Impl: IDependencyObject2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, token: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UnregisterPropertyChangedCallback(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), token).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyObject2>, base.5, RegisterPropertyChangedCallback::<Impl, OFFSET>, UnregisterPropertyChangedCallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DependencyObjectCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyObjectCollectionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObjectCollectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyObjectCollectionFactoryVtbl {
    pub const fn new<Impl: IDependencyObjectCollectionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyObjectCollectionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDependencyObjectCollectionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyObjectCollectionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyObjectFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyObjectFactoryVtbl {
    pub const fn new<Impl: IDependencyObjectFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyObjectFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDependencyObjectFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyObjectFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyImpl: Sized {
    fn GetMetadata(&self, fortype: &Interop::TypeName) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyProperty {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyProperty";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyPropertyVtbl {
    pub const fn new<Impl: IDependencyPropertyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyPropertyVtbl {
        unsafe extern "system" fn GetMetadata<Impl: IDependencyPropertyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fortype: ::core::mem::ManuallyDrop<Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMetadata(&*(&fortype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyProperty>, base.5, GetMetadata::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyChangedEventArgsImpl: Sized {
    fn Property(&self) -> ::windows::core::Result<DependencyProperty>;
    fn OldValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NewValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyPropertyChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyPropertyChangedEventArgsVtbl {
    pub const fn new<Impl: IDependencyPropertyChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyPropertyChangedEventArgsVtbl {
        unsafe extern "system" fn Property<Impl: IDependencyPropertyChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldValue<Impl: IDependencyPropertyChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewValue<Impl: IDependencyPropertyChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyPropertyChangedEventArgs>, base.5, Property::<Impl, OFFSET>, OldValue::<Impl, OFFSET>, NewValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyStaticsImpl: Sized {
    fn UnsetValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Register(&self, name: &::windows::core::HSTRING, propertytype: &Interop::TypeName, ownertype: &Interop::TypeName, typemetadata: &::core::option::Option<PropertyMetadata>) -> ::windows::core::Result<DependencyProperty>;
    fn RegisterAttached(&self, name: &::windows::core::HSTRING, propertytype: &Interop::TypeName, ownertype: &Interop::TypeName, defaultmetadata: &::core::option::Option<PropertyMetadata>) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyPropertyStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyPropertyStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyPropertyStaticsVtbl {
    pub const fn new<Impl: IDependencyPropertyStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDependencyPropertyStaticsVtbl {
        unsafe extern "system" fn UnsetValue<Impl: IDependencyPropertyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnsetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IDependencyPropertyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<Interop::TypeName>, ownertype: ::core::mem::ManuallyDrop<Interop::TypeName>, typemetadata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Register(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&propertytype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&ownertype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&typemetadata as *const <PropertyMetadata as ::windows::core::Abi>::Abi as *const <PropertyMetadata as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAttached<Impl: IDependencyPropertyStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<Interop::TypeName>, ownertype: ::core::mem::ManuallyDrop<Interop::TypeName>, defaultmetadata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterAttached(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&propertytype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&ownertype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultmetadata as *const <PropertyMetadata as ::windows::core::Abi>::Abi as *const <PropertyMetadata as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDependencyPropertyStatics>, base.5, UnsetValue::<Impl, OFFSET>, Register::<Impl, OFFSET>, RegisterAttached::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherTimerImpl: Sized {
    fn Interval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn Tick(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTick(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherTimer {
    const NAME: &'static str = "Windows.UI.Xaml.IDispatcherTimer";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherTimerVtbl {
    pub const fn new<Impl: IDispatcherTimerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherTimerVtbl {
        unsafe extern "system" fn Interval<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tick<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tick(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTick<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTick(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDispatcherTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherTimer>, base.5, Interval::<Impl, OFFSET>, SetInterval::<Impl, OFFSET>, IsEnabled::<Impl, OFFSET>, Tick::<Impl, OFFSET>, RemoveTick::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherTimerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DispatcherTimer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherTimerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDispatcherTimerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherTimerFactoryVtbl {
    pub const fn new<Impl: IDispatcherTimerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDispatcherTimerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDispatcherTimerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDispatcherTimerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn SetData(&self, value: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<UIElement>) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDragEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragEventArgsVtbl {
    pub const fn new<Impl: IDragEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IDragEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDragEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Data<Impl: IDragEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IDragEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IDragEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragEventArgs>, base.5, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragEventArgs2Impl: Sized {
    fn DataView(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageView>;
    fn DragUIOverride(&self) -> ::windows::core::Result<DragUIOverride>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers>;
    fn AcceptedOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn SetAcceptedOperation(&self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<DragOperationDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDragEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDragEventArgs2Vtbl {
    pub const fn new<Impl: IDragEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragEventArgs2Vtbl {
        unsafe extern "system" fn DataView<Impl: IDragEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragUIOverride<Impl: IDragEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragUIOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modifiers<Impl: IDragEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptedOperation<Impl: IDragEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcceptedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceptedOperation<Impl: IDragEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAcceptedOperation(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDragEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragEventArgs2>, base.5, DataView::<Impl, OFFSET>, DragUIOverride::<Impl, OFFSET>, Modifiers::<Impl, OFFSET>, AcceptedOperation::<Impl, OFFSET>, SetAcceptedOperation::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragEventArgs3Impl: Sized {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.IDragEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IDragEventArgs3Vtbl {
    pub const fn new<Impl: IDragEventArgs3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragEventArgs3Vtbl {
        unsafe extern "system" fn AllowedOperations<Impl: IDragEventArgs3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragEventArgs3>, base.5, AllowedOperations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOperationDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragOperationDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.IDragOperationDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDragOperationDeferralVtbl {
    pub const fn new<Impl: IDragOperationDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragOperationDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IDragOperationDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragOperationDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn DragUI(&self) -> ::windows::core::Result<DragUI>;
    fn GetDeferral(&self) -> ::windows::core::Result<DragOperationDeferral>;
    fn GetPosition(&self, relativeto: &::core::option::Option<UIElement>) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDragStartingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragStartingEventArgsVtbl {
    pub const fn new<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragStartingEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn Data<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragUI<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IDragStartingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragStartingEventArgs>, base.5, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>, Data::<Impl, OFFSET>, DragUI::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartingEventArgs2Impl: Sized {
    fn AllowedOperations(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn SetAllowedOperations(&self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragStartingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDragStartingEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IDragStartingEventArgs2Vtbl {
    pub const fn new<Impl: IDragStartingEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragStartingEventArgs2Vtbl {
        unsafe extern "system" fn AllowedOperations<Impl: IDragStartingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedOperations<Impl: IDragStartingEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowedOperations(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragStartingEventArgs2>, base.5, AllowedOperations::<Impl, OFFSET>, SetAllowedOperations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragUIImpl: Sized {
    fn SetContentFromBitmapImage(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImageWithAnchorPoint(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromDataPackage(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragUI {
    const NAME: &'static str = "Windows.UI.Xaml.IDragUI";
}
#[cfg(feature = "implement_exclusive")]
impl IDragUIVtbl {
    pub const fn new<Impl: IDragUIImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragUIVtbl {
        unsafe extern "system" fn SetContentFromBitmapImage<Impl: IDragUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImage(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromBitmapImageWithAnchorPoint<Impl: IDragUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImageWithAnchorPoint(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmap<Impl: IDragUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmap(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmapWithAnchorPoint<Impl: IDragUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmapWithAnchorPoint(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromDataPackage<Impl: IDragUIImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromDataPackage().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragUI>, base.5, SetContentFromBitmapImage::<Impl, OFFSET>, SetContentFromBitmapImageWithAnchorPoint::<Impl, OFFSET>, SetContentFromSoftwareBitmap::<Impl, OFFSET>, SetContentFromSoftwareBitmapWithAnchorPoint::<Impl, OFFSET>, SetContentFromDataPackage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragUIOverrideImpl: Sized {
    fn Caption(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCaption(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsContentVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCaptionVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsCaptionVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsGlyphVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsGlyphVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImage(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImageWithAnchorPoint(&self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmap(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragUIOverride {
    const NAME: &'static str = "Windows.UI.Xaml.IDragUIOverride";
}
#[cfg(feature = "implement_exclusive")]
impl IDragUIOverrideVtbl {
    pub const fn new<Impl: IDragUIOverrideImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragUIOverrideVtbl {
        unsafe extern "system" fn Caption<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCaption(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsContentVisible<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContentVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsContentVisible<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsContentVisible(value).into()
        }
        unsafe extern "system" fn IsCaptionVisible<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCaptionVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCaptionVisible<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsCaptionVisible(value).into()
        }
        unsafe extern "system" fn IsGlyphVisible<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGlyphVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGlyphVisible<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsGlyphVisible(value).into()
        }
        unsafe extern "system" fn Clear<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetContentFromBitmapImage<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImage(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromBitmapImageWithAnchorPoint<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImageWithAnchorPoint(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmap<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmap(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmapWithAnchorPoint<Impl: IDragUIOverrideImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmapWithAnchorPoint(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDragUIOverride>,
            base.5,
            Caption::<Impl, OFFSET>,
            SetCaption::<Impl, OFFSET>,
            IsContentVisible::<Impl, OFFSET>,
            SetIsContentVisible::<Impl, OFFSET>,
            IsCaptionVisible::<Impl, OFFSET>,
            SetIsCaptionVisible::<Impl, OFFSET>,
            IsGlyphVisible::<Impl, OFFSET>,
            SetIsGlyphVisible::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
            SetContentFromBitmapImage::<Impl, OFFSET>,
            SetContentFromBitmapImageWithAnchorPoint::<Impl, OFFSET>,
            SetContentFromSoftwareBitmap::<Impl, OFFSET>,
            SetContentFromSoftwareBitmapWithAnchorPoint::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDropCompletedEventArgsImpl: Sized {
    fn DropResult(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDropCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDropCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDropCompletedEventArgsVtbl {
    pub const fn new<Impl: IDropCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDropCompletedEventArgsVtbl {
        unsafe extern "system" fn DropResult<Impl: IDropCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDropCompletedEventArgs>, base.5, DropResult::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDurationHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDurationHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IDurationHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IDurationHelperVtbl {
    pub const fn new<Impl: IDurationHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDurationHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDurationHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDurationHelperStaticsImpl: Sized {
    fn Automatic(&self) -> ::windows::core::Result<Duration>;
    fn Forever(&self) -> ::windows::core::Result<Duration>;
    fn Compare(&self, duration1: &Duration, duration2: &Duration) -> ::windows::core::Result<i32>;
    fn FromTimeSpan(&self, timespan: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<Duration>;
    fn GetHasTimeSpan(&self, target: &Duration) -> ::windows::core::Result<bool>;
    fn Add(&self, target: &Duration, duration: &Duration) -> ::windows::core::Result<Duration>;
    fn Equals(&self, target: &Duration, value: &Duration) -> ::windows::core::Result<bool>;
    fn Subtract(&self, target: &Duration, duration: &Duration) -> ::windows::core::Result<Duration>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDurationHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IDurationHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IDurationHelperStaticsVtbl {
    pub const fn new<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDurationHelperStaticsVtbl {
        unsafe extern "system" fn Automatic<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Automatic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forever<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Forever() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, duration1: Duration, duration2: Duration, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&duration1 as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&duration2 as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromTimeSpan<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timespan: super::super::Foundation::TimeSpan, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromTimeSpan(&*(&timespan as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasTimeSpan<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: Duration, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHasTimeSpan(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: Duration, duration: Duration, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Add(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: Duration, value: Duration, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtract<Impl: IDurationHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: Duration, duration: Duration, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Subtract(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDurationHelperStatics>, base.5, Automatic::<Impl, OFFSET>, Forever::<Impl, OFFSET>, Compare::<Impl, OFFSET>, FromTimeSpan::<Impl, OFFSET>, GetHasTimeSpan::<Impl, OFFSET>, Add::<Impl, OFFSET>, Equals::<Impl, OFFSET>, Subtract::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEffectiveViewportChangedEventArgsImpl: Sized {
    fn EffectiveViewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn MaxViewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn BringIntoViewDistanceX(&self) -> ::windows::core::Result<f64>;
    fn BringIntoViewDistanceY(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEffectiveViewportChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IEffectiveViewportChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEffectiveViewportChangedEventArgsVtbl {
    pub const fn new<Impl: IEffectiveViewportChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEffectiveViewportChangedEventArgsVtbl {
        unsafe extern "system" fn EffectiveViewport<Impl: IEffectiveViewportChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectiveViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxViewport<Impl: IEffectiveViewportChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringIntoViewDistanceX<Impl: IEffectiveViewportChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BringIntoViewDistanceX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringIntoViewDistanceY<Impl: IEffectiveViewportChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BringIntoViewDistanceY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEffectiveViewportChangedEventArgs>, base.5, EffectiveViewport::<Impl, OFFSET>, MaxViewport::<Impl, OFFSET>, BringIntoViewDistanceX::<Impl, OFFSET>, BringIntoViewDistanceY::<Impl, OFFSET>)
    }
}
pub trait IElementFactoryImpl: Sized {
    fn GetElement(&self, args: &::core::option::Option<ElementFactoryGetArgs>) -> ::windows::core::Result<UIElement>;
    fn RecycleElement(&self, args: &::core::option::Option<ElementFactoryRecycleArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactory";
}
impl IElementFactoryVtbl {
    pub const fn new<Impl: IElementFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementFactoryVtbl {
        unsafe extern "system" fn GetElement<Impl: IElementFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElement(&*(&args as *const <ElementFactoryGetArgs as ::windows::core::Abi>::Abi as *const <ElementFactoryGetArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleElement<Impl: IElementFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RecycleElement(&*(&args as *const <ElementFactoryRecycleArgs as ::windows::core::Abi>::Abi as *const <ElementFactoryRecycleArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementFactory>, base.5, GetElement::<Impl, OFFSET>, RecycleElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryGetArgsImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetData(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<UIElement>;
    fn SetParent(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryGetArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryGetArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryGetArgsVtbl {
    pub const fn new<Impl: IElementFactoryGetArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementFactoryGetArgsVtbl {
        unsafe extern "system" fn Data<Impl: IElementFactoryGetArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IElementFactoryGetArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IElementFactoryGetArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParent<Impl: IElementFactoryGetArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementFactoryGetArgs>, base.5, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>, Parent::<Impl, OFFSET>, SetParent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryGetArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ElementFactoryGetArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryGetArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryGetArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryGetArgsFactoryVtbl {
    pub const fn new<Impl: IElementFactoryGetArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementFactoryGetArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IElementFactoryGetArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementFactoryGetArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryRecycleArgsImpl: Sized {
    fn Element(&self) -> ::windows::core::Result<UIElement>;
    fn SetElement(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<UIElement>;
    fn SetParent(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryRecycleArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryRecycleArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryRecycleArgsVtbl {
    pub const fn new<Impl: IElementFactoryRecycleArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementFactoryRecycleArgsVtbl {
        unsafe extern "system" fn Element<Impl: IElementFactoryRecycleArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Element() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElement<Impl: IElementFactoryRecycleArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetElement(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IElementFactoryRecycleArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParent<Impl: IElementFactoryRecycleArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementFactoryRecycleArgs>, base.5, Element::<Impl, OFFSET>, SetElement::<Impl, OFFSET>, Parent::<Impl, OFFSET>, SetParent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryRecycleArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ElementFactoryRecycleArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryRecycleArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryRecycleArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryRecycleArgsFactoryVtbl {
    pub const fn new<Impl: IElementFactoryRecycleArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementFactoryRecycleArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IElementFactoryRecycleArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementFactoryRecycleArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementSoundPlayer {
    const NAME: &'static str = "Windows.UI.Xaml.IElementSoundPlayer";
}
#[cfg(feature = "implement_exclusive")]
impl IElementSoundPlayerVtbl {
    pub const fn new<Impl: IElementSoundPlayerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementSoundPlayerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementSoundPlayer>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerStaticsImpl: Sized {
    fn Volume(&self) -> ::windows::core::Result<f64>;
    fn SetVolume(&self, value: f64) -> ::windows::core::Result<()>;
    fn State(&self) -> ::windows::core::Result<ElementSoundPlayerState>;
    fn SetState(&self, value: ElementSoundPlayerState) -> ::windows::core::Result<()>;
    fn Play(&self, sound: ElementSoundKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementSoundPlayerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IElementSoundPlayerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IElementSoundPlayerStaticsVtbl {
    pub const fn new<Impl: IElementSoundPlayerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementSoundPlayerStaticsVtbl {
        unsafe extern "system" fn Volume<Impl: IElementSoundPlayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IElementSoundPlayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        unsafe extern "system" fn State<Impl: IElementSoundPlayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ElementSoundPlayerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IElementSoundPlayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ElementSoundPlayerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetState(value).into()
        }
        unsafe extern "system" fn Play<Impl: IElementSoundPlayerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sound: ElementSoundKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Play(sound).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementSoundPlayerStatics>, base.5, Volume::<Impl, OFFSET>, SetVolume::<Impl, OFFSET>, State::<Impl, OFFSET>, SetState::<Impl, OFFSET>, Play::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerStatics2Impl: Sized {
    fn SpatialAudioMode(&self) -> ::windows::core::Result<ElementSpatialAudioMode>;
    fn SetSpatialAudioMode(&self, value: ElementSpatialAudioMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementSoundPlayerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IElementSoundPlayerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IElementSoundPlayerStatics2Vtbl {
    pub const fn new<Impl: IElementSoundPlayerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IElementSoundPlayerStatics2Vtbl {
        unsafe extern "system" fn SpatialAudioMode<Impl: IElementSoundPlayerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ElementSpatialAudioMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SpatialAudioMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpatialAudioMode<Impl: IElementSoundPlayerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ElementSpatialAudioMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSpatialAudioMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IElementSoundPlayerStatics2>, base.5, SpatialAudioMode::<Impl, OFFSET>, SetSpatialAudioMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEventTriggerImpl: Sized {
    fn RoutedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn SetRoutedEvent(&self, value: &::core::option::Option<RoutedEvent>) -> ::windows::core::Result<()>;
    fn Actions(&self) -> ::windows::core::Result<TriggerActionCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEventTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.IEventTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IEventTriggerVtbl {
    pub const fn new<Impl: IEventTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEventTriggerVtbl {
        unsafe extern "system" fn RoutedEvent<Impl: IEventTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoutedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoutedEvent<Impl: IEventTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoutedEvent(&*(&value as *const <RoutedEvent as ::windows::core::Abi>::Abi as *const <RoutedEvent as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Actions<Impl: IEventTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEventTrigger>, base.5, RoutedEvent::<Impl, OFFSET>, SetRoutedEvent::<Impl, OFFSET>, Actions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExceptionRoutedEventArgsImpl: Sized {
    fn ErrorMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExceptionRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IExceptionRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExceptionRoutedEventArgsVtbl {
    pub const fn new<Impl: IExceptionRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExceptionRoutedEventArgsVtbl {
        unsafe extern "system" fn ErrorMessage<Impl: IExceptionRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExceptionRoutedEventArgs>, base.5, ErrorMessage::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExceptionRoutedEventArgsFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExceptionRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IExceptionRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IExceptionRoutedEventArgsFactoryVtbl {
    pub const fn new<Impl: IExceptionRoutedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExceptionRoutedEventArgsFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExceptionRoutedEventArgsFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementImpl: Sized {
    fn Triggers(&self) -> ::windows::core::Result<TriggerCollection>;
    fn Resources(&self) -> ::windows::core::Result<ResourceDictionary>;
    fn SetResources(&self, value: &::core::option::Option<ResourceDictionary>) -> ::windows::core::Result<()>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ActualWidth(&self) -> ::windows::core::Result<f64>;
    fn ActualHeight(&self) -> ::windows::core::Result<f64>;
    fn Width(&self) -> ::windows::core::Result<f64>;
    fn SetWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn Height(&self) -> ::windows::core::Result<f64>;
    fn SetHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMinWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxWidth(&self) -> ::windows::core::Result<f64>;
    fn SetMaxWidth(&self, value: f64) -> ::windows::core::Result<()>;
    fn MinHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMinHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn MaxHeight(&self) -> ::windows::core::Result<f64>;
    fn SetMaxHeight(&self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalAlignment(&self) -> ::windows::core::Result<HorizontalAlignment>;
    fn SetHorizontalAlignment(&self, value: HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalAlignment(&self) -> ::windows::core::Result<VerticalAlignment>;
    fn SetVerticalAlignment(&self, value: VerticalAlignment) -> ::windows::core::Result<()>;
    fn Margin(&self) -> ::windows::core::Result<Thickness>;
    fn SetMargin(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn DataContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDataContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Style(&self) -> ::windows::core::Result<Style>;
    fn SetStyle(&self, value: &::core::option::Option<Style>) -> ::windows::core::Result<()>;
    fn Parent(&self) -> ::windows::core::Result<DependencyObject>;
    fn FlowDirection(&self) -> ::windows::core::Result<FlowDirection>;
    fn SetFlowDirection(&self, value: FlowDirection) -> ::windows::core::Result<()>;
    fn Loaded(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unloaded(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnloaded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<SizeChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LayoutUpdated(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetBinding(&self, dp: &::core::option::Option<DependencyProperty>, binding: &::core::option::Option<Data::BindingBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElement {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementVtbl {
    pub const fn new<Impl: IFrameworkElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementVtbl {
        unsafe extern "system" fn Triggers<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Triggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResources<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetResources(&*(&value as *const <ResourceDictionary as ::windows::core::Abi>::Abi as *const <ResourceDictionary as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualWidth<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualHeight<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        unsafe extern "system" fn Height<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeight(value).into()
        }
        unsafe extern "system" fn MinWidth<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinWidth<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinWidth(value).into()
        }
        unsafe extern "system" fn MaxWidth<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxWidth<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxWidth(value).into()
        }
        unsafe extern "system" fn MinHeight<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinHeight<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinHeight(value).into()
        }
        unsafe extern "system" fn MaxHeight<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHeight<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxHeight(value).into()
        }
        unsafe extern "system" fn HorizontalAlignment<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignment<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignment(value).into()
        }
        unsafe extern "system" fn VerticalAlignment<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignment<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignment(value).into()
        }
        unsafe extern "system" fn Margin<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Margin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMargin<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMargin(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseUri<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BaseUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataContext<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataContext<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDataContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Style<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStyle(&*(&value as *const <Style as ::windows::core::Abi>::Abi as *const <Style as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirection<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFlowDirection(value).into()
        }
        unsafe extern "system" fn Loaded<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Loaded(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoaded<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLoaded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unloaded<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unloaded(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnloaded<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUnloaded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SizeChanged<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SizeChanged(&*(&handler as *const <SizeChangedEventHandler as ::windows::core::Abi>::Abi as *const <SizeChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSizeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LayoutUpdated<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LayoutUpdated(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLayoutUpdated<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLayoutUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindName<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinding<Impl: IFrameworkElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, binding: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBinding(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&binding as *const <Data::BindingBase as ::windows::core::Abi>::Abi as *const <Data::BindingBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IFrameworkElement>,
            base.5,
            Triggers::<Impl, OFFSET>,
            Resources::<Impl, OFFSET>,
            SetResources::<Impl, OFFSET>,
            Tag::<Impl, OFFSET>,
            SetTag::<Impl, OFFSET>,
            Language::<Impl, OFFSET>,
            SetLanguage::<Impl, OFFSET>,
            ActualWidth::<Impl, OFFSET>,
            ActualHeight::<Impl, OFFSET>,
            Width::<Impl, OFFSET>,
            SetWidth::<Impl, OFFSET>,
            Height::<Impl, OFFSET>,
            SetHeight::<Impl, OFFSET>,
            MinWidth::<Impl, OFFSET>,
            SetMinWidth::<Impl, OFFSET>,
            MaxWidth::<Impl, OFFSET>,
            SetMaxWidth::<Impl, OFFSET>,
            MinHeight::<Impl, OFFSET>,
            SetMinHeight::<Impl, OFFSET>,
            MaxHeight::<Impl, OFFSET>,
            SetMaxHeight::<Impl, OFFSET>,
            HorizontalAlignment::<Impl, OFFSET>,
            SetHorizontalAlignment::<Impl, OFFSET>,
            VerticalAlignment::<Impl, OFFSET>,
            SetVerticalAlignment::<Impl, OFFSET>,
            Margin::<Impl, OFFSET>,
            SetMargin::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            BaseUri::<Impl, OFFSET>,
            DataContext::<Impl, OFFSET>,
            SetDataContext::<Impl, OFFSET>,
            Style::<Impl, OFFSET>,
            SetStyle::<Impl, OFFSET>,
            Parent::<Impl, OFFSET>,
            FlowDirection::<Impl, OFFSET>,
            SetFlowDirection::<Impl, OFFSET>,
            Loaded::<Impl, OFFSET>,
            RemoveLoaded::<Impl, OFFSET>,
            Unloaded::<Impl, OFFSET>,
            RemoveUnloaded::<Impl, OFFSET>,
            SizeChanged::<Impl, OFFSET>,
            RemoveSizeChanged::<Impl, OFFSET>,
            LayoutUpdated::<Impl, OFFSET>,
            RemoveLayoutUpdated::<Impl, OFFSET>,
            FindName::<Impl, OFFSET>,
            SetBinding::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement2Impl: Sized {
    fn RequestedTheme(&self) -> ::windows::core::Result<ElementTheme>;
    fn SetRequestedTheme(&self, value: ElementTheme) -> ::windows::core::Result<()>;
    fn DataContextChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataContextChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetBindingExpression(&self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<Data::BindingExpression>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElement2Vtbl {
    pub const fn new<Impl: IFrameworkElement2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElement2Vtbl {
        unsafe extern "system" fn RequestedTheme<Impl: IFrameworkElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ElementTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedTheme<Impl: IFrameworkElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ElementTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequestedTheme(value).into()
        }
        unsafe extern "system" fn DataContextChanged<Impl: IFrameworkElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataContextChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataContextChanged<Impl: IFrameworkElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDataContextChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBindingExpression<Impl: IFrameworkElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBindingExpression(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElement2>, base.5, RequestedTheme::<Impl, OFFSET>, SetRequestedTheme::<Impl, OFFSET>, DataContextChanged::<Impl, OFFSET>, RemoveDataContextChanged::<Impl, OFFSET>, GetBindingExpression::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement3Impl: Sized {
    fn Loading(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoading(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement3";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElement3Vtbl {
    pub const fn new<Impl: IFrameworkElement3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElement3Vtbl {
        unsafe extern "system" fn Loading<Impl: IFrameworkElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Loading(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoading<Impl: IFrameworkElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLoading(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElement3>, base.5, Loading::<Impl, OFFSET>, RemoveLoading::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement4Impl: Sized {
    fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()>;
    fn FocusVisualMargin(&self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualMargin(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualSecondaryThickness(&self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualSecondaryThickness(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualPrimaryThickness(&self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualPrimaryThickness(&self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualSecondaryBrush(&self) -> ::windows::core::Result<Media::Brush>;
    fn SetFocusVisualSecondaryBrush(&self, value: &::core::option::Option<Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusVisualPrimaryBrush(&self) -> ::windows::core::Result<Media::Brush>;
    fn SetFocusVisualPrimaryBrush(&self, value: &::core::option::Option<Media::Brush>) -> ::windows::core::Result<()>;
    fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement4";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElement4Vtbl {
    pub const fn new<Impl: IFrameworkElement4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElement4Vtbl {
        unsafe extern "system" fn AllowFocusOnInteraction<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteraction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusOnInteraction<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowFocusOnInteraction(value).into()
        }
        unsafe extern "system" fn FocusVisualMargin<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualMargin<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusVisualMargin(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualSecondaryThickness<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualSecondaryThickness<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusVisualSecondaryThickness(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualPrimaryThickness<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualPrimaryThickness<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusVisualPrimaryThickness(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualSecondaryBrush<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualSecondaryBrush<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusVisualSecondaryBrush(&*(&value as *const <Media::Brush as ::windows::core::Abi>::Abi as *const <Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualPrimaryBrush<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualPrimaryBrush<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusVisualPrimaryBrush(&*(&value as *const <Media::Brush as ::windows::core::Abi>::Abi as *const <Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowFocusWhenDisabled<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusWhenDisabled<Impl: IFrameworkElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowFocusWhenDisabled(value).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IFrameworkElement4>,
            base.5,
            AllowFocusOnInteraction::<Impl, OFFSET>,
            SetAllowFocusOnInteraction::<Impl, OFFSET>,
            FocusVisualMargin::<Impl, OFFSET>,
            SetFocusVisualMargin::<Impl, OFFSET>,
            FocusVisualSecondaryThickness::<Impl, OFFSET>,
            SetFocusVisualSecondaryThickness::<Impl, OFFSET>,
            FocusVisualPrimaryThickness::<Impl, OFFSET>,
            SetFocusVisualPrimaryThickness::<Impl, OFFSET>,
            FocusVisualSecondaryBrush::<Impl, OFFSET>,
            SetFocusVisualSecondaryBrush::<Impl, OFFSET>,
            FocusVisualPrimaryBrush::<Impl, OFFSET>,
            SetFocusVisualPrimaryBrush::<Impl, OFFSET>,
            AllowFocusWhenDisabled::<Impl, OFFSET>,
            SetAllowFocusWhenDisabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement6Impl: Sized {
    fn ActualTheme(&self) -> ::windows::core::Result<ElementTheme>;
    fn ActualThemeChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualThemeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElement6 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement6";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElement6Vtbl {
    pub const fn new<Impl: IFrameworkElement6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElement6Vtbl {
        unsafe extern "system" fn ActualTheme<Impl: IFrameworkElement6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ElementTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualThemeChanged<Impl: IFrameworkElement6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualThemeChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualThemeChanged<Impl: IFrameworkElement6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActualThemeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElement6>, base.5, ActualTheme::<Impl, OFFSET>, ActualThemeChanged::<Impl, OFFSET>, RemoveActualThemeChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElement7Impl: Sized {
    fn IsLoaded(&self) -> ::windows::core::Result<bool>;
    fn EffectiveViewportChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEffectiveViewportChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElement7 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement7";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElement7Vtbl {
    pub const fn new<Impl: IFrameworkElement7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElement7Vtbl {
        unsafe extern "system" fn IsLoaded<Impl: IFrameworkElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLoaded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectiveViewportChanged<Impl: IFrameworkElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectiveViewportChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEffectiveViewportChanged<Impl: IFrameworkElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveEffectiveViewportChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElement7>, base.5, IsLoaded::<Impl, OFFSET>, EffectiveViewportChanged::<Impl, OFFSET>, RemoveEffectiveViewportChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementFactoryVtbl {
    pub const fn new<Impl: IFrameworkElementFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFrameworkElementFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementOverridesImpl: Sized {
    fn MeasureOverride(&self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ArrangeOverride(&self, finalsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn OnApplyTemplate(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementOverridesVtbl {
    pub const fn new<Impl: IFrameworkElementOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementOverridesVtbl {
        unsafe extern "system" fn MeasureOverride<Impl: IFrameworkElementOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availablesize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MeasureOverride(&*(&availablesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<Impl: IFrameworkElementOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalsize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ArrangeOverride(&*(&finalsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnApplyTemplate<Impl: IFrameworkElementOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnApplyTemplate().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementOverrides>, base.5, MeasureOverride::<Impl, OFFSET>, ArrangeOverride::<Impl, OFFSET>, OnApplyTemplate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementOverrides2Impl: Sized {
    fn GoToElementStateCore(&self, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementOverrides2Vtbl {
    pub const fn new<Impl: IFrameworkElementOverrides2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementOverrides2Vtbl {
        unsafe extern "system" fn GoToElementStateCore<Impl: IFrameworkElementOverrides2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GoToElementStateCore(&*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementOverrides2>, base.5, GoToElementStateCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementProtected7Impl: Sized {
    fn InvalidateViewport(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementProtected7 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementProtected7";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementProtected7Vtbl {
    pub const fn new<Impl: IFrameworkElementProtected7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementProtected7Vtbl {
        unsafe extern "system" fn InvalidateViewport<Impl: IFrameworkElementProtected7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InvalidateViewport().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementProtected7>, base.5, InvalidateViewport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStaticsImpl: Sized {
    fn TagProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn LanguageProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ActualWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ActualHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn WidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn HeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MinWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MaxWidthProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MinHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MaxHeightProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn HorizontalAlignmentProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn VerticalAlignmentProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn MarginProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn NameProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn DataContextProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn StyleProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FlowDirectionProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStaticsVtbl {
    pub const fn new<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementStaticsVtbl {
        unsafe extern "system" fn TagProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TagProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LanguageProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualWidthProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualHeightProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinWidthProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxWidthProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinHeightProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxHeightProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalAlignmentProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalAlignmentProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataContextProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DataContextProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirectionProperty<Impl: IFrameworkElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlowDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IFrameworkElementStatics>,
            base.5,
            TagProperty::<Impl, OFFSET>,
            LanguageProperty::<Impl, OFFSET>,
            ActualWidthProperty::<Impl, OFFSET>,
            ActualHeightProperty::<Impl, OFFSET>,
            WidthProperty::<Impl, OFFSET>,
            HeightProperty::<Impl, OFFSET>,
            MinWidthProperty::<Impl, OFFSET>,
            MaxWidthProperty::<Impl, OFFSET>,
            MinHeightProperty::<Impl, OFFSET>,
            MaxHeightProperty::<Impl, OFFSET>,
            HorizontalAlignmentProperty::<Impl, OFFSET>,
            VerticalAlignmentProperty::<Impl, OFFSET>,
            MarginProperty::<Impl, OFFSET>,
            NameProperty::<Impl, OFFSET>,
            DataContextProperty::<Impl, OFFSET>,
            StyleProperty::<Impl, OFFSET>,
            FlowDirectionProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics2Impl: Sized {
    fn RequestedThemeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics2Vtbl {
    pub const fn new<Impl: IFrameworkElementStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementStatics2Vtbl {
        unsafe extern "system" fn RequestedThemeProperty<Impl: IFrameworkElementStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestedThemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementStatics2>, base.5, RequestedThemeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics4Impl: Sized {
    fn AllowFocusOnInteractionProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualMarginProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualSecondaryThicknessProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualPrimaryThicknessProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualSecondaryBrushProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualPrimaryBrushProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn AllowFocusWhenDisabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics4Vtbl {
    pub const fn new<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementStatics4Vtbl {
        unsafe extern "system" fn AllowFocusOnInteractionProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteractionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualMarginProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualSecondaryThicknessProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualPrimaryThicknessProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualSecondaryBrushProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualPrimaryBrushProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowFocusWhenDisabledProperty<Impl: IFrameworkElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementStatics4>, base.5, AllowFocusOnInteractionProperty::<Impl, OFFSET>, FocusVisualMarginProperty::<Impl, OFFSET>, FocusVisualSecondaryThicknessProperty::<Impl, OFFSET>, FocusVisualPrimaryThicknessProperty::<Impl, OFFSET>, FocusVisualSecondaryBrushProperty::<Impl, OFFSET>, FocusVisualPrimaryBrushProperty::<Impl, OFFSET>, AllowFocusWhenDisabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics5Impl: Sized {
    fn DeferTree(&self, element: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics5Vtbl {
    pub const fn new<Impl: IFrameworkElementStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementStatics5Vtbl {
        unsafe extern "system" fn DeferTree<Impl: IFrameworkElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).DeferTree(&*(&element as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementStatics5>, base.5, DeferTree::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics6Impl: Sized {
    fn ActualThemeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics6Vtbl {
    pub const fn new<Impl: IFrameworkElementStatics6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementStatics6Vtbl {
        unsafe extern "system" fn ActualThemeProperty<Impl: IFrameworkElementStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualThemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementStatics6>, base.5, ActualThemeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkTemplateImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkTemplate";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkTemplateVtbl {
    pub const fn new<Impl: IFrameworkTemplateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkTemplateVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkTemplate>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkTemplateFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkTemplate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkTemplateFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkTemplateFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkTemplateFactoryVtbl {
    pub const fn new<Impl: IFrameworkTemplateFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkTemplateFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFrameworkTemplateFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkTemplateFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkViewImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkView {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkView";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkViewVtbl {
    pub const fn new<Impl: IFrameworkViewImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkViewVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkView>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkViewSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkViewSource {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkViewSource";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkViewSourceVtbl {
    pub const fn new<Impl: IFrameworkViewSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkViewSourceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkViewSource>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridLengthHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridLengthHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IGridLengthHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IGridLengthHelperVtbl {
    pub const fn new<Impl: IGridLengthHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridLengthHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridLengthHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridLengthHelperStaticsImpl: Sized {
    fn Auto(&self) -> ::windows::core::Result<GridLength>;
    fn FromPixels(&self, pixels: f64) -> ::windows::core::Result<GridLength>;
    fn FromValueAndType(&self, value: f64, r#type: GridUnitType) -> ::windows::core::Result<GridLength>;
    fn GetIsAbsolute(&self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn GetIsAuto(&self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn GetIsStar(&self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &GridLength, value: &GridLength) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridLengthHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IGridLengthHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridLengthHelperStaticsVtbl {
    pub const fn new<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridLengthHelperStaticsVtbl {
        unsafe extern "system" fn Auto<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromPixels<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixels: f64, result__: *mut GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromPixels(pixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromValueAndType<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64, r#type: GridUnitType, result__: *mut GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromValueAndType(value, r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAbsolute<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsAbsolute(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAuto<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsAuto(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsStar<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsStar(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IGridLengthHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: GridLength, value: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridLengthHelperStatics>, base.5, Auto::<Impl, OFFSET>, FromPixels::<Impl, OFFSET>, FromValueAndType::<Impl, OFFSET>, GetIsAbsolute::<Impl, OFFSET>, GetIsAuto::<Impl, OFFSET>, GetIsStar::<Impl, OFFSET>, Equals::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFailedRoutedEventArgsImpl: Sized {
    fn ErrorTrace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFailedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IMediaFailedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFailedRoutedEventArgsVtbl {
    pub const fn new<Impl: IMediaFailedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaFailedRoutedEventArgsVtbl {
        unsafe extern "system" fn ErrorTrace<Impl: IMediaFailedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ErrorTrace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaFailedRoutedEventArgs>, base.5, ErrorTrace::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IPointHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IPointHelperVtbl {
    pub const fn new<Impl: IPointHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointHelperStaticsImpl: Sized {
    fn FromCoordinates(&self, x: f32, y: f32) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IPointHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPointHelperStaticsVtbl {
    pub const fn new<Impl: IPointHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointHelperStaticsVtbl {
        unsafe extern "system" fn FromCoordinates<Impl: IPointHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromCoordinates(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointHelperStatics>, base.5, FromCoordinates::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataImpl: Sized {
    fn DefaultValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDefaultValueCallback(&self) -> ::windows::core::Result<CreateDefaultValueCallback>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyMetadata {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyMetadata";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyMetadataVtbl {
    pub const fn new<Impl: IPropertyMetadataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyMetadataVtbl {
        unsafe extern "system" fn DefaultValue<Impl: IPropertyMetadataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultValueCallback<Impl: IPropertyMetadataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateDefaultValueCallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyMetadata>, base.5, DefaultValue::<Impl, OFFSET>, CreateDefaultValueCallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataFactoryImpl: Sized {
    fn CreateInstanceWithDefaultValue(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateInstanceWithDefaultValueAndCallback(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyMetadataFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyMetadataFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyMetadataFactoryVtbl {
    pub const fn new<Impl: IPropertyMetadataFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyMetadataFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithDefaultValue<Impl: IPropertyMetadataFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDefaultValue(&*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDefaultValueAndCallback<Impl: IPropertyMetadataFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, propertychangedcallback: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDefaultValueAndCallback(
                &*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&propertychangedcallback as *const <PropertyChangedCallback as ::windows::core::Abi>::Abi as *const <PropertyChangedCallback as ::windows::core::DefaultType>::DefaultType),
                &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&innerinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyMetadataFactory>, base.5, CreateInstanceWithDefaultValue::<Impl, OFFSET>, CreateInstanceWithDefaultValueAndCallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataStaticsImpl: Sized {
    fn CreateWithDefaultValue(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithDefaultValueAndCallback(&self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithFactory(&self, createdefaultvaluecallback: &::core::option::Option<CreateDefaultValueCallback>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithFactoryAndCallback(&self, createdefaultvaluecallback: &::core::option::Option<CreateDefaultValueCallback>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyMetadataStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyMetadataStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyMetadataStaticsVtbl {
    pub const fn new<Impl: IPropertyMetadataStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyMetadataStaticsVtbl {
        unsafe extern "system" fn CreateWithDefaultValue<Impl: IPropertyMetadataStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithDefaultValue(&*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithDefaultValueAndCallback<Impl: IPropertyMetadataStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, propertychangedcallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithDefaultValueAndCallback(&*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertychangedcallback as *const <PropertyChangedCallback as ::windows::core::Abi>::Abi as *const <PropertyChangedCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithFactory<Impl: IPropertyMetadataStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, createdefaultvaluecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithFactory(&*(&createdefaultvaluecallback as *const <CreateDefaultValueCallback as ::windows::core::Abi>::Abi as *const <CreateDefaultValueCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithFactoryAndCallback<Impl: IPropertyMetadataStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, createdefaultvaluecallback: ::windows::core::RawPtr, propertychangedcallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithFactoryAndCallback(&*(&createdefaultvaluecallback as *const <CreateDefaultValueCallback as ::windows::core::Abi>::Abi as *const <CreateDefaultValueCallback as ::windows::core::DefaultType>::DefaultType), &*(&propertychangedcallback as *const <PropertyChangedCallback as ::windows::core::Abi>::Abi as *const <PropertyChangedCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyMetadataStatics>, base.5, CreateWithDefaultValue::<Impl, OFFSET>, CreateWithDefaultValueAndCallback::<Impl, OFFSET>, CreateWithFactory::<Impl, OFFSET>, CreateWithFactoryAndCallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyPathImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyPath {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyPath";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyPathVtbl {
    pub const fn new<Impl: IPropertyPathImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyPathVtbl {
        unsafe extern "system" fn Path<Impl: IPropertyPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyPath>, base.5, Path::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyPathFactoryImpl: Sized {
    fn CreateInstance(&self, path: &::windows::core::HSTRING) -> ::windows::core::Result<PropertyPath>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyPathFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyPathFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyPathFactoryVtbl {
    pub const fn new<Impl: IPropertyPathFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPropertyPathFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPropertyPathFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPropertyPathFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IRectHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IRectHelperVtbl {
    pub const fn new<Impl: IRectHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRectHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRectHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectHelperStaticsImpl: Sized {
    fn Empty(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromCoordinatesAndDimensions(&self, x: f32, y: f32, width: f32, height: f32) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromPoints(&self, point1: &super::super::Foundation::Point, point2: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromLocationAndSize(&self, location: &super::super::Foundation::Point, size: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn GetIsEmpty(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
    fn GetBottom(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetLeft(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetRight(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetTop(&self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn Contains(&self, target: &super::super::Foundation::Rect, point: &super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &super::super::Foundation::Rect, value: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
    fn Intersect(&self, target: &super::super::Foundation::Rect, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn UnionWithPoint(&self, target: &super::super::Foundation::Rect, point: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn UnionWithRect(&self, target: &super::super::Foundation::Rect, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IRectHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRectHelperStaticsVtbl {
    pub const fn new<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRectHelperStaticsVtbl {
        unsafe extern "system" fn Empty<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromCoordinatesAndDimensions<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, width: f32, height: f32, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromCoordinatesAndDimensions(x, y, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromPoints<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point1: super::super::Foundation::Point, point2: super::super::Foundation::Point, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromPoints(&*(&point1 as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&point2 as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromLocationAndSize<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: super::super::Foundation::Point, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromLocationAndSize(&*(&location as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsEmpty<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsEmpty(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBottom<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBottom(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeft<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLeft(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRight<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRight(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTop<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTop(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contains<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, point: super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Contains(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, value: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Intersect<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Intersect(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnionWithPoint<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, point: super::super::Foundation::Point, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnionWithPoint(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnionWithRect<Impl: IRectHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnionWithRect(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IRectHelperStatics>,
            base.5,
            Empty::<Impl, OFFSET>,
            FromCoordinatesAndDimensions::<Impl, OFFSET>,
            FromPoints::<Impl, OFFSET>,
            FromLocationAndSize::<Impl, OFFSET>,
            GetIsEmpty::<Impl, OFFSET>,
            GetBottom::<Impl, OFFSET>,
            GetLeft::<Impl, OFFSET>,
            GetRight::<Impl, OFFSET>,
            GetTop::<Impl, OFFSET>,
            Contains::<Impl, OFFSET>,
            Equals::<Impl, OFFSET>,
            Intersect::<Impl, OFFSET>,
            UnionWithPoint::<Impl, OFFSET>,
            UnionWithRect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceDictionaryImpl: Sized {
    fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSource(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn MergedDictionaries(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ResourceDictionary>>;
    fn ThemeDictionaries(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::IInspectable, ::windows::core::IInspectable>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceDictionary {
    const NAME: &'static str = "Windows.UI.Xaml.IResourceDictionary";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceDictionaryVtbl {
    pub const fn new<Impl: IResourceDictionaryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceDictionaryVtbl {
        unsafe extern "system" fn Source<Impl: IResourceDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IResourceDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MergedDictionaries<Impl: IResourceDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MergedDictionaries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThemeDictionaries<Impl: IResourceDictionaryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ThemeDictionaries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceDictionary>, base.5, Source::<Impl, OFFSET>, SetSource::<Impl, OFFSET>, MergedDictionaries::<Impl, OFFSET>, ThemeDictionaries::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceDictionaryFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ResourceDictionary>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceDictionaryFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IResourceDictionaryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceDictionaryFactoryVtbl {
    pub const fn new<Impl: IResourceDictionaryFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IResourceDictionaryFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IResourceDictionaryFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IResourceDictionaryFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutedEvent {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEvent";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutedEventVtbl {
    pub const fn new<Impl: IRoutedEventImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRoutedEventVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRoutedEvent>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventArgsImpl: Sized {
    fn OriginalSource(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutedEventArgsVtbl {
    pub const fn new<Impl: IRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRoutedEventArgsVtbl {
        unsafe extern "system" fn OriginalSource<Impl: IRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OriginalSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRoutedEventArgs>, base.5, OriginalSource::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutedEventArgsFactoryVtbl {
    pub const fn new<Impl: IRoutedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRoutedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRoutedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRoutedEventArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarTransitionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarTransition {
    const NAME: &'static str = "Windows.UI.Xaml.IScalarTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarTransitionVtbl {
    pub const fn new<Impl: IScalarTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScalarTransitionVtbl {
        unsafe extern "system" fn Duration<Impl: IScalarTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IScalarTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScalarTransition>, base.5, Duration::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarTransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScalarTransition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarTransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IScalarTransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarTransitionFactoryVtbl {
    pub const fn new<Impl: IScalarTransitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScalarTransitionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IScalarTransitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScalarTransitionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterImpl: Sized {
    fn Property(&self) -> ::windows::core::Result<DependencyProperty>;
    fn SetProperty(&self, value: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetter {
    const NAME: &'static str = "Windows.UI.Xaml.ISetter";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterVtbl {
    pub const fn new<Impl: ISetterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetterVtbl {
        unsafe extern "system" fn Property<Impl: ISetterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ISetterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProperty(&*(&value as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ISetterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISetterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetter>, base.5, Property::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetter2Impl: Sized {
    fn Target(&self) -> ::windows::core::Result<TargetPropertyPath>;
    fn SetTarget(&self, value: &::core::option::Option<TargetPropertyPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetter2 {
    const NAME: &'static str = "Windows.UI.Xaml.ISetter2";
}
#[cfg(feature = "implement_exclusive")]
impl ISetter2Vtbl {
    pub const fn new<Impl: ISetter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetter2Vtbl {
        unsafe extern "system" fn Target<Impl: ISetter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTarget<Impl: ISetter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&value as *const <TargetPropertyPath as ::windows::core::Abi>::Abi as *const <TargetPropertyPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetter2>, base.5, Target::<Impl, OFFSET>, SetTarget::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseImpl: Sized {
    fn IsSealed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterBase {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterBase";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterBaseVtbl {
    pub const fn new<Impl: ISetterBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetterBaseVtbl {
        unsafe extern "system" fn IsSealed<Impl: ISetterBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSealed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetterBase>, base.5, IsSealed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseCollectionImpl: Sized {
    fn IsSealed(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterBaseCollection {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterBaseCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterBaseCollectionVtbl {
    pub const fn new<Impl: ISetterBaseCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetterBaseCollectionVtbl {
        unsafe extern "system" fn IsSealed<Impl: ISetterBaseCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSealed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetterBaseCollection>, base.5, IsSealed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterBaseFactoryVtbl {
    pub const fn new<Impl: ISetterBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetterBaseFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetterBaseFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterFactoryImpl: Sized {
    fn CreateInstance(&self, targetproperty: &::core::option::Option<DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Setter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterFactoryVtbl {
    pub const fn new<Impl: ISetterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISetterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISetterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetproperty: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&targetproperty as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISetterFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeChangedEventArgsImpl: Sized {
    fn PreviousSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn NewSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.ISizeChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ISizeChangedEventArgsVtbl {
    pub const fn new<Impl: ISizeChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISizeChangedEventArgsVtbl {
        unsafe extern "system" fn PreviousSize<Impl: ISizeChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviousSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewSize<Impl: ISizeChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISizeChangedEventArgs>, base.5, PreviousSize::<Impl, OFFSET>, NewSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISizeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.ISizeHelper";
}
#[cfg(feature = "implement_exclusive")]
impl ISizeHelperVtbl {
    pub const fn new<Impl: ISizeHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISizeHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISizeHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeHelperStaticsImpl: Sized {
    fn Empty(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn FromDimensions(&self, width: f32, height: f32) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn GetIsEmpty(&self, target: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn Equals(&self, target: &super::super::Foundation::Size, value: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISizeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.ISizeHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISizeHelperStaticsVtbl {
    pub const fn new<Impl: ISizeHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISizeHelperStaticsVtbl {
        unsafe extern "system" fn Empty<Impl: ISizeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromDimensions<Impl: ISizeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: f32, height: f32, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromDimensions(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsEmpty<Impl: ISizeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsEmpty(&*(&target as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: ISizeHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Size, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISizeHelperStatics>, base.5, Empty::<Impl, OFFSET>, FromDimensions::<Impl, OFFSET>, GetIsEmpty::<Impl, OFFSET>, Equals::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerImpl: Sized {
    fn IsActive(&self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerVtbl {
    pub const fn new<Impl: IStateTriggerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStateTriggerVtbl {
        unsafe extern "system" fn IsActive<Impl: IStateTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActive<Impl: IStateTriggerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsActive(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStateTrigger>, base.5, IsActive::<Impl, OFFSET>, SetIsActive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerBase {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerBase";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerBaseVtbl {
    pub const fn new<Impl: IStateTriggerBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStateTriggerBaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStateTriggerBase>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StateTriggerBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerBaseFactoryVtbl {
    pub const fn new<Impl: IStateTriggerBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStateTriggerBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStateTriggerBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStateTriggerBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseProtectedImpl: Sized {
    fn SetActive(&self, isactive: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerBaseProtected {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerBaseProtected";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerBaseProtectedVtbl {
    pub const fn new<Impl: IStateTriggerBaseProtectedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStateTriggerBaseProtectedVtbl {
        unsafe extern "system" fn SetActive<Impl: IStateTriggerBaseProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isactive: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetActive(isactive).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStateTriggerBaseProtected>, base.5, SetActive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerStaticsImpl: Sized {
    fn IsActiveProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerStaticsVtbl {
    pub const fn new<Impl: IStateTriggerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStateTriggerStaticsVtbl {
        unsafe extern "system" fn IsActiveProperty<Impl: IStateTriggerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsActiveProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStateTriggerStatics>, base.5, IsActiveProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleImpl: Sized {
    fn IsSealed(&self) -> ::windows::core::Result<bool>;
    fn Setters(&self) -> ::windows::core::Result<SetterBaseCollection>;
    fn TargetType(&self) -> ::windows::core::Result<Interop::TypeName>;
    fn SetTargetType(&self, value: &Interop::TypeName) -> ::windows::core::Result<()>;
    fn BasedOn(&self) -> ::windows::core::Result<Style>;
    fn SetBasedOn(&self, value: &::core::option::Option<Style>) -> ::windows::core::Result<()>;
    fn Seal(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStyle {
    const NAME: &'static str = "Windows.UI.Xaml.IStyle";
}
#[cfg(feature = "implement_exclusive")]
impl IStyleVtbl {
    pub const fn new<Impl: IStyleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStyleVtbl {
        unsafe extern "system" fn IsSealed<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSealed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setters<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetType<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetType<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTargetType(&*(&value as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BasedOn<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BasedOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasedOn<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBasedOn(&*(&value as *const <Style as ::windows::core::Abi>::Abi as *const <Style as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Seal<Impl: IStyleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Seal().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStyle>, base.5, IsSealed::<Impl, OFFSET>, Setters::<Impl, OFFSET>, TargetType::<Impl, OFFSET>, SetTargetType::<Impl, OFFSET>, BasedOn::<Impl, OFFSET>, SetBasedOn::<Impl, OFFSET>, Seal::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStyleFactoryImpl: Sized {
    fn CreateInstance(&self, targettype: &Interop::TypeName) -> ::windows::core::Result<Style>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStyleFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IStyleFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStyleFactoryVtbl {
    pub const fn new<Impl: IStyleFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStyleFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStyleFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&targettype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStyleFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetPropertyPathImpl: Sized {
    fn Path(&self) -> ::windows::core::Result<PropertyPath>;
    fn SetPath(&self, value: &::core::option::Option<PropertyPath>) -> ::windows::core::Result<()>;
    fn Target(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTarget(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetPropertyPath {
    const NAME: &'static str = "Windows.UI.Xaml.ITargetPropertyPath";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetPropertyPathVtbl {
    pub const fn new<Impl: ITargetPropertyPathImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITargetPropertyPathVtbl {
        unsafe extern "system" fn Path<Impl: ITargetPropertyPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: ITargetPropertyPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <PropertyPath as ::windows::core::Abi>::Abi as *const <PropertyPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Target<Impl: ITargetPropertyPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTarget<Impl: ITargetPropertyPathImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITargetPropertyPath>, base.5, Path::<Impl, OFFSET>, SetPath::<Impl, OFFSET>, Target::<Impl, OFFSET>, SetTarget::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetPropertyPathFactoryImpl: Sized {
    fn CreateInstance(&self, targetproperty: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<TargetPropertyPath>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetPropertyPathFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ITargetPropertyPathFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetPropertyPathFactoryVtbl {
    pub const fn new<Impl: ITargetPropertyPathFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITargetPropertyPathFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITargetPropertyPathFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetproperty: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&targetproperty as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITargetPropertyPathFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThicknessHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThicknessHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IThicknessHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IThicknessHelperVtbl {
    pub const fn new<Impl: IThicknessHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThicknessHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThicknessHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThicknessHelperStaticsImpl: Sized {
    fn FromLengths(&self, left: f64, top: f64, right: f64, bottom: f64) -> ::windows::core::Result<Thickness>;
    fn FromUniformLength(&self, uniformlength: f64) -> ::windows::core::Result<Thickness>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThicknessHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IThicknessHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IThicknessHelperStaticsVtbl {
    pub const fn new<Impl: IThicknessHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThicknessHelperStaticsVtbl {
        unsafe extern "system" fn FromLengths<Impl: IThicknessHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: f64, top: f64, right: f64, bottom: f64, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromLengths(left, top, right, bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromUniformLength<Impl: IThicknessHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uniformlength: f64, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromUniformLength(uniformlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThicknessHelperStatics>, base.5, FromLengths::<Impl, OFFSET>, FromUniformLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerActionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerAction {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerAction";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerActionVtbl {
    pub const fn new<Impl: ITriggerActionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITriggerActionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITriggerAction>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerActionFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerActionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerActionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerActionFactoryVtbl {
    pub const fn new<Impl: ITriggerActionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITriggerActionFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITriggerActionFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerBase {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerBase";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerBaseVtbl {
    pub const fn new<Impl: ITriggerBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITriggerBaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITriggerBase>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerBaseFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerBaseFactoryVtbl {
    pub const fn new<Impl: ITriggerBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITriggerBaseFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITriggerBaseFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementImpl: Sized {
    fn DesiredSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn AllowDrop(&self) -> ::windows::core::Result<bool>;
    fn SetAllowDrop(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opacity(&self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn Clip(&self) -> ::windows::core::Result<Media::RectangleGeometry>;
    fn SetClip(&self, value: &::core::option::Option<Media::RectangleGeometry>) -> ::windows::core::Result<()>;
    fn RenderTransform(&self) -> ::windows::core::Result<Media::Transform>;
    fn SetRenderTransform(&self, value: &::core::option::Option<Media::Transform>) -> ::windows::core::Result<()>;
    fn Projection(&self) -> ::windows::core::Result<Media::Projection>;
    fn SetProjection(&self, value: &::core::option::Option<Media::Projection>) -> ::windows::core::Result<()>;
    fn RenderTransformOrigin(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetRenderTransformOrigin(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsHitTestVisible(&self) -> ::windows::core::Result<bool>;
    fn SetIsHitTestVisible(&self, value: bool) -> ::windows::core::Result<()>;
    fn Visibility(&self) -> ::windows::core::Result<Visibility>;
    fn SetVisibility(&self, value: Visibility) -> ::windows::core::Result<()>;
    fn RenderSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn UseLayoutRounding(&self) -> ::windows::core::Result<bool>;
    fn SetUseLayoutRounding(&self, value: bool) -> ::windows::core::Result<()>;
    fn Transitions(&self) -> ::windows::core::Result<Media::Animation::TransitionCollection>;
    fn SetTransitions(&self, value: &::core::option::Option<Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn CacheMode(&self) -> ::windows::core::Result<Media::CacheMode>;
    fn SetCacheMode(&self, value: &::core::option::Option<Media::CacheMode>) -> ::windows::core::Result<()>;
    fn IsTapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsTapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDoubleTapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDoubleTapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsRightTapEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsRightTapEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsHoldingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHoldingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ManipulationMode(&self) -> ::windows::core::Result<Input::ManipulationModes>;
    fn SetManipulationMode(&self, value: Input::ManipulationModes) -> ::windows::core::Result<()>;
    fn PointerCaptures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Input::Pointer>>;
    fn KeyUp(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragEnter(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragEnter(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragLeave(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragLeave(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragOver(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragOver(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Drop(&self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrop(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCanceled(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Tapped(&self, handler: &::core::option::Option<Input::TappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DoubleTapped(&self, handler: &::core::option::Option<Input::DoubleTappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDoubleTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Holding(&self, handler: &::core::option::Option<Input::HoldingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHolding(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RightTapped(&self, handler: &::core::option::Option<Input::RightTappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRightTapped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarting(&self, handler: &::core::option::Option<Input::ManipulationStartingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationInertiaStarting(&self, handler: &::core::option::Option<Input::ManipulationInertiaStartingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationInertiaStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&self, handler: &::core::option::Option<Input::ManipulationStartedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationDelta(&self, handler: &::core::option::Option<Input::ManipulationDeltaEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationDelta(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&self, handler: &::core::option::Option<Input::ManipulationCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Measure(&self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn Arrange(&self, finalrect: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn CapturePointer(&self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<bool>;
    fn ReleasePointerCapture(&self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<()>;
    fn ReleasePointerCaptures(&self) -> ::windows::core::Result<()>;
    fn AddHandler(&self, routedevent: &::core::option::Option<RoutedEvent>, handler: &::core::option::Option<::windows::core::IInspectable>, handledeventstoo: bool) -> ::windows::core::Result<()>;
    fn RemoveHandler(&self, routedevent: &::core::option::Option<RoutedEvent>, handler: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransformToVisual(&self, visual: &::core::option::Option<UIElement>) -> ::windows::core::Result<Media::GeneralTransform>;
    fn InvalidateMeasure(&self) -> ::windows::core::Result<()>;
    fn InvalidateArrange(&self) -> ::windows::core::Result<()>;
    fn UpdateLayout(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementVtbl {
    pub const fn new<Impl: IUIElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementVtbl {
        unsafe extern "system" fn DesiredSize<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowDrop<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowDrop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowDrop<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowDrop(value).into()
        }
        unsafe extern "system" fn Opacity<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Clip<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Clip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClip<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClip(&*(&value as *const <Media::RectangleGeometry as ::windows::core::Abi>::Abi as *const <Media::RectangleGeometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RenderTransform<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTransform<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRenderTransform(&*(&value as *const <Media::Transform as ::windows::core::Abi>::Abi as *const <Media::Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Projection<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Projection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjection<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProjection(&*(&value as *const <Media::Projection as ::windows::core::Abi>::Abi as *const <Media::Projection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RenderTransformOrigin<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderTransformOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTransformOrigin<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRenderTransformOrigin(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsHitTestVisible<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHitTestVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHitTestVisible<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsHitTestVisible(value).into()
        }
        unsafe extern "system" fn Visibility<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Visibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisibility<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Visibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVisibility(value).into()
        }
        unsafe extern "system" fn RenderSize<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseLayoutRounding<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseLayoutRounding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseLayoutRounding<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUseLayoutRounding(value).into()
        }
        unsafe extern "system" fn Transitions<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitions<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransitions(&*(&value as *const <Media::Animation::TransitionCollection as ::windows::core::Abi>::Abi as *const <Media::Animation::TransitionCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheMode<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CacheMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheMode<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCacheMode(&*(&value as *const <Media::CacheMode as ::windows::core::Abi>::Abi as *const <Media::CacheMode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsTapEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTapEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTapEnabled(value).into()
        }
        unsafe extern "system" fn IsDoubleTapEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDoubleTapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDoubleTapEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsDoubleTapEnabled(value).into()
        }
        unsafe extern "system" fn IsRightTapEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRightTapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRightTapEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsRightTapEnabled(value).into()
        }
        unsafe extern "system" fn IsHoldingEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHoldingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHoldingEnabled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsHoldingEnabled(value).into()
        }
        unsafe extern "system" fn ManipulationMode<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManipulationMode<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetManipulationMode(value).into()
        }
        unsafe extern "system" fn PointerCaptures<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerCaptures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveKeyUp(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyDown<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveKeyDown(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GotFocus<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragEnter<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragEnter(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragEnter<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragEnter(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragLeave<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragLeave(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragLeave<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragLeave(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragOver<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragOver(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragOver<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragOver(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Drop<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Drop(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDrop<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDrop(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressed<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerPressed(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoved<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerMoved(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerMoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleased<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerReleased(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerEntered<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerEntered(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerEntered(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExited<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerExited(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerExited(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerCaptureLost(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerCaptureLost(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerCanceled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerCanceled(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCanceled<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerWheelChanged(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePointerWheelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tapped<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Tapped(&*(&handler as *const <Input::TappedEventHandler as ::windows::core::Abi>::Abi as *const <Input::TappedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTapped<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DoubleTapped<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DoubleTapped(&*(&handler as *const <Input::DoubleTappedEventHandler as ::windows::core::Abi>::Abi as *const <Input::DoubleTappedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDoubleTapped<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDoubleTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Holding<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Holding(&*(&handler as *const <Input::HoldingEventHandler as ::windows::core::Abi>::Abi as *const <Input::HoldingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHolding<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHolding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RightTapped<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RightTapped(&*(&handler as *const <Input::RightTappedEventHandler as ::windows::core::Abi>::Abi as *const <Input::RightTappedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRightTapped<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRightTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarting<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationStarting(&*(&handler as *const <Input::ManipulationStartingEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationStartingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarting<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationInertiaStarting<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationInertiaStarting(&*(&handler as *const <Input::ManipulationInertiaStartingEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationInertiaStartingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationInertiaStarting<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationInertiaStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarted<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(&*(&handler as *const <Input::ManipulationStartedEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationStartedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationDelta<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationDelta(&*(&handler as *const <Input::ManipulationDeltaEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationDeltaEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationDelta<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationDelta(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(&*(&handler as *const <Input::ManipulationCompletedEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Measure<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availablesize: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Measure(&*(&availablesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Arrange<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finalrect: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Arrange(&*(&finalrect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CapturePointer<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CapturePointer(&*(&value as *const <Input::Pointer as ::windows::core::Abi>::Abi as *const <Input::Pointer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleasePointerCapture<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReleasePointerCapture(&*(&value as *const <Input::Pointer as ::windows::core::Abi>::Abi as *const <Input::Pointer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleasePointerCaptures<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReleasePointerCaptures().into()
        }
        unsafe extern "system" fn AddHandler<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, routedevent: ::windows::core::RawPtr, handler: *mut ::core::ffi::c_void, handledeventstoo: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddHandler(&*(&routedevent as *const <RoutedEvent as ::windows::core::Abi>::Abi as *const <RoutedEvent as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), handledeventstoo).into()
        }
        unsafe extern "system" fn RemoveHandler<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, routedevent: ::windows::core::RawPtr, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHandler(&*(&routedevent as *const <RoutedEvent as ::windows::core::Abi>::Abi as *const <RoutedEvent as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformToVisual<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformToVisual(&*(&visual as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateMeasure<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InvalidateMeasure().into()
        }
        unsafe extern "system" fn InvalidateArrange<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InvalidateArrange().into()
        }
        unsafe extern "system" fn UpdateLayout<Impl: IUIElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).UpdateLayout().into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElement>,
            base.5,
            DesiredSize::<Impl, OFFSET>,
            AllowDrop::<Impl, OFFSET>,
            SetAllowDrop::<Impl, OFFSET>,
            Opacity::<Impl, OFFSET>,
            SetOpacity::<Impl, OFFSET>,
            Clip::<Impl, OFFSET>,
            SetClip::<Impl, OFFSET>,
            RenderTransform::<Impl, OFFSET>,
            SetRenderTransform::<Impl, OFFSET>,
            Projection::<Impl, OFFSET>,
            SetProjection::<Impl, OFFSET>,
            RenderTransformOrigin::<Impl, OFFSET>,
            SetRenderTransformOrigin::<Impl, OFFSET>,
            IsHitTestVisible::<Impl, OFFSET>,
            SetIsHitTestVisible::<Impl, OFFSET>,
            Visibility::<Impl, OFFSET>,
            SetVisibility::<Impl, OFFSET>,
            RenderSize::<Impl, OFFSET>,
            UseLayoutRounding::<Impl, OFFSET>,
            SetUseLayoutRounding::<Impl, OFFSET>,
            Transitions::<Impl, OFFSET>,
            SetTransitions::<Impl, OFFSET>,
            CacheMode::<Impl, OFFSET>,
            SetCacheMode::<Impl, OFFSET>,
            IsTapEnabled::<Impl, OFFSET>,
            SetIsTapEnabled::<Impl, OFFSET>,
            IsDoubleTapEnabled::<Impl, OFFSET>,
            SetIsDoubleTapEnabled::<Impl, OFFSET>,
            IsRightTapEnabled::<Impl, OFFSET>,
            SetIsRightTapEnabled::<Impl, OFFSET>,
            IsHoldingEnabled::<Impl, OFFSET>,
            SetIsHoldingEnabled::<Impl, OFFSET>,
            ManipulationMode::<Impl, OFFSET>,
            SetManipulationMode::<Impl, OFFSET>,
            PointerCaptures::<Impl, OFFSET>,
            KeyUp::<Impl, OFFSET>,
            RemoveKeyUp::<Impl, OFFSET>,
            KeyDown::<Impl, OFFSET>,
            RemoveKeyDown::<Impl, OFFSET>,
            GotFocus::<Impl, OFFSET>,
            RemoveGotFocus::<Impl, OFFSET>,
            LostFocus::<Impl, OFFSET>,
            RemoveLostFocus::<Impl, OFFSET>,
            DragEnter::<Impl, OFFSET>,
            RemoveDragEnter::<Impl, OFFSET>,
            DragLeave::<Impl, OFFSET>,
            RemoveDragLeave::<Impl, OFFSET>,
            DragOver::<Impl, OFFSET>,
            RemoveDragOver::<Impl, OFFSET>,
            Drop::<Impl, OFFSET>,
            RemoveDrop::<Impl, OFFSET>,
            PointerPressed::<Impl, OFFSET>,
            RemovePointerPressed::<Impl, OFFSET>,
            PointerMoved::<Impl, OFFSET>,
            RemovePointerMoved::<Impl, OFFSET>,
            PointerReleased::<Impl, OFFSET>,
            RemovePointerReleased::<Impl, OFFSET>,
            PointerEntered::<Impl, OFFSET>,
            RemovePointerEntered::<Impl, OFFSET>,
            PointerExited::<Impl, OFFSET>,
            RemovePointerExited::<Impl, OFFSET>,
            PointerCaptureLost::<Impl, OFFSET>,
            RemovePointerCaptureLost::<Impl, OFFSET>,
            PointerCanceled::<Impl, OFFSET>,
            RemovePointerCanceled::<Impl, OFFSET>,
            PointerWheelChanged::<Impl, OFFSET>,
            RemovePointerWheelChanged::<Impl, OFFSET>,
            Tapped::<Impl, OFFSET>,
            RemoveTapped::<Impl, OFFSET>,
            DoubleTapped::<Impl, OFFSET>,
            RemoveDoubleTapped::<Impl, OFFSET>,
            Holding::<Impl, OFFSET>,
            RemoveHolding::<Impl, OFFSET>,
            RightTapped::<Impl, OFFSET>,
            RemoveRightTapped::<Impl, OFFSET>,
            ManipulationStarting::<Impl, OFFSET>,
            RemoveManipulationStarting::<Impl, OFFSET>,
            ManipulationInertiaStarting::<Impl, OFFSET>,
            RemoveManipulationInertiaStarting::<Impl, OFFSET>,
            ManipulationStarted::<Impl, OFFSET>,
            RemoveManipulationStarted::<Impl, OFFSET>,
            ManipulationDelta::<Impl, OFFSET>,
            RemoveManipulationDelta::<Impl, OFFSET>,
            ManipulationCompleted::<Impl, OFFSET>,
            RemoveManipulationCompleted::<Impl, OFFSET>,
            Measure::<Impl, OFFSET>,
            Arrange::<Impl, OFFSET>,
            CapturePointer::<Impl, OFFSET>,
            ReleasePointerCapture::<Impl, OFFSET>,
            ReleasePointerCaptures::<Impl, OFFSET>,
            AddHandler::<Impl, OFFSET>,
            RemoveHandler::<Impl, OFFSET>,
            TransformToVisual::<Impl, OFFSET>,
            InvalidateMeasure::<Impl, OFFSET>,
            InvalidateArrange::<Impl, OFFSET>,
            UpdateLayout::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement10Impl: Sized {
    fn ActualOffset(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn ActualSize(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn XamlRoot(&self) -> ::windows::core::Result<XamlRoot>;
    fn SetXamlRoot(&self, value: &::core::option::Option<XamlRoot>) -> ::windows::core::Result<()>;
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
    fn Shadow(&self) -> ::windows::core::Result<Media::Shadow>;
    fn SetShadow(&self, value: &::core::option::Option<Media::Shadow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement10 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement10";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement10Vtbl {
    pub const fn new<Impl: IUIElement10Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement10Vtbl {
        unsafe extern "system" fn ActualOffset<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualSize<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XamlRoot<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XamlRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXamlRoot<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXamlRoot(&*(&value as *const <XamlRoot as ::windows::core::Abi>::Abi as *const <XamlRoot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UIContext<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shadow<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Impl: IUIElement10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShadow(&*(&value as *const <Media::Shadow as ::windows::core::Abi>::Abi as *const <Media::Shadow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElement10>, base.5, ActualOffset::<Impl, OFFSET>, ActualSize::<Impl, OFFSET>, XamlRoot::<Impl, OFFSET>, SetXamlRoot::<Impl, OFFSET>, UIContext::<Impl, OFFSET>, Shadow::<Impl, OFFSET>, SetShadow::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement2Impl: Sized {
    fn CompositeMode(&self) -> ::windows::core::Result<Media::ElementCompositeMode>;
    fn SetCompositeMode(&self, value: Media::ElementCompositeMode) -> ::windows::core::Result<()>;
    fn CancelDirectManipulations(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement2";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement2Vtbl {
    pub const fn new<Impl: IUIElement2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement2Vtbl {
        unsafe extern "system" fn CompositeMode<Impl: IUIElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Media::ElementCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IUIElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Media::ElementCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCompositeMode(value).into()
        }
        unsafe extern "system" fn CancelDirectManipulations<Impl: IUIElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelDirectManipulations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElement2>, base.5, CompositeMode::<Impl, OFFSET>, SetCompositeMode::<Impl, OFFSET>, CancelDirectManipulations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement3Impl: Sized {
    fn Transform3D(&self) -> ::windows::core::Result<Media::Media3D::Transform3D>;
    fn SetTransform3D(&self, value: &::core::option::Option<Media::Media3D::Transform3D>) -> ::windows::core::Result<()>;
    fn CanDrag(&self) -> ::windows::core::Result<bool>;
    fn SetCanDrag(&self, value: bool) -> ::windows::core::Result<()>;
    fn DragStarting(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragStarting(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DropCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDropCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartDragAsync(&self, pointerpoint: &::core::option::Option<super::Input::PointerPoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackageOperation>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement3";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement3Vtbl {
    pub const fn new<Impl: IUIElement3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement3Vtbl {
        unsafe extern "system" fn Transform3D<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform3D<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransform3D(&*(&value as *const <Media::Media3D::Transform3D as ::windows::core::Abi>::Abi as *const <Media::Media3D::Transform3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanDrag<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanDrag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanDrag<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanDrag(value).into()
        }
        unsafe extern "system" fn DragStarting<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragStarting<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DropCompleted<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDropCompleted<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDropCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDragAsync<Impl: IUIElement3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StartDragAsync(&*(&pointerpoint as *const <super::Input::PointerPoint as ::windows::core::Abi>::Abi as *const <super::Input::PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElement3>, base.5, Transform3D::<Impl, OFFSET>, SetTransform3D::<Impl, OFFSET>, CanDrag::<Impl, OFFSET>, SetCanDrag::<Impl, OFFSET>, DragStarting::<Impl, OFFSET>, RemoveDragStarting::<Impl, OFFSET>, DropCompleted::<Impl, OFFSET>, RemoveDropCompleted::<Impl, OFFSET>, StartDragAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement4Impl: Sized {
    fn ContextFlyout(&self) -> ::windows::core::Result<Controls::Primitives::FlyoutBase>;
    fn SetContextFlyout(&self, value: &::core::option::Option<Controls::Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn ExitDisplayModeOnAccessKeyInvoked(&self) -> ::windows::core::Result<bool>;
    fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAccessKeyScope(&self) -> ::windows::core::Result<bool>;
    fn SetIsAccessKeyScope(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKeyScopeOwner(&self) -> ::windows::core::Result<DependencyObject>;
    fn SetAccessKeyScopeOwner(&self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContextRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextCanceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayDismissed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayDismissed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyInvoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyInvoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement4";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement4Vtbl {
    pub const fn new<Impl: IUIElement4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement4Vtbl {
        unsafe extern "system" fn ContextFlyout<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContextFlyout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContextFlyout<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContextFlyout(&*(&value as *const <Controls::Primitives::FlyoutBase as ::windows::core::Abi>::Abi as *const <Controls::Primitives::FlyoutBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvoked<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitDisplayModeOnAccessKeyInvoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitDisplayModeOnAccessKeyInvoked<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExitDisplayModeOnAccessKeyInvoked(value).into()
        }
        unsafe extern "system" fn IsAccessKeyScope<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAccessKeyScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccessKeyScope<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsAccessKeyScope(value).into()
        }
        unsafe extern "system" fn AccessKeyScopeOwner<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyScopeOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKeyScopeOwner<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAccessKeyScopeOwner(&*(&value as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKey<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKey<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContextRequested<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContextRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContextRequested<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveContextRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContextCanceled<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContextCanceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContextCanceled<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveContextCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyDisplayRequested<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyDisplayRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayRequested<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyDisplayRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyDisplayDismissed<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyDisplayDismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayDismissed<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyDisplayDismissed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyInvoked<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyInvoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyInvoked<Impl: IUIElement4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyInvoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElement4>,
            base.5,
            ContextFlyout::<Impl, OFFSET>,
            SetContextFlyout::<Impl, OFFSET>,
            ExitDisplayModeOnAccessKeyInvoked::<Impl, OFFSET>,
            SetExitDisplayModeOnAccessKeyInvoked::<Impl, OFFSET>,
            IsAccessKeyScope::<Impl, OFFSET>,
            SetIsAccessKeyScope::<Impl, OFFSET>,
            AccessKeyScopeOwner::<Impl, OFFSET>,
            SetAccessKeyScopeOwner::<Impl, OFFSET>,
            AccessKey::<Impl, OFFSET>,
            SetAccessKey::<Impl, OFFSET>,
            ContextRequested::<Impl, OFFSET>,
            RemoveContextRequested::<Impl, OFFSET>,
            ContextCanceled::<Impl, OFFSET>,
            RemoveContextCanceled::<Impl, OFFSET>,
            AccessKeyDisplayRequested::<Impl, OFFSET>,
            RemoveAccessKeyDisplayRequested::<Impl, OFFSET>,
            AccessKeyDisplayDismissed::<Impl, OFFSET>,
            RemoveAccessKeyDisplayDismissed::<Impl, OFFSET>,
            AccessKeyInvoked::<Impl, OFFSET>,
            RemoveAccessKeyInvoked::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement5Impl: Sized {
    fn Lights(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Media::XamlLight>>;
    fn KeyTipPlacementMode(&self) -> ::windows::core::Result<Input::KeyTipPlacementMode>;
    fn SetKeyTipPlacementMode(&self, value: Input::KeyTipPlacementMode) -> ::windows::core::Result<()>;
    fn KeyTipHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTipVerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetKeyTipVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn XYFocusKeyboardNavigation(&self) -> ::windows::core::Result<Input::XYFocusKeyboardNavigationMode>;
    fn SetXYFocusKeyboardNavigation(&self, value: Input::XYFocusKeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn XYFocusUpNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn HighContrastAdjustment(&self) -> ::windows::core::Result<ElementHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&self, value: ElementHighContrastAdjustment) -> ::windows::core::Result<()>;
    fn TabFocusNavigation(&self) -> ::windows::core::Result<Input::KeyboardNavigationMode>;
    fn SetTabFocusNavigation(&self, value: Input::KeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn GettingFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGettingFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LosingFocus(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLosingFocus(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NoFocusCandidateFound(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNoFocusCandidateFound(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartBringIntoView(&self) -> ::windows::core::Result<()>;
    fn StartBringIntoViewWithOptions(&self, options: &::core::option::Option<BringIntoViewOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement5 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement5";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement5Vtbl {
    pub const fn new<Impl: IUIElement5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement5Vtbl {
        unsafe extern "system" fn Lights<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lights() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipPlacementMode<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::KeyTipPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipPlacementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipPlacementMode<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::KeyTipPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTipPlacementMode(value).into()
        }
        unsafe extern "system" fn KeyTipHorizontalOffset<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipHorizontalOffset<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTipHorizontalOffset(value).into()
        }
        unsafe extern "system" fn KeyTipVerticalOffset<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipVerticalOffset<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTipVerticalOffset(value).into()
        }
        unsafe extern "system" fn XYFocusKeyboardNavigation<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusKeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusKeyboardNavigation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusKeyboardNavigation<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusKeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXYFocusKeyboardNavigation(value).into()
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusUpNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXYFocusUpNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusDownNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXYFocusDownNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusLeftNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXYFocusLeftNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusRightNavigationStrategy<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXYFocusRightNavigationStrategy(value).into()
        }
        unsafe extern "system" fn HighContrastAdjustment<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ElementHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighContrastAdjustment<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ElementHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHighContrastAdjustment(value).into()
        }
        unsafe extern "system" fn TabFocusNavigation<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::KeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TabFocusNavigation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTabFocusNavigation<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::KeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTabFocusNavigation(value).into()
        }
        unsafe extern "system" fn GettingFocus<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GettingFocus(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGettingFocus<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGettingFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LosingFocus<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LosingFocus(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLosingFocus<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLosingFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NoFocusCandidateFound<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NoFocusCandidateFound(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNoFocusCandidateFound<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveNoFocusCandidateFound(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartBringIntoView<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartBringIntoView().into()
        }
        unsafe extern "system" fn StartBringIntoViewWithOptions<Impl: IUIElement5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartBringIntoViewWithOptions(&*(&options as *const <BringIntoViewOptions as ::windows::core::Abi>::Abi as *const <BringIntoViewOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElement5>,
            base.5,
            Lights::<Impl, OFFSET>,
            KeyTipPlacementMode::<Impl, OFFSET>,
            SetKeyTipPlacementMode::<Impl, OFFSET>,
            KeyTipHorizontalOffset::<Impl, OFFSET>,
            SetKeyTipHorizontalOffset::<Impl, OFFSET>,
            KeyTipVerticalOffset::<Impl, OFFSET>,
            SetKeyTipVerticalOffset::<Impl, OFFSET>,
            XYFocusKeyboardNavigation::<Impl, OFFSET>,
            SetXYFocusKeyboardNavigation::<Impl, OFFSET>,
            XYFocusUpNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusUpNavigationStrategy::<Impl, OFFSET>,
            XYFocusDownNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusDownNavigationStrategy::<Impl, OFFSET>,
            XYFocusLeftNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusLeftNavigationStrategy::<Impl, OFFSET>,
            XYFocusRightNavigationStrategy::<Impl, OFFSET>,
            SetXYFocusRightNavigationStrategy::<Impl, OFFSET>,
            HighContrastAdjustment::<Impl, OFFSET>,
            SetHighContrastAdjustment::<Impl, OFFSET>,
            TabFocusNavigation::<Impl, OFFSET>,
            SetTabFocusNavigation::<Impl, OFFSET>,
            GettingFocus::<Impl, OFFSET>,
            RemoveGettingFocus::<Impl, OFFSET>,
            LosingFocus::<Impl, OFFSET>,
            RemoveLosingFocus::<Impl, OFFSET>,
            NoFocusCandidateFound::<Impl, OFFSET>,
            RemoveNoFocusCandidateFound::<Impl, OFFSET>,
            StartBringIntoView::<Impl, OFFSET>,
            StartBringIntoViewWithOptions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement7Impl: Sized {
    fn KeyboardAccelerators(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Input::KeyboardAccelerator>>;
    fn CharacterReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProcessKeyboardAccelerators(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessKeyboardAccelerators(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewKeyDown(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewKeyDown(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewKeyUp(&self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewKeyUp(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryInvokeKeyboardAccelerator(&self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement7";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement7Vtbl {
    pub const fn new<Impl: IUIElement7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement7Vtbl {
        unsafe extern "system" fn KeyboardAccelerators<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAccelerators() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterReceived<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCharacterReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessKeyboardAccelerators<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessKeyboardAccelerators(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProcessKeyboardAccelerators<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveProcessKeyboardAccelerators(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewKeyDown<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewKeyDown(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewKeyDown<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePreviewKeyDown(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewKeyUp<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewKeyUp(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewKeyUp<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemovePreviewKeyUp(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryInvokeKeyboardAccelerator<Impl: IUIElement7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).TryInvokeKeyboardAccelerator(&*(&args as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElement7>,
            base.5,
            KeyboardAccelerators::<Impl, OFFSET>,
            CharacterReceived::<Impl, OFFSET>,
            RemoveCharacterReceived::<Impl, OFFSET>,
            ProcessKeyboardAccelerators::<Impl, OFFSET>,
            RemoveProcessKeyboardAccelerators::<Impl, OFFSET>,
            PreviewKeyDown::<Impl, OFFSET>,
            RemovePreviewKeyDown::<Impl, OFFSET>,
            PreviewKeyUp::<Impl, OFFSET>,
            RemovePreviewKeyUp::<Impl, OFFSET>,
            TryInvokeKeyboardAccelerator::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement8Impl: Sized {
    fn KeyTipTarget(&self) -> ::windows::core::Result<DependencyObject>;
    fn SetKeyTipTarget(&self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyboardAcceleratorPlacementTarget(&self) -> ::windows::core::Result<DependencyObject>;
    fn SetKeyboardAcceleratorPlacementTarget(&self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyboardAcceleratorPlacementMode(&self) -> ::windows::core::Result<Input::KeyboardAcceleratorPlacementMode>;
    fn SetKeyboardAcceleratorPlacementMode(&self, value: Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::Result<()>;
    fn BringIntoViewRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBringIntoViewRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement8";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement8Vtbl {
    pub const fn new<Impl: IUIElement8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement8Vtbl {
        unsafe extern "system" fn KeyTipTarget<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipTarget<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyTipTarget(&*(&value as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementTarget<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyboardAcceleratorPlacementTarget<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyboardAcceleratorPlacementTarget(&*(&value as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementMode<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyboardAcceleratorPlacementMode<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKeyboardAcceleratorPlacementMode(value).into()
        }
        unsafe extern "system" fn BringIntoViewRequested<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BringIntoViewRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBringIntoViewRequested<Impl: IUIElement8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveBringIntoViewRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElement8>, base.5, KeyTipTarget::<Impl, OFFSET>, SetKeyTipTarget::<Impl, OFFSET>, KeyboardAcceleratorPlacementTarget::<Impl, OFFSET>, SetKeyboardAcceleratorPlacementTarget::<Impl, OFFSET>, KeyboardAcceleratorPlacementMode::<Impl, OFFSET>, SetKeyboardAcceleratorPlacementMode::<Impl, OFFSET>, BringIntoViewRequested::<Impl, OFFSET>, RemoveBringIntoViewRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElement9Impl: Sized {
    fn CanBeScrollAnchor(&self) -> ::windows::core::Result<bool>;
    fn SetCanBeScrollAnchor(&self, value: bool) -> ::windows::core::Result<()>;
    fn OpacityTransition(&self) -> ::windows::core::Result<ScalarTransition>;
    fn SetOpacityTransition(&self, value: &::core::option::Option<ScalarTransition>) -> ::windows::core::Result<()>;
    fn Translation(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetTranslation(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn TranslationTransition(&self) -> ::windows::core::Result<Vector3Transition>;
    fn SetTranslationTransition(&self, value: &::core::option::Option<Vector3Transition>) -> ::windows::core::Result<()>;
    fn Rotation(&self) -> ::windows::core::Result<f32>;
    fn SetRotation(&self, value: f32) -> ::windows::core::Result<()>;
    fn RotationTransition(&self) -> ::windows::core::Result<ScalarTransition>;
    fn SetRotationTransition(&self, value: &::core::option::Option<ScalarTransition>) -> ::windows::core::Result<()>;
    fn Scale(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn ScaleTransition(&self) -> ::windows::core::Result<Vector3Transition>;
    fn SetScaleTransition(&self, value: &::core::option::Option<Vector3Transition>) -> ::windows::core::Result<()>;
    fn TransformMatrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetTransformMatrix(&self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn CenterPoint(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetCenterPoint(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RotationAxis(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn StartAnimation(&self, animation: &::core::option::Option<super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn StopAnimation(&self, animation: &::core::option::Option<super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElement9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement9";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElement9Vtbl {
    pub const fn new<Impl: IUIElement9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElement9Vtbl {
        unsafe extern "system" fn CanBeScrollAnchor<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanBeScrollAnchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanBeScrollAnchor<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanBeScrollAnchor(value).into()
        }
        unsafe extern "system" fn OpacityTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpacityTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOpacityTransition(&*(&value as *const <ScalarTransition as ::windows::core::Abi>::Abi as *const <ScalarTransition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Translation<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Translation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslation<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslation(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslationTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslationTransition(&*(&value as *const <Vector3Transition as ::windows::core::Abi>::Abi as *const <Vector3Transition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rotation<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn RotationTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationTransition(&*(&value as *const <ScalarTransition as ::windows::core::Abi>::Abi as *const <ScalarTransition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scale<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScaleTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScaleTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleTransition<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScaleTransition(&*(&value as *const <Vector3Transition as ::windows::core::Abi>::Abi as *const <Vector3Transition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAxis<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationAxis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAxis<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationAxis(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAnimation<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartAnimation(&*(&animation as *const <super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAnimation<Impl: IUIElement9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StopAnimation(&*(&animation as *const <super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElement9>,
            base.5,
            CanBeScrollAnchor::<Impl, OFFSET>,
            SetCanBeScrollAnchor::<Impl, OFFSET>,
            OpacityTransition::<Impl, OFFSET>,
            SetOpacityTransition::<Impl, OFFSET>,
            Translation::<Impl, OFFSET>,
            SetTranslation::<Impl, OFFSET>,
            TranslationTransition::<Impl, OFFSET>,
            SetTranslationTransition::<Impl, OFFSET>,
            Rotation::<Impl, OFFSET>,
            SetRotation::<Impl, OFFSET>,
            RotationTransition::<Impl, OFFSET>,
            SetRotationTransition::<Impl, OFFSET>,
            Scale::<Impl, OFFSET>,
            SetScale::<Impl, OFFSET>,
            ScaleTransition::<Impl, OFFSET>,
            SetScaleTransition::<Impl, OFFSET>,
            TransformMatrix::<Impl, OFFSET>,
            SetTransformMatrix::<Impl, OFFSET>,
            CenterPoint::<Impl, OFFSET>,
            SetCenterPoint::<Impl, OFFSET>,
            RotationAxis::<Impl, OFFSET>,
            SetRotationAxis::<Impl, OFFSET>,
            StartAnimation::<Impl, OFFSET>,
            StopAnimation::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementFactoryVtbl {
    pub const fn new<Impl: IUIElementFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverridesImpl: Sized {
    fn OnCreateAutomationPeer(&self) -> ::windows::core::Result<Automation::Peers::AutomationPeer>;
    fn OnDisconnectVisualChildren(&self) -> ::windows::core::Result<()>;
    fn FindSubElementsForTouchTargeting(&self, point: &super::super::Foundation::Point, boundingrect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Point>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementOverridesVtbl {
    pub const fn new<Impl: IUIElementOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementOverridesVtbl {
        unsafe extern "system" fn OnCreateAutomationPeer<Impl: IUIElementOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCreateAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisconnectVisualChildren<Impl: IUIElementOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnDisconnectVisualChildren().into()
        }
        unsafe extern "system" fn FindSubElementsForTouchTargeting<Impl: IUIElementOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, boundingrect: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindSubElementsForTouchTargeting(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&boundingrect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementOverrides>, base.5, OnCreateAutomationPeer::<Impl, OFFSET>, OnDisconnectVisualChildren::<Impl, OFFSET>, FindSubElementsForTouchTargeting::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverrides7Impl: Sized {
    fn GetChildrenInTabFocusOrder(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<DependencyObject>>;
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementOverrides7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides7";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementOverrides7Vtbl {
    pub const fn new<Impl: IUIElementOverrides7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementOverrides7Vtbl {
        unsafe extern "system" fn GetChildrenInTabFocusOrder<Impl: IUIElementOverrides7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildrenInTabFocusOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Impl: IUIElementOverrides7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(&*(&args as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementOverrides7>, base.5, GetChildrenInTabFocusOrder::<Impl, OFFSET>, OnProcessKeyboardAccelerators::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverrides8Impl: Sized {
    fn OnKeyboardAcceleratorInvoked(&self, args: &::core::option::Option<Input::KeyboardAcceleratorInvokedEventArgs>) -> ::windows::core::Result<()>;
    fn OnBringIntoViewRequested(&self, e: &::core::option::Option<BringIntoViewRequestedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides8";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementOverrides8Vtbl {
    pub const fn new<Impl: IUIElementOverrides8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementOverrides8Vtbl {
        unsafe extern "system" fn OnKeyboardAcceleratorInvoked<Impl: IUIElementOverrides8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnKeyboardAcceleratorInvoked(&*(&args as *const <Input::KeyboardAcceleratorInvokedEventArgs as ::windows::core::Abi>::Abi as *const <Input::KeyboardAcceleratorInvokedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnBringIntoViewRequested<Impl: IUIElementOverrides8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnBringIntoViewRequested(&*(&e as *const <BringIntoViewRequestedEventArgs as ::windows::core::Abi>::Abi as *const <BringIntoViewRequestedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementOverrides8>, base.5, OnKeyboardAcceleratorInvoked::<Impl, OFFSET>, OnBringIntoViewRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementOverrides9Impl: Sized {
    fn PopulatePropertyInfoOverride(&self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides9";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementOverrides9Vtbl {
    pub const fn new<Impl: IUIElementOverrides9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementOverrides9Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Impl: IUIElementOverrides9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfoOverride(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animationpropertyinfo as *const <super::Composition::AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <super::Composition::AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementOverrides9>, base.5, PopulatePropertyInfoOverride::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStaticsImpl: Sized {
    fn KeyDownEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn KeyUpEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerEnteredEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerPressedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerMovedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerReleasedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerExitedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerCaptureLostEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerCanceledEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerWheelChangedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn TappedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DoubleTappedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn HoldingEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn RightTappedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationStartingEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationInertiaStartingEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationStartedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationDeltaEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationCompletedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DragEnterEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DragLeaveEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DragOverEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn DropEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn AllowDropProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn OpacityProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ClipProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn RenderTransformProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ProjectionProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn RenderTransformOriginProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsHitTestVisibleProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn VisibilityProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn UseLayoutRoundingProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn TransitionsProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn CacheModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsTapEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsDoubleTapEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsRightTapEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsHoldingEnabledProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ManipulationModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn PointerCapturesProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStaticsVtbl {
    pub const fn new<Impl: IUIElementStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStaticsVtbl {
        unsafe extern "system" fn KeyDownEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyDownEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUpEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyUpEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerEnteredEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerEnteredEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerPressedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerPressedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerMovedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerMovedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerReleasedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerReleasedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerExitedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerExitedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCaptureLostEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerCaptureLostEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCanceledEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerCanceledEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerWheelChangedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerWheelChangedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TappedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TappedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoubleTappedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DoubleTappedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldingEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldingEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RightTappedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RightTappedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationStartingEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationStartingEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationInertiaStartingEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationInertiaStartingEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationStartedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationStartedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationDeltaEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationDeltaEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationCompletedEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationCompletedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragEnterEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragEnterEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragLeaveEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragLeaveEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOverEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragOverEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEvent<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowDropProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowDropProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpacityProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClipProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTransformProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderTransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProjectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTransformOriginProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RenderTransformOriginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHitTestVisibleProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHitTestVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibilityProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibilityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseLayoutRoundingProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UseLayoutRoundingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitionsProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransitionsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheModeProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CacheModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTapEnabledProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTapEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleTapEnabledProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDoubleTapEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRightTapEnabledProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRightTapEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHoldingEnabledProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHoldingEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationModeProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ManipulationModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCapturesProperty<Impl: IUIElementStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerCapturesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElementStatics>,
            base.5,
            KeyDownEvent::<Impl, OFFSET>,
            KeyUpEvent::<Impl, OFFSET>,
            PointerEnteredEvent::<Impl, OFFSET>,
            PointerPressedEvent::<Impl, OFFSET>,
            PointerMovedEvent::<Impl, OFFSET>,
            PointerReleasedEvent::<Impl, OFFSET>,
            PointerExitedEvent::<Impl, OFFSET>,
            PointerCaptureLostEvent::<Impl, OFFSET>,
            PointerCanceledEvent::<Impl, OFFSET>,
            PointerWheelChangedEvent::<Impl, OFFSET>,
            TappedEvent::<Impl, OFFSET>,
            DoubleTappedEvent::<Impl, OFFSET>,
            HoldingEvent::<Impl, OFFSET>,
            RightTappedEvent::<Impl, OFFSET>,
            ManipulationStartingEvent::<Impl, OFFSET>,
            ManipulationInertiaStartingEvent::<Impl, OFFSET>,
            ManipulationStartedEvent::<Impl, OFFSET>,
            ManipulationDeltaEvent::<Impl, OFFSET>,
            ManipulationCompletedEvent::<Impl, OFFSET>,
            DragEnterEvent::<Impl, OFFSET>,
            DragLeaveEvent::<Impl, OFFSET>,
            DragOverEvent::<Impl, OFFSET>,
            DropEvent::<Impl, OFFSET>,
            AllowDropProperty::<Impl, OFFSET>,
            OpacityProperty::<Impl, OFFSET>,
            ClipProperty::<Impl, OFFSET>,
            RenderTransformProperty::<Impl, OFFSET>,
            ProjectionProperty::<Impl, OFFSET>,
            RenderTransformOriginProperty::<Impl, OFFSET>,
            IsHitTestVisibleProperty::<Impl, OFFSET>,
            VisibilityProperty::<Impl, OFFSET>,
            UseLayoutRoundingProperty::<Impl, OFFSET>,
            TransitionsProperty::<Impl, OFFSET>,
            CacheModeProperty::<Impl, OFFSET>,
            IsTapEnabledProperty::<Impl, OFFSET>,
            IsDoubleTapEnabledProperty::<Impl, OFFSET>,
            IsRightTapEnabledProperty::<Impl, OFFSET>,
            IsHoldingEnabledProperty::<Impl, OFFSET>,
            ManipulationModeProperty::<Impl, OFFSET>,
            PointerCapturesProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics10Impl: Sized {
    fn ShadowProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics10 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics10";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics10Vtbl {
    pub const fn new<Impl: IUIElementStatics10Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics10Vtbl {
        unsafe extern "system" fn ShadowProperty<Impl: IUIElementStatics10Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShadowProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics10>, base.5, ShadowProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics2Impl: Sized {
    fn CompositeModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics2Vtbl {
    pub const fn new<Impl: IUIElementStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics2Vtbl {
        unsafe extern "system" fn CompositeModeProperty<Impl: IUIElementStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompositeModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics2>, base.5, CompositeModeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics3Impl: Sized {
    fn Transform3DProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn CanDragProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn TryStartDirectManipulation(&self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics3Vtbl {
    pub const fn new<Impl: IUIElementStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics3Vtbl {
        unsafe extern "system" fn Transform3DProperty<Impl: IUIElementStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transform3DProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDragProperty<Impl: IUIElementStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanDragProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStartDirectManipulation<Impl: IUIElementStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryStartDirectManipulation(&*(&value as *const <Input::Pointer as ::windows::core::Abi>::Abi as *const <Input::Pointer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics3>, base.5, Transform3DProperty::<Impl, OFFSET>, CanDragProperty::<Impl, OFFSET>, TryStartDirectManipulation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics4Impl: Sized {
    fn ContextFlyoutProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn ExitDisplayModeOnAccessKeyInvokedProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn IsAccessKeyScopeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn AccessKeyScopeOwnerProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics4Vtbl {
    pub const fn new<Impl: IUIElementStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics4Vtbl {
        unsafe extern "system" fn ContextFlyoutProperty<Impl: IUIElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContextFlyoutProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvokedProperty<Impl: IUIElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExitDisplayModeOnAccessKeyInvokedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAccessKeyScopeProperty<Impl: IUIElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAccessKeyScopeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyScopeOwnerProperty<Impl: IUIElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyScopeOwnerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IUIElementStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics4>, base.5, ContextFlyoutProperty::<Impl, OFFSET>, ExitDisplayModeOnAccessKeyInvokedProperty::<Impl, OFFSET>, IsAccessKeyScopeProperty::<Impl, OFFSET>, AccessKeyScopeOwnerProperty::<Impl, OFFSET>, AccessKeyProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics5Impl: Sized {
    fn LightsProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipPlacementModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipHorizontalOffsetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipVerticalOffsetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusKeyboardNavigationProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn HighContrastAdjustmentProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn TabFocusNavigationProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics5Vtbl {
    pub const fn new<Impl: IUIElementStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics5Vtbl {
        unsafe extern "system" fn LightsProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LightsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipPlacementModeProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipPlacementModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipHorizontalOffsetProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipVerticalOffsetProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusKeyboardNavigationProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusKeyboardNavigationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategyProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategyProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategyProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategyProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighContrastAdjustmentProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TabFocusNavigationProperty<Impl: IUIElementStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TabFocusNavigationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IUIElementStatics5>,
            base.5,
            LightsProperty::<Impl, OFFSET>,
            KeyTipPlacementModeProperty::<Impl, OFFSET>,
            KeyTipHorizontalOffsetProperty::<Impl, OFFSET>,
            KeyTipVerticalOffsetProperty::<Impl, OFFSET>,
            XYFocusKeyboardNavigationProperty::<Impl, OFFSET>,
            XYFocusUpNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusDownNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusLeftNavigationStrategyProperty::<Impl, OFFSET>,
            XYFocusRightNavigationStrategyProperty::<Impl, OFFSET>,
            HighContrastAdjustmentProperty::<Impl, OFFSET>,
            TabFocusNavigationProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics6Impl: Sized {
    fn GettingFocusEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn LosingFocusEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn NoFocusCandidateFoundEvent(&self) -> ::windows::core::Result<RoutedEvent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics6Vtbl {
    pub const fn new<Impl: IUIElementStatics6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics6Vtbl {
        unsafe extern "system" fn GettingFocusEvent<Impl: IUIElementStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GettingFocusEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LosingFocusEvent<Impl: IUIElementStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LosingFocusEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoFocusCandidateFoundEvent<Impl: IUIElementStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NoFocusCandidateFoundEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics6>, base.5, GettingFocusEvent::<Impl, OFFSET>, LosingFocusEvent::<Impl, OFFSET>, NoFocusCandidateFoundEvent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics7Impl: Sized {
    fn PreviewKeyDownEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn CharacterReceivedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn PreviewKeyUpEvent(&self) -> ::windows::core::Result<RoutedEvent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics7Vtbl {
    pub const fn new<Impl: IUIElementStatics7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics7Vtbl {
        unsafe extern "system" fn PreviewKeyDownEvent<Impl: IUIElementStatics7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewKeyDownEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterReceivedEvent<Impl: IUIElementStatics7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CharacterReceivedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviewKeyUpEvent<Impl: IUIElementStatics7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreviewKeyUpEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics7>, base.5, PreviewKeyDownEvent::<Impl, OFFSET>, CharacterReceivedEvent::<Impl, OFFSET>, PreviewKeyUpEvent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics8Impl: Sized {
    fn BringIntoViewRequestedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn ContextRequestedEvent(&self) -> ::windows::core::Result<RoutedEvent>;
    fn KeyTipTargetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyboardAcceleratorPlacementTargetProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyboardAcceleratorPlacementModeProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn RegisterAsScrollPort(&self, element: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics8Vtbl {
    pub const fn new<Impl: IUIElementStatics8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics8Vtbl {
        unsafe extern "system" fn BringIntoViewRequestedEvent<Impl: IUIElementStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BringIntoViewRequestedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextRequestedEvent<Impl: IUIElementStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContextRequestedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipTargetProperty<Impl: IUIElementStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyTipTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementTargetProperty<Impl: IUIElementStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementModeProperty<Impl: IUIElementStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAsScrollPort<Impl: IUIElementStatics8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RegisterAsScrollPort(&*(&element as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics8>, base.5, BringIntoViewRequestedEvent::<Impl, OFFSET>, ContextRequestedEvent::<Impl, OFFSET>, KeyTipTargetProperty::<Impl, OFFSET>, KeyboardAcceleratorPlacementTargetProperty::<Impl, OFFSET>, KeyboardAcceleratorPlacementModeProperty::<Impl, OFFSET>, RegisterAsScrollPort::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics9Impl: Sized {
    fn CanBeScrollAnchorProperty(&self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics9";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics9Vtbl {
    pub const fn new<Impl: IUIElementStatics9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementStatics9Vtbl {
        unsafe extern "system" fn CanBeScrollAnchorProperty<Impl: IUIElementStatics9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanBeScrollAnchorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementStatics9>, base.5, CanBeScrollAnchorProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementWeakCollectionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementWeakCollection {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementWeakCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementWeakCollectionVtbl {
    pub const fn new<Impl: IUIElementWeakCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementWeakCollectionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementWeakCollection>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementWeakCollectionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<UIElementWeakCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementWeakCollectionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementWeakCollectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementWeakCollectionFactoryVtbl {
    pub const fn new<Impl: IUIElementWeakCollectionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIElementWeakCollectionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IUIElementWeakCollectionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIElementWeakCollectionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledExceptionEventArgsImpl: Sized {
    fn Exception(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnhandledExceptionEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IUnhandledExceptionEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUnhandledExceptionEventArgsVtbl {
    pub const fn new<Impl: IUnhandledExceptionEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUnhandledExceptionEventArgsVtbl {
        unsafe extern "system" fn Exception<Impl: IUnhandledExceptionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Exception() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IUnhandledExceptionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IUnhandledExceptionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IUnhandledExceptionEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUnhandledExceptionEventArgs>, base.5, Exception::<Impl, OFFSET>, Message::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3TransitionImpl: Sized {
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<Vector3TransitionComponents>;
    fn SetComponents(&self, value: Vector3TransitionComponents) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector3Transition {
    const NAME: &'static str = "Windows.UI.Xaml.IVector3Transition";
}
#[cfg(feature = "implement_exclusive")]
impl IVector3TransitionVtbl {
    pub const fn new<Impl: IVector3TransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVector3TransitionVtbl {
        unsafe extern "system" fn Duration<Impl: IVector3TransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IVector3TransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Components<Impl: IVector3TransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Vector3TransitionComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Impl: IVector3TransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Vector3TransitionComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetComponents(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVector3Transition>, base.5, Duration::<Impl, OFFSET>, SetDuration::<Impl, OFFSET>, Components::<Impl, OFFSET>, SetComponents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3TransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Vector3Transition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector3TransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IVector3TransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVector3TransitionFactoryVtbl {
    pub const fn new<Impl: IVector3TransitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVector3TransitionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IVector3TransitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVector3TransitionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Storyboard(&self) -> ::windows::core::Result<Media::Animation::Storyboard>;
    fn SetStoryboard(&self, value: &::core::option::Option<Media::Animation::Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualState {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualState";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateVtbl {
    pub const fn new<Impl: IVisualStateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateVtbl {
        unsafe extern "system" fn Name<Impl: IVisualStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Storyboard<Impl: IVisualStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Storyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboard<Impl: IVisualStateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Media::Animation::Storyboard as ::windows::core::Abi>::Abi as *const <Media::Animation::Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualState>, base.5, Name::<Impl, OFFSET>, Storyboard::<Impl, OFFSET>, SetStoryboard::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualState2Impl: Sized {
    fn Setters(&self) -> ::windows::core::Result<SetterBaseCollection>;
    fn StateTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StateTriggerBase>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualState2 {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualState2";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualState2Vtbl {
    pub const fn new<Impl: IVisualState2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualState2Vtbl {
        unsafe extern "system" fn Setters<Impl: IVisualState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateTriggers<Impl: IVisualState2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StateTriggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualState2>, base.5, Setters::<Impl, OFFSET>, StateTriggers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateChangedEventArgsImpl: Sized {
    fn OldState(&self) -> ::windows::core::Result<VisualState>;
    fn SetOldState(&self, value: &::core::option::Option<VisualState>) -> ::windows::core::Result<()>;
    fn NewState(&self) -> ::windows::core::Result<VisualState>;
    fn SetNewState(&self, value: &::core::option::Option<VisualState>) -> ::windows::core::Result<()>;
    fn Control(&self) -> ::windows::core::Result<Controls::Control>;
    fn SetControl(&self, value: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateChangedEventArgsVtbl {
    pub const fn new<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateChangedEventArgsVtbl {
        unsafe extern "system" fn OldState<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldState<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOldState(&*(&value as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NewState<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewState<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNewState(&*(&value as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Control<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControl<Impl: IVisualStateChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetControl(&*(&value as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateChangedEventArgs>, base.5, OldState::<Impl, OFFSET>, SetOldState::<Impl, OFFSET>, NewState::<Impl, OFFSET>, SetNewState::<Impl, OFFSET>, Control::<Impl, OFFSET>, SetControl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateGroupImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualTransition>>;
    fn States(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualState>>;
    fn CurrentState(&self) -> ::windows::core::Result<VisualState>;
    fn CurrentStateChanged(&self, handler: &::core::option::Option<VisualStateChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanging(&self, handler: &::core::option::Option<VisualStateChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanging(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateGroup {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateGroup";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateGroupVtbl {
    pub const fn new<Impl: IVisualStateGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateGroupVtbl {
        unsafe extern "system" fn Name<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn States<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).States() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStateChanged<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentStateChanged(&*(&handler as *const <VisualStateChangedEventHandler as ::windows::core::Abi>::Abi as *const <VisualStateChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentStateChanged<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCurrentStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentStateChanging<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentStateChanging(&*(&handler as *const <VisualStateChangedEventHandler as ::windows::core::Abi>::Abi as *const <VisualStateChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentStateChanging<Impl: IVisualStateGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCurrentStateChanging(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateGroup>, base.5, Name::<Impl, OFFSET>, Transitions::<Impl, OFFSET>, States::<Impl, OFFSET>, CurrentState::<Impl, OFFSET>, CurrentStateChanged::<Impl, OFFSET>, RemoveCurrentStateChanged::<Impl, OFFSET>, CurrentStateChanging::<Impl, OFFSET>, RemoveCurrentStateChanging::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManager {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManager";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManagerVtbl {
    pub const fn new<Impl: IVisualStateManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateManagerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateManager>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VisualStateManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManagerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManagerFactoryVtbl {
    pub const fn new<Impl: IVisualStateManagerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateManagerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IVisualStateManagerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateManagerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerOverridesImpl: Sized {
    fn GoToStateCore(&self, control: &::core::option::Option<Controls::Control>, templateroot: &::core::option::Option<FrameworkElement>, statename: &::windows::core::HSTRING, group: &::core::option::Option<VisualStateGroup>, state: &::core::option::Option<VisualState>, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManagerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManagerOverridesVtbl {
    pub const fn new<Impl: IVisualStateManagerOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateManagerOverridesVtbl {
        unsafe extern "system" fn GoToStateCore<Impl: IVisualStateManagerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, control: ::windows::core::RawPtr, templateroot: ::windows::core::RawPtr, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::windows::core::RawPtr, state: ::windows::core::RawPtr, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GoToStateCore(
                &*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType),
                &*(&templateroot as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType),
                &*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&group as *const <VisualStateGroup as ::windows::core::Abi>::Abi as *const <VisualStateGroup as ::windows::core::DefaultType>::DefaultType),
                &*(&state as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                usetransitions,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateManagerOverrides>, base.5, GoToStateCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerProtectedImpl: Sized {
    fn RaiseCurrentStateChanging(&self, stategroup: &::core::option::Option<VisualStateGroup>, oldstate: &::core::option::Option<VisualState>, newstate: &::core::option::Option<VisualState>, control: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
    fn RaiseCurrentStateChanged(&self, stategroup: &::core::option::Option<VisualStateGroup>, oldstate: &::core::option::Option<VisualState>, newstate: &::core::option::Option<VisualState>, control: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManagerProtected {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerProtected";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManagerProtectedVtbl {
    pub const fn new<Impl: IVisualStateManagerProtectedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateManagerProtectedVtbl {
        unsafe extern "system" fn RaiseCurrentStateChanging<Impl: IVisualStateManagerProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stategroup: ::windows::core::RawPtr, oldstate: ::windows::core::RawPtr, newstate: ::windows::core::RawPtr, control: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RaiseCurrentStateChanging(
                    &*(&stategroup as *const <VisualStateGroup as ::windows::core::Abi>::Abi as *const <VisualStateGroup as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&newstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RaiseCurrentStateChanged<Impl: IVisualStateManagerProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stategroup: ::windows::core::RawPtr, oldstate: ::windows::core::RawPtr, newstate: ::windows::core::RawPtr, control: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RaiseCurrentStateChanged(
                    &*(&stategroup as *const <VisualStateGroup as ::windows::core::Abi>::Abi as *const <VisualStateGroup as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&newstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateManagerProtected>, base.5, RaiseCurrentStateChanging::<Impl, OFFSET>, RaiseCurrentStateChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerStaticsImpl: Sized {
    fn GetVisualStateGroups(&self, obj: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualStateGroup>>;
    fn CustomVisualStateManagerProperty(&self) -> ::windows::core::Result<DependencyProperty>;
    fn GetCustomVisualStateManager(&self, obj: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<VisualStateManager>;
    fn SetCustomVisualStateManager(&self, obj: &::core::option::Option<FrameworkElement>, value: &::core::option::Option<VisualStateManager>) -> ::windows::core::Result<()>;
    fn GoToState(&self, control: &::core::option::Option<Controls::Control>, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManagerStaticsVtbl {
    pub const fn new<Impl: IVisualStateManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualStateManagerStaticsVtbl {
        unsafe extern "system" fn GetVisualStateGroups<Impl: IVisualStateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisualStateGroups(&*(&obj as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomVisualStateManagerProperty<Impl: IVisualStateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomVisualStateManagerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomVisualStateManager<Impl: IVisualStateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCustomVisualStateManager(&*(&obj as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVisualStateManager<Impl: IVisualStateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomVisualStateManager(&*(&obj as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <VisualStateManager as ::windows::core::Abi>::Abi as *const <VisualStateManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GoToState<Impl: IVisualStateManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, control: ::windows::core::RawPtr, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GoToState(&*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType), &*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualStateManagerStatics>, base.5, GetVisualStateGroups::<Impl, OFFSET>, CustomVisualStateManagerProperty::<Impl, OFFSET>, GetCustomVisualStateManager::<Impl, OFFSET>, SetCustomVisualStateManager::<Impl, OFFSET>, GoToState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTransitionImpl: Sized {
    fn GeneratedDuration(&self) -> ::windows::core::Result<Duration>;
    fn SetGeneratedDuration(&self, value: &Duration) -> ::windows::core::Result<()>;
    fn GeneratedEasingFunction(&self) -> ::windows::core::Result<Media::Animation::EasingFunctionBase>;
    fn SetGeneratedEasingFunction(&self, value: &::core::option::Option<Media::Animation::EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn To(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Storyboard(&self) -> ::windows::core::Result<Media::Animation::Storyboard>;
    fn SetStoryboard(&self, value: &::core::option::Option<Media::Animation::Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTransition {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualTransition";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTransitionVtbl {
    pub const fn new<Impl: IVisualTransitionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualTransitionVtbl {
        unsafe extern "system" fn GeneratedDuration<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeneratedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedDuration<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGeneratedDuration(&*(&value as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GeneratedEasingFunction<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GeneratedEasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedEasingFunction<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGeneratedEasingFunction(&*(&value as *const <Media::Animation::EasingFunctionBase as ::windows::core::Abi>::Abi as *const <Media::Animation::EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn From<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Storyboard<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Storyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboard<Impl: IVisualTransitionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Media::Animation::Storyboard as ::windows::core::Abi>::Abi as *const <Media::Animation::Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualTransition>, base.5, GeneratedDuration::<Impl, OFFSET>, SetGeneratedDuration::<Impl, OFFSET>, GeneratedEasingFunction::<Impl, OFFSET>, SetGeneratedEasingFunction::<Impl, OFFSET>, To::<Impl, OFFSET>, SetTo::<Impl, OFFSET>, From::<Impl, OFFSET>, SetFrom::<Impl, OFFSET>, Storyboard::<Impl, OFFSET>, SetStoryboard::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTransitionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VisualTransition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualTransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTransitionFactoryVtbl {
    pub const fn new<Impl: IVisualTransitionFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVisualTransitionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IVisualTransitionFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVisualTransitionFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowImpl: Sized {
    fn Bounds(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Visible(&self) -> ::windows::core::Result<bool>;
    fn Content(&self) -> ::windows::core::Result<UIElement>;
    fn SetContent(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn CoreWindow(&self) -> ::windows::core::Result<super::Core::CoreWindow>;
    fn Dispatcher(&self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
    fn Activated(&self, handler: &::core::option::Option<WindowActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<WindowClosedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&self, handler: &::core::option::Option<WindowSizeChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VisibilityChanged(&self, handler: &::core::option::Option<WindowVisibilityChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activate(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindow {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowVtbl {
    pub const fn new<Impl: IWindowImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowVtbl {
        unsafe extern "system" fn Bounds<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContent(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CoreWindow<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CoreWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Activated<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <WindowActivatedEventHandler as ::windows::core::Abi>::Abi as *const <WindowActivatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <WindowClosedEventHandler as ::windows::core::Abi>::Abi as *const <WindowClosedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SizeChanged<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SizeChanged(&*(&handler as *const <WindowSizeChangedEventHandler as ::windows::core::Abi>::Abi as *const <WindowSizeChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSizeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VisibilityChanged<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VisibilityChanged(&*(&handler as *const <WindowVisibilityChangedEventHandler as ::windows::core::Abi>::Abi as *const <WindowVisibilityChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVisibilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Activate<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Activate().into()
        }
        unsafe extern "system" fn Close<Impl: IWindowImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWindow>,
            base.5,
            Bounds::<Impl, OFFSET>,
            Visible::<Impl, OFFSET>,
            Content::<Impl, OFFSET>,
            SetContent::<Impl, OFFSET>,
            CoreWindow::<Impl, OFFSET>,
            Dispatcher::<Impl, OFFSET>,
            Activated::<Impl, OFFSET>,
            RemoveActivated::<Impl, OFFSET>,
            Closed::<Impl, OFFSET>,
            RemoveClosed::<Impl, OFFSET>,
            SizeChanged::<Impl, OFFSET>,
            RemoveSizeChanged::<Impl, OFFSET>,
            VisibilityChanged::<Impl, OFFSET>,
            RemoveVisibilityChanged::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow2Impl: Sized {
    fn SetTitleBar(&self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindow2 {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow2";
}
#[cfg(feature = "implement_exclusive")]
impl IWindow2Vtbl {
    pub const fn new<Impl: IWindow2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindow2Vtbl {
        unsafe extern "system" fn SetTitleBar<Impl: IWindow2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitleBar(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindow2>, base.5, SetTitleBar::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow3Impl: Sized {
    fn Compositor(&self) -> ::windows::core::Result<super::Composition::Compositor>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindow3 {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow3";
}
#[cfg(feature = "implement_exclusive")]
impl IWindow3Vtbl {
    pub const fn new<Impl: IWindow3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindow3Vtbl {
        unsafe extern "system" fn Compositor<Impl: IWindow3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compositor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindow3>, base.5, Compositor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow4Impl: Sized {
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindow4 {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow4";
}
#[cfg(feature = "implement_exclusive")]
impl IWindow4Vtbl {
    pub const fn new<Impl: IWindow4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindow4Vtbl {
        unsafe extern "system" fn UIContext<Impl: IWindow4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindow4>, base.5, UIContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowCreatedEventArgsImpl: Sized {
    fn Window(&self) -> ::windows::core::Result<Window>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IWindowCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowCreatedEventArgsVtbl {
    pub const fn new<Impl: IWindowCreatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowCreatedEventArgsVtbl {
        unsafe extern "system" fn Window<Impl: IWindowCreatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Window() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowCreatedEventArgs>, base.5, Window::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<Window>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IWindowStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowStaticsVtbl {
    pub const fn new<Impl: IWindowStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWindowStaticsVtbl {
        unsafe extern "system" fn Current<Impl: IWindowStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWindowStatics>, base.5, Current::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRootImpl: Sized {
    fn Content(&self) -> ::windows::core::Result<UIElement>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RasterizationScale(&self) -> ::windows::core::Result<f64>;
    fn IsHostVisible(&self) -> ::windows::core::Result<bool>;
    fn UIContext(&self) -> ::windows::core::Result<super::UIContext>;
    fn Changed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRoot {
    const NAME: &'static str = "Windows.UI.Xaml.IXamlRoot";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRootVtbl {
    pub const fn new<Impl: IXamlRootImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlRootVtbl {
        unsafe extern "system" fn Content<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RasterizationScale<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RasterizationScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHostVisible<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsHostVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIContext<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IXamlRootImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlRoot>, base.5, Content::<Impl, OFFSET>, Size::<Impl, OFFSET>, RasterizationScale::<Impl, OFFSET>, IsHostVisible::<Impl, OFFSET>, UIContext::<Impl, OFFSET>, Changed::<Impl, OFFSET>, RemoveChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRootChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRootChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IXamlRootChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRootChangedEventArgsVtbl {
    pub const fn new<Impl: IXamlRootChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlRootChangedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlRootChangedEventArgs>, base.5)
    }
}
