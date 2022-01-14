#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTrigger_Impl: Sized {
    fn MinWindowWidth(&mut self) -> ::windows::core::Result<f64>;
    fn SetMinWindowWidth(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MinWindowHeight(&mut self) -> ::windows::core::Result<f64>;
    fn SetMinWindowHeight(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.IAdaptiveTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveTrigger_Vtbl {
        unsafe extern "system" fn MinWindowWidth<Impl: IAdaptiveTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinWindowWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinWindowWidth<Impl: IAdaptiveTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinWindowWidth(value).into()
        }
        unsafe extern "system" fn MinWindowHeight<Impl: IAdaptiveTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinWindowHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinWindowHeight<Impl: IAdaptiveTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinWindowHeight(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveTrigger, BASE_OFFSET>(),
            MinWindowWidth: MinWindowWidth::<Impl, IMPL_OFFSET>,
            SetMinWindowWidth: SetMinWindowWidth::<Impl, IMPL_OFFSET>,
            MinWindowHeight: MinWindowHeight::<Impl, IMPL_OFFSET>,
            SetMinWindowHeight: SetMinWindowHeight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AdaptiveTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveTriggerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IAdaptiveTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveTriggerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAdaptiveTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveTriggerFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveTriggerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveTriggerStatics_Impl: Sized {
    fn MinWindowWidthProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn MinWindowHeightProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAdaptiveTriggerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IAdaptiveTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAdaptiveTriggerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdaptiveTriggerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdaptiveTriggerStatics_Vtbl {
        unsafe extern "system" fn MinWindowWidthProperty<Impl: IAdaptiveTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinWindowWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinWindowHeightProperty<Impl: IAdaptiveTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinWindowHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAdaptiveTriggerStatics, BASE_OFFSET>(),
            MinWindowWidthProperty: MinWindowWidthProperty::<Impl, IMPL_OFFSET>,
            MinWindowHeightProperty: MinWindowHeightProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdaptiveTriggerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplication_Impl: Sized {
    fn Resources(&mut self) -> ::windows::core::Result<ResourceDictionary>;
    fn SetResources(&mut self, value: &::core::option::Option<ResourceDictionary>) -> ::windows::core::Result<()>;
    fn DebugSettings(&mut self) -> ::windows::core::Result<DebugSettings>;
    fn RequestedTheme(&mut self) -> ::windows::core::Result<ApplicationTheme>;
    fn SetRequestedTheme(&mut self, value: ApplicationTheme) -> ::windows::core::Result<()>;
    fn UnhandledException(&mut self, handler: &::core::option::Option<UnhandledExceptionEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledException(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Suspending(&mut self, handler: &::core::option::Option<SuspendingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSuspending(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Resuming(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveResuming(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Exit(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplication {
    const NAME: &'static str = "Windows.UI.Xaml.IApplication";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplication_Vtbl {
        unsafe extern "system" fn Resources<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResources<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResources(&*(&value as *const <ResourceDictionary as ::windows::core::Abi>::Abi as *const <ResourceDictionary as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DebugSettings<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DebugSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedTheme<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedTheme<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ApplicationTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedTheme(value).into()
        }
        unsafe extern "system" fn UnhandledException<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnhandledException(&*(&handler as *const <UnhandledExceptionEventHandler as ::windows::core::Abi>::Abi as *const <UnhandledExceptionEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnhandledException<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnhandledException(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Suspending<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspending(&*(&handler as *const <SuspendingEventHandler as ::windows::core::Abi>::Abi as *const <SuspendingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSuspending<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSuspending(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Resuming<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resuming(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResuming<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResuming(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Exit<Impl: IApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Exit().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplication, BASE_OFFSET>(),
            Resources: Resources::<Impl, IMPL_OFFSET>,
            SetResources: SetResources::<Impl, IMPL_OFFSET>,
            DebugSettings: DebugSettings::<Impl, IMPL_OFFSET>,
            RequestedTheme: RequestedTheme::<Impl, IMPL_OFFSET>,
            SetRequestedTheme: SetRequestedTheme::<Impl, IMPL_OFFSET>,
            UnhandledException: UnhandledException::<Impl, IMPL_OFFSET>,
            RemoveUnhandledException: RemoveUnhandledException::<Impl, IMPL_OFFSET>,
            Suspending: Suspending::<Impl, IMPL_OFFSET>,
            RemoveSuspending: RemoveSuspending::<Impl, IMPL_OFFSET>,
            Resuming: Resuming::<Impl, IMPL_OFFSET>,
            RemoveResuming: RemoveResuming::<Impl, IMPL_OFFSET>,
            Exit: Exit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IApplication2_Impl: Sized {
    fn FocusVisualKind(&mut self) -> ::windows::core::Result<FocusVisualKind>;
    fn SetFocusVisualKind(&mut self, value: FocusVisualKind) -> ::windows::core::Result<()>;
    fn RequiresPointerMode(&mut self) -> ::windows::core::Result<ApplicationRequiresPointerMode>;
    fn SetRequiresPointerMode(&mut self, value: ApplicationRequiresPointerMode) -> ::windows::core::Result<()>;
    fn LeavingBackground(&mut self, handler: &::core::option::Option<LeavingBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLeavingBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnteredBackground(&mut self, handler: &::core::option::Option<EnteredBackgroundEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnteredBackground(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplication2 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplication2";
}
#[cfg(all(feature = "ApplicationModel", feature = "Foundation", feature = "implement_exclusive"))]
impl IApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplication2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplication2_Vtbl {
        unsafe extern "system" fn FocusVisualKind<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusVisualKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualKind<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FocusVisualKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusVisualKind(value).into()
        }
        unsafe extern "system" fn RequiresPointerMode<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationRequiresPointerMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequiresPointerMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequiresPointerMode<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ApplicationRequiresPointerMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequiresPointerMode(value).into()
        }
        unsafe extern "system" fn LeavingBackground<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LeavingBackground(&*(&handler as *const <LeavingBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <LeavingBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLeavingBackground<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLeavingBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnteredBackground<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnteredBackground(&*(&handler as *const <EnteredBackgroundEventHandler as ::windows::core::Abi>::Abi as *const <EnteredBackgroundEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnteredBackground<Impl: IApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEnteredBackground(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplication2, BASE_OFFSET>(),
            FocusVisualKind: FocusVisualKind::<Impl, IMPL_OFFSET>,
            SetFocusVisualKind: SetFocusVisualKind::<Impl, IMPL_OFFSET>,
            RequiresPointerMode: RequiresPointerMode::<Impl, IMPL_OFFSET>,
            SetRequiresPointerMode: SetRequiresPointerMode::<Impl, IMPL_OFFSET>,
            LeavingBackground: LeavingBackground::<Impl, IMPL_OFFSET>,
            RemoveLeavingBackground: RemoveLeavingBackground::<Impl, IMPL_OFFSET>,
            EnteredBackground: EnteredBackground::<Impl, IMPL_OFFSET>,
            RemoveEnteredBackground: RemoveEnteredBackground::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplication3_Impl: Sized {
    fn HighContrastAdjustment(&mut self) -> ::windows::core::Result<ApplicationHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&mut self, value: ApplicationHighContrastAdjustment) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplication3 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplication3";
}
#[cfg(feature = "implement_exclusive")]
impl IApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplication3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplication3_Vtbl {
        unsafe extern "system" fn HighContrastAdjustment<Impl: IApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ApplicationHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighContrastAdjustment<Impl: IApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ApplicationHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighContrastAdjustment(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplication3, BASE_OFFSET>(),
            HighContrastAdjustment: HighContrastAdjustment::<Impl, IMPL_OFFSET>,
            SetHighContrastAdjustment: SetHighContrastAdjustment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Application>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IApplicationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationInitializationCallbackParams_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IApplicationInitializationCallbackParams {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationInitializationCallbackParams";
}
#[cfg(feature = "implement_exclusive")]
impl IApplicationInitializationCallbackParams_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationInitializationCallbackParams_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationInitializationCallbackParams_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationInitializationCallbackParams, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationInitializationCallbackParams as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IApplicationOverrides_Impl: Sized {
    fn OnActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnLaunched(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::LaunchActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnSearchActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::SearchActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnShareTargetActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileOpenPickerActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnFileSavePickerActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnCachedFileUpdaterActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs>) -> ::windows::core::Result<()>;
    fn OnWindowCreated(&mut self, args: &::core::option::Option<WindowCreatedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl IApplicationOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationOverrides_Vtbl {
        unsafe extern "system" fn OnActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivated(&*(&args as *const <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLaunched<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLaunched(&*(&args as *const <super::super::ApplicationModel::Activation::LaunchActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::LaunchActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFileActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnSearchActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSearchActivated(&*(&args as *const <super::super::ApplicationModel::Activation::SearchActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::SearchActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnShareTargetActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnShareTargetActivated(&*(&args as *const <super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileOpenPickerActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFileOpenPickerActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileSavePickerActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFileSavePickerActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnCachedFileUpdaterActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCachedFileUpdaterActivated(&*(&args as *const <super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnWindowCreated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowCreated(&*(&args as *const <WindowCreatedEventArgs as ::windows::core::Abi>::Abi as *const <WindowCreatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationOverrides, BASE_OFFSET>(),
            OnActivated: OnActivated::<Impl, IMPL_OFFSET>,
            OnLaunched: OnLaunched::<Impl, IMPL_OFFSET>,
            OnFileActivated: OnFileActivated::<Impl, IMPL_OFFSET>,
            OnSearchActivated: OnSearchActivated::<Impl, IMPL_OFFSET>,
            OnShareTargetActivated: OnShareTargetActivated::<Impl, IMPL_OFFSET>,
            OnFileOpenPickerActivated: OnFileOpenPickerActivated::<Impl, IMPL_OFFSET>,
            OnFileSavePickerActivated: OnFileSavePickerActivated::<Impl, IMPL_OFFSET>,
            OnCachedFileUpdaterActivated: OnCachedFileUpdaterActivated::<Impl, IMPL_OFFSET>,
            OnWindowCreated: OnWindowCreated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
pub trait IApplicationOverrides2_Impl: Sized {
    fn OnBackgroundActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides2";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "implement_exclusive"))]
impl IApplicationOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationOverrides2_Vtbl {
        unsafe extern "system" fn OnBackgroundActivated<Impl: IApplicationOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBackgroundActivated(&*(&args as *const <super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationOverrides2, BASE_OFFSET>(),
            OnBackgroundActivated: OnBackgroundActivated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
pub trait IApplicationStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<Application>;
    fn Start(&mut self, callback: &::core::option::Option<ApplicationInitializationCallback>) -> ::windows::core::Result<()>;
    fn LoadComponent(&mut self, component: &::core::option::Option<::windows::core::IInspectable>, resourcelocator: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn LoadComponentWithResourceLocation(&mut self, component: &::core::option::Option<::windows::core::IInspectable>, resourcelocator: &::core::option::Option<super::super::Foundation::Uri>, componentresourcelocation: Controls::Primitives::ComponentResourceLocation) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IApplicationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationStatics";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "implement_exclusive"))]
impl IApplicationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IApplicationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IApplicationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(&*(&callback as *const <ApplicationInitializationCallback as ::windows::core::Abi>::Abi as *const <ApplicationInitializationCallback as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadComponent<Impl: IApplicationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, component: *mut ::core::ffi::c_void, resourcelocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadComponent(&*(&component as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&resourcelocator as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LoadComponentWithResourceLocation<Impl: IApplicationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, component: *mut ::core::ffi::c_void, resourcelocator: ::windows::core::RawPtr, componentresourcelocation: Controls::Primitives::ComponentResourceLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LoadComponentWithResourceLocation(&*(&component as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&resourcelocator as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType), componentresourcelocation).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationStatics, BASE_OFFSET>(),
            Current: Current::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            LoadComponent: LoadComponent::<Impl, IMPL_OFFSET>,
            LoadComponentWithResourceLocation: LoadComponentWithResourceLocation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBindingFailedEventArgs_Impl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBindingFailedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IBindingFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IBindingFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindingFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindingFailedEventArgs_Vtbl {
        unsafe extern "system" fn Message<Impl: IBindingFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBindingFailedEventArgs, BASE_OFFSET>(), Message: Message::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindingFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBringIntoViewOptions_Impl: Sized {
    fn AnimationDesired(&mut self) -> ::windows::core::Result<bool>;
    fn SetAnimationDesired(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TargetRect(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
    fn SetTargetRect(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBringIntoViewOptions {
    const NAME: &'static str = "Windows.UI.Xaml.IBringIntoViewOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBringIntoViewOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBringIntoViewOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBringIntoViewOptions_Vtbl {
        unsafe extern "system" fn AnimationDesired<Impl: IBringIntoViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationDesired<Impl: IBringIntoViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationDesired(value).into()
        }
        unsafe extern "system" fn TargetRect<Impl: IBringIntoViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetRect<Impl: IBringIntoViewOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetRect(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBringIntoViewOptions, BASE_OFFSET>(),
            AnimationDesired: AnimationDesired::<Impl, IMPL_OFFSET>,
            SetAnimationDesired: SetAnimationDesired::<Impl, IMPL_OFFSET>,
            TargetRect: TargetRect::<Impl, IMPL_OFFSET>,
            SetTargetRect: SetTargetRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBringIntoViewOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBringIntoViewOptions2_Impl: Sized {
    fn HorizontalAlignmentRatio(&mut self) -> ::windows::core::Result<f64>;
    fn SetHorizontalAlignmentRatio(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalAlignmentRatio(&mut self) -> ::windows::core::Result<f64>;
    fn SetVerticalAlignmentRatio(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBringIntoViewOptions2 {
    const NAME: &'static str = "Windows.UI.Xaml.IBringIntoViewOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IBringIntoViewOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBringIntoViewOptions2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBringIntoViewOptions2_Vtbl {
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignmentRatio<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignmentRatio<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignmentRatio(value).into()
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IBringIntoViewOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBringIntoViewOptions2, BASE_OFFSET>(),
            HorizontalAlignmentRatio: HorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            SetHorizontalAlignmentRatio: SetHorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            VerticalAlignmentRatio: VerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
            SetVerticalAlignmentRatio: SetVerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBringIntoViewOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBringIntoViewRequestedEventArgs_Impl: Sized {
    fn TargetElement(&mut self) -> ::windows::core::Result<UIElement>;
    fn SetTargetElement(&mut self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn AnimationDesired(&mut self) -> ::windows::core::Result<bool>;
    fn SetAnimationDesired(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TargetRect(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn SetTargetRect(&mut self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn HorizontalAlignmentRatio(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalAlignmentRatio(&mut self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBringIntoViewRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IBringIntoViewRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBringIntoViewRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBringIntoViewRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBringIntoViewRequestedEventArgs_Vtbl {
        unsafe extern "system" fn TargetElement<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetElement<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetElement(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AnimationDesired<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AnimationDesired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAnimationDesired<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAnimationDesired(value).into()
        }
        unsafe extern "system" fn TargetRect<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetRect<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetRect(&*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HorizontalAlignmentRatio<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        unsafe extern "system" fn Handled<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBringIntoViewRequestedEventArgs, BASE_OFFSET>(),
            TargetElement: TargetElement::<Impl, IMPL_OFFSET>,
            SetTargetElement: SetTargetElement::<Impl, IMPL_OFFSET>,
            AnimationDesired: AnimationDesired::<Impl, IMPL_OFFSET>,
            SetAnimationDesired: SetAnimationDesired::<Impl, IMPL_OFFSET>,
            TargetRect: TargetRect::<Impl, IMPL_OFFSET>,
            SetTargetRect: SetTargetRect::<Impl, IMPL_OFFSET>,
            HorizontalAlignmentRatio: HorizontalAlignmentRatio::<Impl, IMPL_OFFSET>,
            VerticalAlignmentRatio: VerticalAlignmentRatio::<Impl, IMPL_OFFSET>,
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBringIntoViewRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IBrushTransition_Impl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IBrushTransition {
    const NAME: &'static str = "Windows.UI.Xaml.IBrushTransition";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IBrushTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrushTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrushTransition_Vtbl {
        unsafe extern "system" fn Duration<Impl: IBrushTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IBrushTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrushTransition, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IBrushTransitionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<BrushTransition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IBrushTransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IBrushTransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IBrushTransitionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBrushTransitionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBrushTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IBrushTransitionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBrushTransitionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBrushTransitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IColorPaletteResources_Impl: Sized {
    fn AltHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMedium(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMedium(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMediumHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMediumHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn AltMediumLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAltMediumLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMedium(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMedium(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMediumHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMediumHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn BaseMediumLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetBaseMediumLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeAltLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeAltLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackMediumLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackMediumLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeBlackMedium(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeBlackMedium(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeDisabledHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeDisabledHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeDisabledLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeDisabledLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeHigh(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeHigh(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeMedium(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeMedium(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeMediumLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeMediumLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeWhite(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeWhite(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ChromeGray(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetChromeGray(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ListLow(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetListLow(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ListMedium(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetListMedium(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn ErrorText(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetErrorText(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
    fn Accent(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::Color>>;
    fn SetAccent(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::Color>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorPaletteResources {
    const NAME: &'static str = "Windows.UI.Xaml.IColorPaletteResources";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IColorPaletteResources_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPaletteResources_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPaletteResources_Vtbl {
        unsafe extern "system" fn AltHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAltHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAltLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAltMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltMediumHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltMediumHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltMediumHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAltMediumHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AltMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AltMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAltMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAltMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseMediumHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseMediumHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMediumHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseMediumHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBaseMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBaseMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeAltLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeAltLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeAltLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeAltLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeBlackHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeBlackHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeBlackLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeBlackLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeBlackMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeBlackMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeBlackMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeBlackMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeBlackMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeBlackMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeDisabledHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeDisabledHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeDisabledHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeDisabledHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeDisabledLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeDisabledLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeDisabledLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeDisabledLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeHigh<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeHigh(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeMediumLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeMediumLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeMediumLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeWhite<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeWhite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeWhite<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeWhite(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ChromeGray<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChromeGray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChromeGray<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChromeGray(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ListLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListLow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListLow<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListLow(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ListMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListMedium() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListMedium<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListMedium(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ErrorText<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetErrorText<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorText(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Accent<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Accent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccent<Impl: IColorPaletteResources_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccent(&*(&value as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::Color> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPaletteResources, BASE_OFFSET>(),
            AltHigh: AltHigh::<Impl, IMPL_OFFSET>,
            SetAltHigh: SetAltHigh::<Impl, IMPL_OFFSET>,
            AltLow: AltLow::<Impl, IMPL_OFFSET>,
            SetAltLow: SetAltLow::<Impl, IMPL_OFFSET>,
            AltMedium: AltMedium::<Impl, IMPL_OFFSET>,
            SetAltMedium: SetAltMedium::<Impl, IMPL_OFFSET>,
            AltMediumHigh: AltMediumHigh::<Impl, IMPL_OFFSET>,
            SetAltMediumHigh: SetAltMediumHigh::<Impl, IMPL_OFFSET>,
            AltMediumLow: AltMediumLow::<Impl, IMPL_OFFSET>,
            SetAltMediumLow: SetAltMediumLow::<Impl, IMPL_OFFSET>,
            BaseHigh: BaseHigh::<Impl, IMPL_OFFSET>,
            SetBaseHigh: SetBaseHigh::<Impl, IMPL_OFFSET>,
            BaseLow: BaseLow::<Impl, IMPL_OFFSET>,
            SetBaseLow: SetBaseLow::<Impl, IMPL_OFFSET>,
            BaseMedium: BaseMedium::<Impl, IMPL_OFFSET>,
            SetBaseMedium: SetBaseMedium::<Impl, IMPL_OFFSET>,
            BaseMediumHigh: BaseMediumHigh::<Impl, IMPL_OFFSET>,
            SetBaseMediumHigh: SetBaseMediumHigh::<Impl, IMPL_OFFSET>,
            BaseMediumLow: BaseMediumLow::<Impl, IMPL_OFFSET>,
            SetBaseMediumLow: SetBaseMediumLow::<Impl, IMPL_OFFSET>,
            ChromeAltLow: ChromeAltLow::<Impl, IMPL_OFFSET>,
            SetChromeAltLow: SetChromeAltLow::<Impl, IMPL_OFFSET>,
            ChromeBlackHigh: ChromeBlackHigh::<Impl, IMPL_OFFSET>,
            SetChromeBlackHigh: SetChromeBlackHigh::<Impl, IMPL_OFFSET>,
            ChromeBlackLow: ChromeBlackLow::<Impl, IMPL_OFFSET>,
            SetChromeBlackLow: SetChromeBlackLow::<Impl, IMPL_OFFSET>,
            ChromeBlackMediumLow: ChromeBlackMediumLow::<Impl, IMPL_OFFSET>,
            SetChromeBlackMediumLow: SetChromeBlackMediumLow::<Impl, IMPL_OFFSET>,
            ChromeBlackMedium: ChromeBlackMedium::<Impl, IMPL_OFFSET>,
            SetChromeBlackMedium: SetChromeBlackMedium::<Impl, IMPL_OFFSET>,
            ChromeDisabledHigh: ChromeDisabledHigh::<Impl, IMPL_OFFSET>,
            SetChromeDisabledHigh: SetChromeDisabledHigh::<Impl, IMPL_OFFSET>,
            ChromeDisabledLow: ChromeDisabledLow::<Impl, IMPL_OFFSET>,
            SetChromeDisabledLow: SetChromeDisabledLow::<Impl, IMPL_OFFSET>,
            ChromeHigh: ChromeHigh::<Impl, IMPL_OFFSET>,
            SetChromeHigh: SetChromeHigh::<Impl, IMPL_OFFSET>,
            ChromeLow: ChromeLow::<Impl, IMPL_OFFSET>,
            SetChromeLow: SetChromeLow::<Impl, IMPL_OFFSET>,
            ChromeMedium: ChromeMedium::<Impl, IMPL_OFFSET>,
            SetChromeMedium: SetChromeMedium::<Impl, IMPL_OFFSET>,
            ChromeMediumLow: ChromeMediumLow::<Impl, IMPL_OFFSET>,
            SetChromeMediumLow: SetChromeMediumLow::<Impl, IMPL_OFFSET>,
            ChromeWhite: ChromeWhite::<Impl, IMPL_OFFSET>,
            SetChromeWhite: SetChromeWhite::<Impl, IMPL_OFFSET>,
            ChromeGray: ChromeGray::<Impl, IMPL_OFFSET>,
            SetChromeGray: SetChromeGray::<Impl, IMPL_OFFSET>,
            ListLow: ListLow::<Impl, IMPL_OFFSET>,
            SetListLow: SetListLow::<Impl, IMPL_OFFSET>,
            ListMedium: ListMedium::<Impl, IMPL_OFFSET>,
            SetListMedium: SetListMedium::<Impl, IMPL_OFFSET>,
            ErrorText: ErrorText::<Impl, IMPL_OFFSET>,
            SetErrorText: SetErrorText::<Impl, IMPL_OFFSET>,
            Accent: Accent::<Impl, IMPL_OFFSET>,
            SetAccent: SetAccent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPaletteResources as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPaletteResourcesFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPaletteResources>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPaletteResourcesFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IColorPaletteResourcesFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPaletteResourcesFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPaletteResourcesFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPaletteResourcesFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorPaletteResourcesFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPaletteResourcesFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPaletteResourcesFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICornerRadiusHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICornerRadiusHelper {
    const NAME: &'static str = "Windows.UI.Xaml.ICornerRadiusHelper";
}
#[cfg(feature = "implement_exclusive")]
impl ICornerRadiusHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICornerRadiusHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICornerRadiusHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICornerRadiusHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICornerRadiusHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICornerRadiusHelperStatics_Impl: Sized {
    fn FromRadii(&mut self, topleft: f64, topright: f64, bottomright: f64, bottomleft: f64) -> ::windows::core::Result<CornerRadius>;
    fn FromUniformRadius(&mut self, uniformradius: f64) -> ::windows::core::Result<CornerRadius>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICornerRadiusHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.ICornerRadiusHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICornerRadiusHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICornerRadiusHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICornerRadiusHelperStatics_Vtbl {
        unsafe extern "system" fn FromRadii<Impl: ICornerRadiusHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, topleft: f64, topright: f64, bottomright: f64, bottomleft: f64, result__: *mut CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromRadii(topleft, topright, bottomright, bottomleft) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromUniformRadius<Impl: ICornerRadiusHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniformradius: f64, result__: *mut CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromUniformRadius(uniformradius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICornerRadiusHelperStatics, BASE_OFFSET>(),
            FromRadii: FromRadii::<Impl, IMPL_OFFSET>,
            FromUniformRadius: FromUniformRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICornerRadiusHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataContextChangedEventArgs_Impl: Sized {
    fn NewValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataContextChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDataContextChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDataContextChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataContextChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataContextChangedEventArgs_Vtbl {
        unsafe extern "system" fn NewValue<Impl: IDataContextChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IDataContextChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDataContextChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataContextChangedEventArgs, BASE_OFFSET>(),
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataContextChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplate_Impl: Sized {
    fn LoadContent(&mut self) -> ::windows::core::Result<DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplate";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplate_Vtbl {
        unsafe extern "system" fn LoadContent<Impl: IDataTemplate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplate, BASE_OFFSET>(), LoadContent: LoadContent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls")]
pub trait IDataTemplateExtension_Impl: Sized {
    fn ResetTemplate(&mut self) -> ::windows::core::Result<()>;
    fn ProcessBinding(&mut self, phase: u32) -> ::windows::core::Result<bool>;
    fn ProcessBindings(&mut self, arg: &::core::option::Option<Controls::ContainerContentChangingEventArgs>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "UI_Xaml_Controls")]
impl ::windows::core::RuntimeName for IDataTemplateExtension {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateExtension";
}
#[cfg(feature = "UI_Xaml_Controls")]
impl IDataTemplateExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateExtension_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateExtension_Vtbl {
        unsafe extern "system" fn ResetTemplate<Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetTemplate().into()
        }
        unsafe extern "system" fn ProcessBinding<Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phase: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessBinding(phase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessBindings<Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arg: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessBindings(&*(&arg as *const <Controls::ContainerContentChangingEventArgs as ::windows::core::Abi>::Abi as *const <Controls::ContainerContentChangingEventArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateExtension, BASE_OFFSET>(),
            ResetTemplate: ResetTemplate::<Impl, IMPL_OFFSET>,
            ProcessBinding: ProcessBinding::<Impl, IMPL_OFFSET>,
            ProcessBindings: ProcessBindings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDataTemplateFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateKey_Impl: Sized {
    fn DataType(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDataType(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateKey {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateKey";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateKey_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateKey_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateKey_Vtbl {
        unsafe extern "system" fn DataType<Impl: IDataTemplateKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataType<Impl: IDataTemplateKey_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataType(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateKey, BASE_OFFSET>(),
            DataType: DataType::<Impl, IMPL_OFFSET>,
            SetDataType: SetDataType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateKey as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateKeyFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateKey>;
    fn CreateInstanceWithType(&mut self, datatype: &::core::option::Option<::windows::core::IInspectable>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DataTemplateKey>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateKeyFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateKeyFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateKeyFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateKeyFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateKeyFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDataTemplateKeyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithType<Impl: IDataTemplateKeyFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithType(&*(&datatype as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateKeyFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithType: CreateInstanceWithType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateKeyFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTemplateStatics2_Impl: Sized {
    fn ExtensionInstanceProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn GetExtensionInstance(&mut self, element: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<IDataTemplateExtension>;
    fn SetExtensionInstance(&mut self, element: &::core::option::Option<FrameworkElement>, value: &::core::option::Option<IDataTemplateExtension>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDataTemplateStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IDataTemplateStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateStatics2_Vtbl {
        unsafe extern "system" fn ExtensionInstanceProperty<Impl: IDataTemplateStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtensionInstanceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtensionInstance<Impl: IDataTemplateStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtensionInstance(&*(&element as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtensionInstance<Impl: IDataTemplateStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtensionInstance(&*(&element as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IDataTemplateExtension as ::windows::core::Abi>::Abi as *const <IDataTemplateExtension as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateStatics2, BASE_OFFSET>(),
            ExtensionInstanceProperty: ExtensionInstanceProperty::<Impl, IMPL_OFFSET>,
            GetExtensionInstance: GetExtensionInstance::<Impl, IMPL_OFFSET>,
            SetExtensionInstance: SetExtensionInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDebugSettings_Impl: Sized {
    fn EnableFrameRateCounter(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableFrameRateCounter(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsBindingTracingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsBindingTracingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsOverdrawHeatMapEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOverdrawHeatMapEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn BindingFailed(&mut self, handler: &::core::option::Option<BindingFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBindingFailed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDebugSettings {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDebugSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDebugSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDebugSettings_Vtbl {
        unsafe extern "system" fn EnableFrameRateCounter<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableFrameRateCounter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFrameRateCounter<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableFrameRateCounter(value).into()
        }
        unsafe extern "system" fn IsBindingTracingEnabled<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBindingTracingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsBindingTracingEnabled<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsBindingTracingEnabled(value).into()
        }
        unsafe extern "system" fn IsOverdrawHeatMapEnabled<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOverdrawHeatMapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOverdrawHeatMapEnabled<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOverdrawHeatMapEnabled(value).into()
        }
        unsafe extern "system" fn BindingFailed<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindingFailed(&*(&handler as *const <BindingFailedEventHandler as ::windows::core::Abi>::Abi as *const <BindingFailedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBindingFailed<Impl: IDebugSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBindingFailed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDebugSettings, BASE_OFFSET>(),
            EnableFrameRateCounter: EnableFrameRateCounter::<Impl, IMPL_OFFSET>,
            SetEnableFrameRateCounter: SetEnableFrameRateCounter::<Impl, IMPL_OFFSET>,
            IsBindingTracingEnabled: IsBindingTracingEnabled::<Impl, IMPL_OFFSET>,
            SetIsBindingTracingEnabled: SetIsBindingTracingEnabled::<Impl, IMPL_OFFSET>,
            IsOverdrawHeatMapEnabled: IsOverdrawHeatMapEnabled::<Impl, IMPL_OFFSET>,
            SetIsOverdrawHeatMapEnabled: SetIsOverdrawHeatMapEnabled::<Impl, IMPL_OFFSET>,
            BindingFailed: BindingFailed::<Impl, IMPL_OFFSET>,
            RemoveBindingFailed: RemoveBindingFailed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings2_Impl: Sized {
    fn EnableRedrawRegions(&mut self) -> ::windows::core::Result<bool>;
    fn SetEnableRedrawRegions(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDebugSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDebugSettings2_Vtbl {
        unsafe extern "system" fn EnableRedrawRegions<Impl: IDebugSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableRedrawRegions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableRedrawRegions<Impl: IDebugSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableRedrawRegions(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDebugSettings2, BASE_OFFSET>(),
            EnableRedrawRegions: EnableRedrawRegions::<Impl, IMPL_OFFSET>,
            SetEnableRedrawRegions: SetEnableRedrawRegions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugSettings2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings3_Impl: Sized {
    fn IsTextPerformanceVisualizationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsTextPerformanceVisualizationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings3 {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDebugSettings3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDebugSettings3_Vtbl {
        unsafe extern "system" fn IsTextPerformanceVisualizationEnabled<Impl: IDebugSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTextPerformanceVisualizationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextPerformanceVisualizationEnabled<Impl: IDebugSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTextPerformanceVisualizationEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDebugSettings3, BASE_OFFSET>(),
            IsTextPerformanceVisualizationEnabled: IsTextPerformanceVisualizationEnabled::<Impl, IMPL_OFFSET>,
            SetIsTextPerformanceVisualizationEnabled: SetIsTextPerformanceVisualizationEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugSettings3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDebugSettings4_Impl: Sized {
    fn FailFastOnErrors(&mut self) -> ::windows::core::Result<bool>;
    fn SetFailFastOnErrors(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDebugSettings4 {
    const NAME: &'static str = "Windows.UI.Xaml.IDebugSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl IDebugSettings4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDebugSettings4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDebugSettings4_Vtbl {
        unsafe extern "system" fn FailFastOnErrors<Impl: IDebugSettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FailFastOnErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFailFastOnErrors<Impl: IDebugSettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFailFastOnErrors(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDebugSettings4, BASE_OFFSET>(),
            FailFastOnErrors: FailFastOnErrors::<Impl, IMPL_OFFSET>,
            SetFailFastOnErrors: SetFailFastOnErrors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDebugSettings4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IDependencyObject_Impl: Sized {
    fn GetValue(&mut self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&mut self, dp: &::core::option::Option<DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ClearValue(&mut self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<()>;
    fn ReadLocalValue(&mut self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnimationBaseValue(&mut self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Dispatcher(&mut self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDependencyObject {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObject";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl IDependencyObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyObject_Vtbl {
        unsafe extern "system" fn GetValue<Impl: IDependencyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IDependencyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ClearValue<Impl: IDependencyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReadLocalValue<Impl: IDependencyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadLocalValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnimationBaseValue<Impl: IDependencyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnimationBaseValue(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: IDependencyObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyObject, BASE_OFFSET>(),
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            ClearValue: ClearValue::<Impl, IMPL_OFFSET>,
            ReadLocalValue: ReadLocalValue::<Impl, IMPL_OFFSET>,
            GetAnimationBaseValue: GetAnimationBaseValue::<Impl, IMPL_OFFSET>,
            Dispatcher: Dispatcher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObject2_Impl: Sized {
    fn RegisterPropertyChangedCallback(&mut self, dp: &::core::option::Option<DependencyProperty>, callback: &::core::option::Option<DependencyPropertyChangedCallback>) -> ::windows::core::Result<i64>;
    fn UnregisterPropertyChangedCallback(&mut self, dp: &::core::option::Option<DependencyProperty>, token: i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyObject2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObject2";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyObject2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyObject2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyObject2_Vtbl {
        unsafe extern "system" fn RegisterPropertyChangedCallback<Impl: IDependencyObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, callback: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPropertyChangedCallback(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&callback as *const <DependencyPropertyChangedCallback as ::windows::core::Abi>::Abi as *const <DependencyPropertyChangedCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterPropertyChangedCallback<Impl: IDependencyObject2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, token: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterPropertyChangedCallback(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), token).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyObject2, BASE_OFFSET>(),
            RegisterPropertyChangedCallback: RegisterPropertyChangedCallback::<Impl, IMPL_OFFSET>,
            UnregisterPropertyChangedCallback: UnregisterPropertyChangedCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyObject2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDependencyObjectCollectionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DependencyObjectCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDependencyObjectCollectionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObjectCollectionFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDependencyObjectCollectionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyObjectCollectionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyObjectCollectionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDependencyObjectCollectionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyObjectCollectionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyObjectCollectionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyObjectFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyObjectFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyObjectFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyObjectFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyObjectFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyObjectFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDependencyObjectFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyObjectFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyObjectFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait IDependencyProperty_Impl: Sized {
    fn GetMetadata(&mut self, fortype: &Interop::TypeName) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDependencyProperty {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyProperty";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl IDependencyProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyProperty_Vtbl {
        unsafe extern "system" fn GetMetadata<Impl: IDependencyProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fortype: ::core::mem::ManuallyDrop<Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata(&*(&fortype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyProperty, BASE_OFFSET>(), GetMetadata: GetMetadata::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDependencyPropertyChangedEventArgs_Impl: Sized {
    fn Property(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn OldValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn NewValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDependencyPropertyChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyPropertyChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDependencyPropertyChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyPropertyChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyPropertyChangedEventArgs_Vtbl {
        unsafe extern "system" fn Property<Impl: IDependencyPropertyChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldValue<Impl: IDependencyPropertyChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewValue<Impl: IDependencyPropertyChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyPropertyChangedEventArgs, BASE_OFFSET>(),
            Property: Property::<Impl, IMPL_OFFSET>,
            OldValue: OldValue::<Impl, IMPL_OFFSET>,
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyPropertyChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait IDependencyPropertyStatics_Impl: Sized {
    fn UnsetValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Register(&mut self, name: &::windows::core::HSTRING, propertytype: &Interop::TypeName, ownertype: &Interop::TypeName, typemetadata: &::core::option::Option<PropertyMetadata>) -> ::windows::core::Result<DependencyProperty>;
    fn RegisterAttached(&mut self, name: &::windows::core::HSTRING, propertytype: &Interop::TypeName, ownertype: &Interop::TypeName, defaultmetadata: &::core::option::Option<PropertyMetadata>) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDependencyPropertyStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IDependencyPropertyStatics";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl IDependencyPropertyStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDependencyPropertyStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDependencyPropertyStatics_Vtbl {
        unsafe extern "system" fn UnsetValue<Impl: IDependencyPropertyStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnsetValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Register<Impl: IDependencyPropertyStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<Interop::TypeName>, ownertype: ::core::mem::ManuallyDrop<Interop::TypeName>, typemetadata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn RegisterAttached<Impl: IDependencyPropertyStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertytype: ::core::mem::ManuallyDrop<Interop::TypeName>, ownertype: ::core::mem::ManuallyDrop<Interop::TypeName>, defaultmetadata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDependencyPropertyStatics, BASE_OFFSET>(),
            UnsetValue: UnsetValue::<Impl, IMPL_OFFSET>,
            Register: Register::<Impl, IMPL_OFFSET>,
            RegisterAttached: RegisterAttached::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDependencyPropertyStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDispatcherTimer_Impl: Sized {
    fn Interval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn Tick(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTick(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDispatcherTimer {
    const NAME: &'static str = "Windows.UI.Xaml.IDispatcherTimer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDispatcherTimer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherTimer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherTimer_Vtbl {
        unsafe extern "system" fn Interval<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tick<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tick(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTick<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTick(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IDispatcherTimer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherTimer, BASE_OFFSET>(),
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            Tick: Tick::<Impl, IMPL_OFFSET>,
            RemoveTick: RemoveTick::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherTimer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDispatcherTimerFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DispatcherTimer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDispatcherTimerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IDispatcherTimerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDispatcherTimerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatcherTimerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatcherTimerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IDispatcherTimerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDispatcherTimerFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatcherTimerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDragEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn SetData(&mut self, value: &::core::option::Option<super::super::ApplicationModel::DataTransfer::DataPackage>) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self, relativeto: &::core::option::Option<UIElement>) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDragEventArgs";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl IDragEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IDragEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDragEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Data<Impl: IDragEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IDragEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::DataTransfer::DataPackage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IDragEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "ApplicationModel_DataTransfer_DragDrop", feature = "implement_exclusive"))]
pub trait IDragEventArgs2_Impl: Sized {
    fn DataView(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageView>;
    fn DragUIOverride(&mut self) -> ::windows::core::Result<DragUIOverride>;
    fn Modifiers(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers>;
    fn AcceptedOperation(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn SetAcceptedOperation(&mut self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<DragOperationDeferral>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "ApplicationModel_DataTransfer_DragDrop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDragEventArgs2";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "ApplicationModel_DataTransfer_DragDrop", feature = "implement_exclusive"))]
impl IDragEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragEventArgs2_Vtbl {
        unsafe extern "system" fn DataView<Impl: IDragEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragUIOverride<Impl: IDragEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragUIOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modifiers<Impl: IDragEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DragDrop::DragDropModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcceptedOperation<Impl: IDragEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcceptedOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceptedOperation<Impl: IDragEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceptedOperation(value).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IDragEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragEventArgs2, BASE_OFFSET>(),
            DataView: DataView::<Impl, IMPL_OFFSET>,
            DragUIOverride: DragUIOverride::<Impl, IMPL_OFFSET>,
            Modifiers: Modifiers::<Impl, IMPL_OFFSET>,
            AcceptedOperation: AcceptedOperation::<Impl, IMPL_OFFSET>,
            SetAcceptedOperation: SetAcceptedOperation::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
pub trait IDragEventArgs3_Impl: Sized {
    fn AllowedOperations(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.IDragEventArgs3";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
impl IDragEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragEventArgs3_Vtbl {
        unsafe extern "system" fn AllowedOperations<Impl: IDragEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragEventArgs3, BASE_OFFSET>(),
            AllowedOperations: AllowedOperations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragOperationDeferral_Impl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragOperationDeferral {
    const NAME: &'static str = "Windows.UI.Xaml.IDragOperationDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDragOperationDeferral_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragOperationDeferral_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragOperationDeferral_Vtbl {
        unsafe extern "system" fn Complete<Impl: IDragOperationDeferral_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDragOperationDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragOperationDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDragStartingEventArgs_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackage>;
    fn DragUI(&mut self) -> ::windows::core::Result<DragUI>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<DragOperationDeferral>;
    fn GetPosition(&mut self, relativeto: &::core::option::Option<UIElement>) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragStartingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDragStartingEventArgs";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "implement_exclusive"))]
impl IDragStartingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragStartingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragStartingEventArgs_Vtbl {
        unsafe extern "system" fn Cancel<Impl: IDragStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: IDragStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        unsafe extern "system" fn Data<Impl: IDragStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragUI<Impl: IDragStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IDragStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPosition<Impl: IDragStartingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragStartingEventArgs, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            DragUI: DragUI::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragStartingEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
pub trait IDragStartingEventArgs2_Impl: Sized {
    fn AllowedOperations(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
    fn SetAllowedOperations(&mut self, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragStartingEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.IDragStartingEventArgs2";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
impl IDragStartingEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragStartingEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragStartingEventArgs2_Vtbl {
        unsafe extern "system" fn AllowedOperations<Impl: IDragStartingEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowedOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowedOperations<Impl: IDragStartingEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedOperations(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragStartingEventArgs2, BASE_OFFSET>(),
            AllowedOperations: AllowedOperations::<Impl, IMPL_OFFSET>,
            SetAllowedOperations: SetAllowedOperations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragStartingEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "UI_Xaml_Media_Imaging", feature = "implement_exclusive"))]
pub trait IDragUI_Impl: Sized {
    fn SetContentFromBitmapImage(&mut self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImageWithAnchorPoint(&mut self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmap(&mut self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&mut self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromDataPackage(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "UI_Xaml_Media_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragUI {
    const NAME: &'static str = "Windows.UI.Xaml.IDragUI";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "UI_Xaml_Media_Imaging", feature = "implement_exclusive"))]
impl IDragUI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragUI_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragUI_Vtbl {
        unsafe extern "system" fn SetContentFromBitmapImage<Impl: IDragUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImage(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromBitmapImageWithAnchorPoint<Impl: IDragUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImageWithAnchorPoint(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmap<Impl: IDragUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmap(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmapWithAnchorPoint<Impl: IDragUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmapWithAnchorPoint(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromDataPackage<Impl: IDragUI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromDataPackage().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragUI, BASE_OFFSET>(),
            SetContentFromBitmapImage: SetContentFromBitmapImage::<Impl, IMPL_OFFSET>,
            SetContentFromBitmapImageWithAnchorPoint: SetContentFromBitmapImageWithAnchorPoint::<Impl, IMPL_OFFSET>,
            SetContentFromSoftwareBitmap: SetContentFromSoftwareBitmap::<Impl, IMPL_OFFSET>,
            SetContentFromSoftwareBitmapWithAnchorPoint: SetContentFromSoftwareBitmapWithAnchorPoint::<Impl, IMPL_OFFSET>,
            SetContentFromDataPackage: SetContentFromDataPackage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragUI as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "UI_Xaml_Media_Imaging", feature = "implement_exclusive"))]
pub trait IDragUIOverride_Impl: Sized {
    fn Caption(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCaption(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsContentVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsContentVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsCaptionVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsCaptionVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsGlyphVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsGlyphVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImage(&mut self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>) -> ::windows::core::Result<()>;
    fn SetContentFromBitmapImageWithAnchorPoint(&mut self, bitmapimage: &::core::option::Option<Media::Imaging::BitmapImage>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmap(&mut self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>) -> ::windows::core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&mut self, softwarebitmap: &::core::option::Option<super::super::Graphics::Imaging::SoftwareBitmap>, anchorpoint: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "UI_Xaml_Media_Imaging", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDragUIOverride {
    const NAME: &'static str = "Windows.UI.Xaml.IDragUIOverride";
}
#[cfg(all(feature = "Foundation", feature = "Graphics_Imaging", feature = "UI_Xaml_Media_Imaging", feature = "implement_exclusive"))]
impl IDragUIOverride_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragUIOverride_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragUIOverride_Vtbl {
        unsafe extern "system" fn Caption<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Caption() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCaption<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCaption(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsContentVisible<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsContentVisible<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsContentVisible(value).into()
        }
        unsafe extern "system" fn IsCaptionVisible<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCaptionVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsCaptionVisible<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsCaptionVisible(value).into()
        }
        unsafe extern "system" fn IsGlyphVisible<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGlyphVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsGlyphVisible<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsGlyphVisible(value).into()
        }
        unsafe extern "system" fn Clear<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetContentFromBitmapImage<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImage(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromBitmapImageWithAnchorPoint<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bitmapimage: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromBitmapImageWithAnchorPoint(&*(&bitmapimage as *const <Media::Imaging::BitmapImage as ::windows::core::Abi>::Abi as *const <Media::Imaging::BitmapImage as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmap<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmap(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmapWithAnchorPoint<Impl: IDragUIOverride_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, softwarebitmap: ::windows::core::RawPtr, anchorpoint: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentFromSoftwareBitmapWithAnchorPoint(&*(&softwarebitmap as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::Abi>::Abi as *const <super::super::Graphics::Imaging::SoftwareBitmap as ::windows::core::DefaultType>::DefaultType), &*(&anchorpoint as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragUIOverride, BASE_OFFSET>(),
            Caption: Caption::<Impl, IMPL_OFFSET>,
            SetCaption: SetCaption::<Impl, IMPL_OFFSET>,
            IsContentVisible: IsContentVisible::<Impl, IMPL_OFFSET>,
            SetIsContentVisible: SetIsContentVisible::<Impl, IMPL_OFFSET>,
            IsCaptionVisible: IsCaptionVisible::<Impl, IMPL_OFFSET>,
            SetIsCaptionVisible: SetIsCaptionVisible::<Impl, IMPL_OFFSET>,
            IsGlyphVisible: IsGlyphVisible::<Impl, IMPL_OFFSET>,
            SetIsGlyphVisible: SetIsGlyphVisible::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            SetContentFromBitmapImage: SetContentFromBitmapImage::<Impl, IMPL_OFFSET>,
            SetContentFromBitmapImageWithAnchorPoint: SetContentFromBitmapImageWithAnchorPoint::<Impl, IMPL_OFFSET>,
            SetContentFromSoftwareBitmap: SetContentFromSoftwareBitmap::<Impl, IMPL_OFFSET>,
            SetContentFromSoftwareBitmapWithAnchorPoint: SetContentFromSoftwareBitmapWithAnchorPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragUIOverride as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
pub trait IDropCompletedEventArgs_Impl: Sized {
    fn DropResult(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackageOperation>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDropCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IDropCompletedEventArgs";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "implement_exclusive"))]
impl IDropCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDropCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDropCompletedEventArgs_Vtbl {
        unsafe extern "system" fn DropResult<Impl: IDropCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ApplicationModel::DataTransfer::DataPackageOperation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropResult() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDropCompletedEventArgs, BASE_OFFSET>(), DropResult: DropResult::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDropCompletedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDurationHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDurationHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IDurationHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IDurationHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDurationHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDurationHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDurationHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDurationHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDurationHelperStatics_Impl: Sized {
    fn Automatic(&mut self) -> ::windows::core::Result<Duration>;
    fn Forever(&mut self) -> ::windows::core::Result<Duration>;
    fn Compare(&mut self, duration1: &Duration, duration2: &Duration) -> ::windows::core::Result<i32>;
    fn FromTimeSpan(&mut self, timespan: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<Duration>;
    fn GetHasTimeSpan(&mut self, target: &Duration) -> ::windows::core::Result<bool>;
    fn Add(&mut self, target: &Duration, duration: &Duration) -> ::windows::core::Result<Duration>;
    fn Equals(&mut self, target: &Duration, value: &Duration) -> ::windows::core::Result<bool>;
    fn Subtract(&mut self, target: &Duration, duration: &Duration) -> ::windows::core::Result<Duration>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDurationHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IDurationHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDurationHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDurationHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDurationHelperStatics_Vtbl {
        unsafe extern "system" fn Automatic<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Automatic() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Forever<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Forever() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compare<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration1: Duration, duration2: Duration, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compare(&*(&duration1 as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&duration2 as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromTimeSpan<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timespan: super::super::Foundation::TimeSpan, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromTimeSpan(&*(&timespan as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHasTimeSpan<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Duration, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHasTimeSpan(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Duration, duration: Duration, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Duration, value: Duration, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subtract<Impl: IDurationHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: Duration, duration: Duration, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subtract(&*(&target as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType), &*(&duration as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDurationHelperStatics, BASE_OFFSET>(),
            Automatic: Automatic::<Impl, IMPL_OFFSET>,
            Forever: Forever::<Impl, IMPL_OFFSET>,
            Compare: Compare::<Impl, IMPL_OFFSET>,
            FromTimeSpan: FromTimeSpan::<Impl, IMPL_OFFSET>,
            GetHasTimeSpan: GetHasTimeSpan::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
            Subtract: Subtract::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDurationHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IEffectiveViewportChangedEventArgs_Impl: Sized {
    fn EffectiveViewport(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn MaxViewport(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn BringIntoViewDistanceX(&mut self) -> ::windows::core::Result<f64>;
    fn BringIntoViewDistanceY(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEffectiveViewportChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IEffectiveViewportChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IEffectiveViewportChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEffectiveViewportChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEffectiveViewportChangedEventArgs_Vtbl {
        unsafe extern "system" fn EffectiveViewport<Impl: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxViewport<Impl: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxViewport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringIntoViewDistanceX<Impl: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BringIntoViewDistanceX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringIntoViewDistanceY<Impl: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BringIntoViewDistanceY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEffectiveViewportChangedEventArgs, BASE_OFFSET>(),
            EffectiveViewport: EffectiveViewport::<Impl, IMPL_OFFSET>,
            MaxViewport: MaxViewport::<Impl, IMPL_OFFSET>,
            BringIntoViewDistanceX: BringIntoViewDistanceX::<Impl, IMPL_OFFSET>,
            BringIntoViewDistanceY: BringIntoViewDistanceY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEffectiveViewportChangedEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IElementFactory_Impl: Sized {
    fn GetElement(&mut self, args: &::core::option::Option<ElementFactoryGetArgs>) -> ::windows::core::Result<UIElement>;
    fn RecycleElement(&mut self, args: &::core::option::Option<ElementFactoryRecycleArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactory";
}
impl IElementFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementFactory_Vtbl {
        unsafe extern "system" fn GetElement<Impl: IElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(&*(&args as *const <ElementFactoryGetArgs as ::windows::core::Abi>::Abi as *const <ElementFactoryGetArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleElement<Impl: IElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecycleElement(&*(&args as *const <ElementFactoryRecycleArgs as ::windows::core::Abi>::Abi as *const <ElementFactoryRecycleArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactory, BASE_OFFSET>(),
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
            RecycleElement: RecycleElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryGetArgs_Impl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetData(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<UIElement>;
    fn SetParent(&mut self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryGetArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryGetArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryGetArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactoryGetArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementFactoryGetArgs_Vtbl {
        unsafe extern "system" fn Data<Impl: IElementFactoryGetArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IElementFactoryGetArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IElementFactoryGetArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParent<Impl: IElementFactoryGetArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactoryGetArgs, BASE_OFFSET>(),
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            SetParent: SetParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactoryGetArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryGetArgsFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ElementFactoryGetArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryGetArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryGetArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryGetArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactoryGetArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementFactoryGetArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IElementFactoryGetArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactoryGetArgsFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactoryGetArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryRecycleArgs_Impl: Sized {
    fn Element(&mut self) -> ::windows::core::Result<UIElement>;
    fn SetElement(&mut self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<UIElement>;
    fn SetParent(&mut self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryRecycleArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryRecycleArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryRecycleArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactoryRecycleArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementFactoryRecycleArgs_Vtbl {
        unsafe extern "system" fn Element<Impl: IElementFactoryRecycleArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Element() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElement<Impl: IElementFactoryRecycleArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElement(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IElementFactoryRecycleArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParent<Impl: IElementFactoryRecycleArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactoryRecycleArgs, BASE_OFFSET>(),
            Element: Element::<Impl, IMPL_OFFSET>,
            SetElement: SetElement::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            SetParent: SetParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactoryRecycleArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementFactoryRecycleArgsFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ElementFactoryRecycleArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementFactoryRecycleArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactoryRecycleArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IElementFactoryRecycleArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactoryRecycleArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementFactoryRecycleArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IElementFactoryRecycleArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactoryRecycleArgsFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactoryRecycleArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayer_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementSoundPlayer {
    const NAME: &'static str = "Windows.UI.Xaml.IElementSoundPlayer";
}
#[cfg(feature = "implement_exclusive")]
impl IElementSoundPlayer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementSoundPlayer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementSoundPlayer_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IElementSoundPlayer, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementSoundPlayer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerStatics_Impl: Sized {
    fn Volume(&mut self) -> ::windows::core::Result<f64>;
    fn SetVolume(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn State(&mut self) -> ::windows::core::Result<ElementSoundPlayerState>;
    fn SetState(&mut self, value: ElementSoundPlayerState) -> ::windows::core::Result<()>;
    fn Play(&mut self, sound: ElementSoundKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementSoundPlayerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IElementSoundPlayerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IElementSoundPlayerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementSoundPlayerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementSoundPlayerStatics_Vtbl {
        unsafe extern "system" fn Volume<Impl: IElementSoundPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Volume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IElementSoundPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(value).into()
        }
        unsafe extern "system" fn State<Impl: IElementSoundPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ElementSoundPlayerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).State() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IElementSoundPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ElementSoundPlayerState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(value).into()
        }
        unsafe extern "system" fn Play<Impl: IElementSoundPlayerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sound: ElementSoundKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play(sound).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementSoundPlayerStatics, BASE_OFFSET>(),
            Volume: Volume::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            State: State::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementSoundPlayerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IElementSoundPlayerStatics2_Impl: Sized {
    fn SpatialAudioMode(&mut self) -> ::windows::core::Result<ElementSpatialAudioMode>;
    fn SetSpatialAudioMode(&mut self, value: ElementSpatialAudioMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IElementSoundPlayerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IElementSoundPlayerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IElementSoundPlayerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementSoundPlayerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementSoundPlayerStatics2_Vtbl {
        unsafe extern "system" fn SpatialAudioMode<Impl: IElementSoundPlayerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ElementSpatialAudioMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SpatialAudioMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpatialAudioMode<Impl: IElementSoundPlayerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ElementSpatialAudioMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpatialAudioMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementSoundPlayerStatics2, BASE_OFFSET>(),
            SpatialAudioMode: SpatialAudioMode::<Impl, IMPL_OFFSET>,
            SetSpatialAudioMode: SetSpatialAudioMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementSoundPlayerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IEventTrigger_Impl: Sized {
    fn RoutedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn SetRoutedEvent(&mut self, value: &::core::option::Option<RoutedEvent>) -> ::windows::core::Result<()>;
    fn Actions(&mut self) -> ::windows::core::Result<TriggerActionCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IEventTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.IEventTrigger";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IEventTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventTrigger_Vtbl {
        unsafe extern "system" fn RoutedEvent<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoutedEvent<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoutedEvent(&*(&value as *const <RoutedEvent as ::windows::core::Abi>::Abi as *const <RoutedEvent as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Actions<Impl: IEventTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IEventTrigger, BASE_OFFSET>(),
            RoutedEvent: RoutedEvent::<Impl, IMPL_OFFSET>,
            SetRoutedEvent: SetRoutedEvent::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExceptionRoutedEventArgs_Impl: Sized {
    fn ErrorMessage(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExceptionRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IExceptionRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExceptionRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExceptionRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExceptionRoutedEventArgs_Vtbl {
        unsafe extern "system" fn ErrorMessage<Impl: IExceptionRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExceptionRoutedEventArgs, BASE_OFFSET>(),
            ErrorMessage: ErrorMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExceptionRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExceptionRoutedEventArgsFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExceptionRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IExceptionRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IExceptionRoutedEventArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExceptionRoutedEventArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExceptionRoutedEventArgsFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IExceptionRoutedEventArgsFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExceptionRoutedEventArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Data", feature = "implement_exclusive"))]
pub trait IFrameworkElement_Impl: Sized {
    fn Triggers(&mut self) -> ::windows::core::Result<TriggerCollection>;
    fn Resources(&mut self) -> ::windows::core::Result<ResourceDictionary>;
    fn SetResources(&mut self, value: &::core::option::Option<ResourceDictionary>) -> ::windows::core::Result<()>;
    fn Tag(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Language(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ActualWidth(&mut self) -> ::windows::core::Result<f64>;
    fn ActualHeight(&mut self) -> ::windows::core::Result<f64>;
    fn Width(&mut self) -> ::windows::core::Result<f64>;
    fn SetWidth(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Height(&mut self) -> ::windows::core::Result<f64>;
    fn SetHeight(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MinWidth(&mut self) -> ::windows::core::Result<f64>;
    fn SetMinWidth(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MaxWidth(&mut self) -> ::windows::core::Result<f64>;
    fn SetMaxWidth(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MinHeight(&mut self) -> ::windows::core::Result<f64>;
    fn SetMinHeight(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn MaxHeight(&mut self) -> ::windows::core::Result<f64>;
    fn SetMaxHeight(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn HorizontalAlignment(&mut self) -> ::windows::core::Result<HorizontalAlignment>;
    fn SetHorizontalAlignment(&mut self, value: HorizontalAlignment) -> ::windows::core::Result<()>;
    fn VerticalAlignment(&mut self) -> ::windows::core::Result<VerticalAlignment>;
    fn SetVerticalAlignment(&mut self, value: VerticalAlignment) -> ::windows::core::Result<()>;
    fn Margin(&mut self) -> ::windows::core::Result<Thickness>;
    fn SetMargin(&mut self, value: &Thickness) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BaseUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn DataContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetDataContext(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Style(&mut self) -> ::windows::core::Result<Style>;
    fn SetStyle(&mut self, value: &::core::option::Option<Style>) -> ::windows::core::Result<()>;
    fn Parent(&mut self) -> ::windows::core::Result<DependencyObject>;
    fn FlowDirection(&mut self) -> ::windows::core::Result<FlowDirection>;
    fn SetFlowDirection(&mut self, value: FlowDirection) -> ::windows::core::Result<()>;
    fn Loaded(&mut self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoaded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unloaded(&mut self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnloaded(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&mut self, handler: &::core::option::Option<SizeChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LayoutUpdated(&mut self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLayoutUpdated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn FindName(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetBinding(&mut self, dp: &::core::option::Option<DependencyProperty>, binding: &::core::option::Option<Data::BindingBase>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Data", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElement {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Data", feature = "implement_exclusive"))]
impl IFrameworkElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElement_Vtbl {
        unsafe extern "system" fn Triggers<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Triggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resources<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResources<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResources(&*(&value as *const <ResourceDictionary as ::windows::core::Abi>::Abi as *const <ResourceDictionary as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tag<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Language<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Language() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLanguage<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualWidth<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualHeight<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Width<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Width() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWidth<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWidth(value).into()
        }
        unsafe extern "system" fn Height<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Height() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeight<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeight(value).into()
        }
        unsafe extern "system" fn MinWidth<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinWidth<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinWidth(value).into()
        }
        unsafe extern "system" fn MaxWidth<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxWidth<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxWidth(value).into()
        }
        unsafe extern "system" fn MinHeight<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinHeight<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinHeight(value).into()
        }
        unsafe extern "system" fn MaxHeight<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHeight<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxHeight(value).into()
        }
        unsafe extern "system" fn HorizontalAlignment<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignment<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignment(value).into()
        }
        unsafe extern "system" fn VerticalAlignment<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignment<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignment(value).into()
        }
        unsafe extern "system" fn Margin<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Margin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMargin<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMargin(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Name<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BaseUri<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataContext<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataContext<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Style<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Style() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStyle<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStyle(&*(&value as *const <Style as ::windows::core::Abi>::Abi as *const <Style as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Parent<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirection<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlowDirection<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlowDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFlowDirection(value).into()
        }
        unsafe extern "system" fn Loaded<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Loaded(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoaded<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoaded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unloaded<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unloaded(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnloaded<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnloaded(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SizeChanged<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeChanged(&*(&handler as *const <SizeChangedEventHandler as ::windows::core::Abi>::Abi as *const <SizeChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSizeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LayoutUpdated<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LayoutUpdated(&*(&handler as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLayoutUpdated<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLayoutUpdated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FindName<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBinding<Impl: IFrameworkElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, binding: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBinding(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&binding as *const <Data::BindingBase as ::windows::core::Abi>::Abi as *const <Data::BindingBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElement, BASE_OFFSET>(),
            Triggers: Triggers::<Impl, IMPL_OFFSET>,
            Resources: Resources::<Impl, IMPL_OFFSET>,
            SetResources: SetResources::<Impl, IMPL_OFFSET>,
            Tag: Tag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            Language: Language::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
            ActualWidth: ActualWidth::<Impl, IMPL_OFFSET>,
            ActualHeight: ActualHeight::<Impl, IMPL_OFFSET>,
            Width: Width::<Impl, IMPL_OFFSET>,
            SetWidth: SetWidth::<Impl, IMPL_OFFSET>,
            Height: Height::<Impl, IMPL_OFFSET>,
            SetHeight: SetHeight::<Impl, IMPL_OFFSET>,
            MinWidth: MinWidth::<Impl, IMPL_OFFSET>,
            SetMinWidth: SetMinWidth::<Impl, IMPL_OFFSET>,
            MaxWidth: MaxWidth::<Impl, IMPL_OFFSET>,
            SetMaxWidth: SetMaxWidth::<Impl, IMPL_OFFSET>,
            MinHeight: MinHeight::<Impl, IMPL_OFFSET>,
            SetMinHeight: SetMinHeight::<Impl, IMPL_OFFSET>,
            MaxHeight: MaxHeight::<Impl, IMPL_OFFSET>,
            SetMaxHeight: SetMaxHeight::<Impl, IMPL_OFFSET>,
            HorizontalAlignment: HorizontalAlignment::<Impl, IMPL_OFFSET>,
            SetHorizontalAlignment: SetHorizontalAlignment::<Impl, IMPL_OFFSET>,
            VerticalAlignment: VerticalAlignment::<Impl, IMPL_OFFSET>,
            SetVerticalAlignment: SetVerticalAlignment::<Impl, IMPL_OFFSET>,
            Margin: Margin::<Impl, IMPL_OFFSET>,
            SetMargin: SetMargin::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            BaseUri: BaseUri::<Impl, IMPL_OFFSET>,
            DataContext: DataContext::<Impl, IMPL_OFFSET>,
            SetDataContext: SetDataContext::<Impl, IMPL_OFFSET>,
            Style: Style::<Impl, IMPL_OFFSET>,
            SetStyle: SetStyle::<Impl, IMPL_OFFSET>,
            Parent: Parent::<Impl, IMPL_OFFSET>,
            FlowDirection: FlowDirection::<Impl, IMPL_OFFSET>,
            SetFlowDirection: SetFlowDirection::<Impl, IMPL_OFFSET>,
            Loaded: Loaded::<Impl, IMPL_OFFSET>,
            RemoveLoaded: RemoveLoaded::<Impl, IMPL_OFFSET>,
            Unloaded: Unloaded::<Impl, IMPL_OFFSET>,
            RemoveUnloaded: RemoveUnloaded::<Impl, IMPL_OFFSET>,
            SizeChanged: SizeChanged::<Impl, IMPL_OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Impl, IMPL_OFFSET>,
            LayoutUpdated: LayoutUpdated::<Impl, IMPL_OFFSET>,
            RemoveLayoutUpdated: RemoveLayoutUpdated::<Impl, IMPL_OFFSET>,
            FindName: FindName::<Impl, IMPL_OFFSET>,
            SetBinding: SetBinding::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Data", feature = "implement_exclusive"))]
pub trait IFrameworkElement2_Impl: Sized {
    fn RequestedTheme(&mut self) -> ::windows::core::Result<ElementTheme>;
    fn SetRequestedTheme(&mut self, value: ElementTheme) -> ::windows::core::Result<()>;
    fn DataContextChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataContextChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetBindingExpression(&mut self, dp: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<Data::BindingExpression>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Data", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement2";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Data", feature = "implement_exclusive"))]
impl IFrameworkElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElement2_Vtbl {
        unsafe extern "system" fn RequestedTheme<Impl: IFrameworkElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ElementTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedTheme<Impl: IFrameworkElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ElementTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedTheme(value).into()
        }
        unsafe extern "system" fn DataContextChanged<Impl: IFrameworkElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataContextChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, DataContextChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDataContextChanged<Impl: IFrameworkElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDataContextChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetBindingExpression<Impl: IFrameworkElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dp: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindingExpression(&*(&dp as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElement2, BASE_OFFSET>(),
            RequestedTheme: RequestedTheme::<Impl, IMPL_OFFSET>,
            SetRequestedTheme: SetRequestedTheme::<Impl, IMPL_OFFSET>,
            DataContextChanged: DataContextChanged::<Impl, IMPL_OFFSET>,
            RemoveDataContextChanged: RemoveDataContextChanged::<Impl, IMPL_OFFSET>,
            GetBindingExpression: GetBindingExpression::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFrameworkElement3_Impl: Sized {
    fn Loading(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLoading(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFrameworkElement3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElement3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElement3_Vtbl {
        unsafe extern "system" fn Loading<Impl: IFrameworkElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Loading(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLoading<Impl: IFrameworkElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLoading(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElement3, BASE_OFFSET>(),
            Loading: Loading::<Impl, IMPL_OFFSET>,
            RemoveLoading: RemoveLoading::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElement3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IFrameworkElement4_Impl: Sized {
    fn AllowFocusOnInteraction(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn FocusVisualMargin(&mut self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualMargin(&mut self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualSecondaryThickness(&mut self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualSecondaryThickness(&mut self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualPrimaryThickness(&mut self) -> ::windows::core::Result<Thickness>;
    fn SetFocusVisualPrimaryThickness(&mut self, value: &Thickness) -> ::windows::core::Result<()>;
    fn FocusVisualSecondaryBrush(&mut self) -> ::windows::core::Result<Media::Brush>;
    fn SetFocusVisualSecondaryBrush(&mut self, value: &::core::option::Option<Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusVisualPrimaryBrush(&mut self) -> ::windows::core::Result<Media::Brush>;
    fn SetFocusVisualPrimaryBrush(&mut self, value: &::core::option::Option<Media::Brush>) -> ::windows::core::Result<()>;
    fn AllowFocusWhenDisabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement4";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IFrameworkElement4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElement4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElement4_Vtbl {
        unsafe extern "system" fn AllowFocusOnInteraction<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteraction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusOnInteraction<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusOnInteraction(value).into()
        }
        unsafe extern "system" fn FocusVisualMargin<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualMargin<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusVisualMargin(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualSecondaryThickness<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualSecondaryThickness<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusVisualSecondaryThickness(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualPrimaryThickness<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualPrimaryThickness<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusVisualPrimaryThickness(&*(&value as *const <Thickness as ::windows::core::Abi>::Abi as *const <Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualSecondaryBrush<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualSecondaryBrush<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusVisualSecondaryBrush(&*(&value as *const <Media::Brush as ::windows::core::Abi>::Abi as *const <Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusVisualPrimaryBrush<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusVisualPrimaryBrush<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusVisualPrimaryBrush(&*(&value as *const <Media::Brush as ::windows::core::Abi>::Abi as *const <Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowFocusWhenDisabled<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusWhenDisabled<Impl: IFrameworkElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusWhenDisabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElement4, BASE_OFFSET>(),
            AllowFocusOnInteraction: AllowFocusOnInteraction::<Impl, IMPL_OFFSET>,
            SetAllowFocusOnInteraction: SetAllowFocusOnInteraction::<Impl, IMPL_OFFSET>,
            FocusVisualMargin: FocusVisualMargin::<Impl, IMPL_OFFSET>,
            SetFocusVisualMargin: SetFocusVisualMargin::<Impl, IMPL_OFFSET>,
            FocusVisualSecondaryThickness: FocusVisualSecondaryThickness::<Impl, IMPL_OFFSET>,
            SetFocusVisualSecondaryThickness: SetFocusVisualSecondaryThickness::<Impl, IMPL_OFFSET>,
            FocusVisualPrimaryThickness: FocusVisualPrimaryThickness::<Impl, IMPL_OFFSET>,
            SetFocusVisualPrimaryThickness: SetFocusVisualPrimaryThickness::<Impl, IMPL_OFFSET>,
            FocusVisualSecondaryBrush: FocusVisualSecondaryBrush::<Impl, IMPL_OFFSET>,
            SetFocusVisualSecondaryBrush: SetFocusVisualSecondaryBrush::<Impl, IMPL_OFFSET>,
            FocusVisualPrimaryBrush: FocusVisualPrimaryBrush::<Impl, IMPL_OFFSET>,
            SetFocusVisualPrimaryBrush: SetFocusVisualPrimaryBrush::<Impl, IMPL_OFFSET>,
            AllowFocusWhenDisabled: AllowFocusWhenDisabled::<Impl, IMPL_OFFSET>,
            SetAllowFocusWhenDisabled: SetAllowFocusWhenDisabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElement4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFrameworkElement6_Impl: Sized {
    fn ActualTheme(&mut self) -> ::windows::core::Result<ElementTheme>;
    fn ActualThemeChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualThemeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElement6 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement6";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFrameworkElement6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElement6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElement6_Vtbl {
        unsafe extern "system" fn ActualTheme<Impl: IFrameworkElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ElementTheme) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualTheme() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualThemeChanged<Impl: IFrameworkElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualThemeChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualThemeChanged<Impl: IFrameworkElement6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActualThemeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElement6, BASE_OFFSET>(),
            ActualTheme: ActualTheme::<Impl, IMPL_OFFSET>,
            ActualThemeChanged: ActualThemeChanged::<Impl, IMPL_OFFSET>,
            RemoveActualThemeChanged: RemoveActualThemeChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElement6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFrameworkElement7_Impl: Sized {
    fn IsLoaded(&mut self) -> ::windows::core::Result<bool>;
    fn EffectiveViewportChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEffectiveViewportChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElement7 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElement7";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFrameworkElement7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElement7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElement7_Vtbl {
        unsafe extern "system" fn IsLoaded<Impl: IFrameworkElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLoaded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EffectiveViewportChanged<Impl: IFrameworkElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveViewportChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<FrameworkElement, EffectiveViewportChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEffectiveViewportChanged<Impl: IFrameworkElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEffectiveViewportChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElement7, BASE_OFFSET>(),
            IsLoaded: IsLoaded::<Impl, IMPL_OFFSET>,
            EffectiveViewportChanged: EffectiveViewportChanged::<Impl, IMPL_OFFSET>,
            RemoveEffectiveViewportChanged: RemoveEffectiveViewportChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElement7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFrameworkElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFrameworkElementOverrides_Impl: Sized {
    fn MeasureOverride(&mut self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ArrangeOverride(&mut self, finalsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn OnApplyTemplate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFrameworkElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementOverrides_Vtbl {
        unsafe extern "system" fn MeasureOverride<Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablesize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasureOverride(&*(&availablesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalsize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrangeOverride(&*(&finalsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnApplyTemplate<Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnApplyTemplate().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementOverrides, BASE_OFFSET>(),
            MeasureOverride: MeasureOverride::<Impl, IMPL_OFFSET>,
            ArrangeOverride: ArrangeOverride::<Impl, IMPL_OFFSET>,
            OnApplyTemplate: OnApplyTemplate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementOverrides2_Impl: Sized {
    fn GoToElementStateCore(&mut self, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementOverrides2_Vtbl {
        unsafe extern "system" fn GoToElementStateCore<Impl: IFrameworkElementOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoToElementStateCore(&*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementOverrides2, BASE_OFFSET>(),
            GoToElementStateCore: GoToElementStateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementProtected7_Impl: Sized {
    fn InvalidateViewport(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementProtected7 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementProtected7";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementProtected7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementProtected7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementProtected7_Vtbl {
        unsafe extern "system" fn InvalidateViewport<Impl: IFrameworkElementProtected7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateViewport().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementProtected7, BASE_OFFSET>(),
            InvalidateViewport: InvalidateViewport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementProtected7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics_Impl: Sized {
    fn TagProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn LanguageProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn ActualWidthProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn ActualHeightProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn WidthProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn HeightProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn MinWidthProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn MaxWidthProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn MinHeightProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn MaxHeightProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn HorizontalAlignmentProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn VerticalAlignmentProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn MarginProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn NameProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn DataContextProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn StyleProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn FlowDirectionProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementStatics_Vtbl {
        unsafe extern "system" fn TagProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TagProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LanguageProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LanguageProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualWidthProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualHeightProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeightProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinWidthProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxWidthProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinHeightProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxHeightProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalAlignmentProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalAlignmentProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MarginProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataContextProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataContextProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StyleProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StyleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlowDirectionProperty<Impl: IFrameworkElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlowDirectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementStatics, BASE_OFFSET>(),
            TagProperty: TagProperty::<Impl, IMPL_OFFSET>,
            LanguageProperty: LanguageProperty::<Impl, IMPL_OFFSET>,
            ActualWidthProperty: ActualWidthProperty::<Impl, IMPL_OFFSET>,
            ActualHeightProperty: ActualHeightProperty::<Impl, IMPL_OFFSET>,
            WidthProperty: WidthProperty::<Impl, IMPL_OFFSET>,
            HeightProperty: HeightProperty::<Impl, IMPL_OFFSET>,
            MinWidthProperty: MinWidthProperty::<Impl, IMPL_OFFSET>,
            MaxWidthProperty: MaxWidthProperty::<Impl, IMPL_OFFSET>,
            MinHeightProperty: MinHeightProperty::<Impl, IMPL_OFFSET>,
            MaxHeightProperty: MaxHeightProperty::<Impl, IMPL_OFFSET>,
            HorizontalAlignmentProperty: HorizontalAlignmentProperty::<Impl, IMPL_OFFSET>,
            VerticalAlignmentProperty: VerticalAlignmentProperty::<Impl, IMPL_OFFSET>,
            MarginProperty: MarginProperty::<Impl, IMPL_OFFSET>,
            NameProperty: NameProperty::<Impl, IMPL_OFFSET>,
            DataContextProperty: DataContextProperty::<Impl, IMPL_OFFSET>,
            StyleProperty: StyleProperty::<Impl, IMPL_OFFSET>,
            FlowDirectionProperty: FlowDirectionProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics2_Impl: Sized {
    fn RequestedThemeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementStatics2_Vtbl {
        unsafe extern "system" fn RequestedThemeProperty<Impl: IFrameworkElementStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedThemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementStatics2, BASE_OFFSET>(),
            RequestedThemeProperty: RequestedThemeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics4_Impl: Sized {
    fn AllowFocusOnInteractionProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualMarginProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualSecondaryThicknessProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualPrimaryThicknessProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualSecondaryBrushProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn FocusVisualPrimaryBrushProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn AllowFocusWhenDisabledProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementStatics4_Vtbl {
        unsafe extern "system" fn AllowFocusOnInteractionProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteractionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualMarginProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualSecondaryThicknessProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualPrimaryThicknessProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualSecondaryBrushProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualSecondaryBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusVisualPrimaryBrushProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusVisualPrimaryBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowFocusWhenDisabledProperty<Impl: IFrameworkElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementStatics4, BASE_OFFSET>(),
            AllowFocusOnInteractionProperty: AllowFocusOnInteractionProperty::<Impl, IMPL_OFFSET>,
            FocusVisualMarginProperty: FocusVisualMarginProperty::<Impl, IMPL_OFFSET>,
            FocusVisualSecondaryThicknessProperty: FocusVisualSecondaryThicknessProperty::<Impl, IMPL_OFFSET>,
            FocusVisualPrimaryThicknessProperty: FocusVisualPrimaryThicknessProperty::<Impl, IMPL_OFFSET>,
            FocusVisualSecondaryBrushProperty: FocusVisualSecondaryBrushProperty::<Impl, IMPL_OFFSET>,
            FocusVisualPrimaryBrushProperty: FocusVisualPrimaryBrushProperty::<Impl, IMPL_OFFSET>,
            AllowFocusWhenDisabledProperty: AllowFocusWhenDisabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics5_Impl: Sized {
    fn DeferTree(&mut self, element: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementStatics5_Vtbl {
        unsafe extern "system" fn DeferTree<Impl: IFrameworkElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeferTree(&*(&element as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementStatics5, BASE_OFFSET>(), DeferTree: DeferTree::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementStatics6_Impl: Sized {
    fn ActualThemeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementStatics6_Vtbl {
        unsafe extern "system" fn ActualThemeProperty<Impl: IFrameworkElementStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualThemeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementStatics6, BASE_OFFSET>(),
            ActualThemeProperty: ActualThemeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkTemplate_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkTemplate {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkTemplate";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkTemplate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkTemplate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkTemplate_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkTemplate, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkTemplate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkTemplateFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkTemplate>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkTemplateFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkTemplateFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkTemplateFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkTemplateFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkTemplateFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFrameworkTemplateFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkTemplateFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkTemplateFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkView_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkView {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkView";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkView_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkView_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkView, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkView as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkViewSource_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkViewSource {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkViewSource";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkViewSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkViewSource_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkViewSource_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkViewSource, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkViewSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridLengthHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridLengthHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IGridLengthHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IGridLengthHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridLengthHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridLengthHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGridLengthHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridLengthHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridLengthHelperStatics_Impl: Sized {
    fn Auto(&mut self) -> ::windows::core::Result<GridLength>;
    fn FromPixels(&mut self, pixels: f64) -> ::windows::core::Result<GridLength>;
    fn FromValueAndType(&mut self, value: f64, r#type: GridUnitType) -> ::windows::core::Result<GridLength>;
    fn GetIsAbsolute(&mut self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn GetIsAuto(&mut self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn GetIsStar(&mut self, target: &GridLength) -> ::windows::core::Result<bool>;
    fn Equals(&mut self, target: &GridLength, value: &GridLength) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridLengthHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IGridLengthHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridLengthHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridLengthHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridLengthHelperStatics_Vtbl {
        unsafe extern "system" fn Auto<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Auto() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromPixels<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pixels: f64, result__: *mut GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromPixels(pixels) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromValueAndType<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, r#type: GridUnitType, result__: *mut GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromValueAndType(value, r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAbsolute<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsAbsolute(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAuto<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsAuto(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsStar<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsStar(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IGridLengthHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: GridLength, value: GridLength, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <GridLength as ::windows::core::Abi>::Abi as *const <GridLength as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridLengthHelperStatics, BASE_OFFSET>(),
            Auto: Auto::<Impl, IMPL_OFFSET>,
            FromPixels: FromPixels::<Impl, IMPL_OFFSET>,
            FromValueAndType: FromValueAndType::<Impl, IMPL_OFFSET>,
            GetIsAbsolute: GetIsAbsolute::<Impl, IMPL_OFFSET>,
            GetIsAuto: GetIsAuto::<Impl, IMPL_OFFSET>,
            GetIsStar: GetIsStar::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridLengthHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFailedRoutedEventArgs_Impl: Sized {
    fn ErrorTrace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaFailedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IMediaFailedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaFailedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaFailedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaFailedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn ErrorTrace<Impl: IMediaFailedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorTrace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaFailedRoutedEventArgs, BASE_OFFSET>(),
            ErrorTrace: ErrorTrace::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaFailedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IPointHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IPointHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPointHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPointHelperStatics_Impl: Sized {
    fn FromCoordinates(&mut self, x: f32, y: f32) -> ::windows::core::Result<super::super::Foundation::Point>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IPointHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPointHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointHelperStatics_Vtbl {
        unsafe extern "system" fn FromCoordinates<Impl: IPointHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromCoordinates(x, y) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointHelperStatics, BASE_OFFSET>(),
            FromCoordinates: FromCoordinates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadata_Impl: Sized {
    fn DefaultValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDefaultValueCallback(&mut self) -> ::windows::core::Result<CreateDefaultValueCallback>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyMetadata {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyMetadata";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyMetadata_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyMetadata_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyMetadata_Vtbl {
        unsafe extern "system" fn DefaultValue<Impl: IPropertyMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDefaultValueCallback<Impl: IPropertyMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDefaultValueCallback() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyMetadata, BASE_OFFSET>(),
            DefaultValue: DefaultValue::<Impl, IMPL_OFFSET>,
            CreateDefaultValueCallback: CreateDefaultValueCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyMetadata as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataFactory_Impl: Sized {
    fn CreateInstanceWithDefaultValue(&mut self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateInstanceWithDefaultValueAndCallback(&mut self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyMetadataFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyMetadataFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyMetadataFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyMetadataFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyMetadataFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithDefaultValue<Impl: IPropertyMetadataFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithDefaultValue(&*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceWithDefaultValueAndCallback<Impl: IPropertyMetadataFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, propertychangedcallback: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyMetadataFactory, BASE_OFFSET>(),
            CreateInstanceWithDefaultValue: CreateInstanceWithDefaultValue::<Impl, IMPL_OFFSET>,
            CreateInstanceWithDefaultValueAndCallback: CreateInstanceWithDefaultValueAndCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyMetadataFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyMetadataStatics_Impl: Sized {
    fn CreateWithDefaultValue(&mut self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithDefaultValueAndCallback(&mut self, defaultvalue: &::core::option::Option<::windows::core::IInspectable>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithFactory(&mut self, createdefaultvaluecallback: &::core::option::Option<CreateDefaultValueCallback>) -> ::windows::core::Result<PropertyMetadata>;
    fn CreateWithFactoryAndCallback(&mut self, createdefaultvaluecallback: &::core::option::Option<CreateDefaultValueCallback>, propertychangedcallback: &::core::option::Option<PropertyChangedCallback>) -> ::windows::core::Result<PropertyMetadata>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyMetadataStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyMetadataStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyMetadataStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyMetadataStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyMetadataStatics_Vtbl {
        unsafe extern "system" fn CreateWithDefaultValue<Impl: IPropertyMetadataStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithDefaultValue(&*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithDefaultValueAndCallback<Impl: IPropertyMetadataStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::core::ffi::c_void, propertychangedcallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithDefaultValueAndCallback(&*(&defaultvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertychangedcallback as *const <PropertyChangedCallback as ::windows::core::Abi>::Abi as *const <PropertyChangedCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithFactory<Impl: IPropertyMetadataStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, createdefaultvaluecallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithFactory(&*(&createdefaultvaluecallback as *const <CreateDefaultValueCallback as ::windows::core::Abi>::Abi as *const <CreateDefaultValueCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithFactoryAndCallback<Impl: IPropertyMetadataStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, createdefaultvaluecallback: ::windows::core::RawPtr, propertychangedcallback: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithFactoryAndCallback(&*(&createdefaultvaluecallback as *const <CreateDefaultValueCallback as ::windows::core::Abi>::Abi as *const <CreateDefaultValueCallback as ::windows::core::DefaultType>::DefaultType), &*(&propertychangedcallback as *const <PropertyChangedCallback as ::windows::core::Abi>::Abi as *const <PropertyChangedCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyMetadataStatics, BASE_OFFSET>(),
            CreateWithDefaultValue: CreateWithDefaultValue::<Impl, IMPL_OFFSET>,
            CreateWithDefaultValueAndCallback: CreateWithDefaultValueAndCallback::<Impl, IMPL_OFFSET>,
            CreateWithFactory: CreateWithFactory::<Impl, IMPL_OFFSET>,
            CreateWithFactoryAndCallback: CreateWithFactoryAndCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyMetadataStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyPath_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyPath {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyPath";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPath_Vtbl {
        unsafe extern "system" fn Path<Impl: IPropertyPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyPath, BASE_OFFSET>(), Path: Path::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPath as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyPathFactory_Impl: Sized {
    fn CreateInstance(&mut self, path: &::windows::core::HSTRING) -> ::windows::core::Result<PropertyPath>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyPathFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IPropertyPathFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyPathFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyPathFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyPathFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPropertyPathFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&path as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyPathFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyPathFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRectHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRectHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IRectHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IRectHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRectHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRectHelperStatics_Impl: Sized {
    fn Empty(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromCoordinatesAndDimensions(&mut self, x: f32, y: f32, width: f32, height: f32) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromPoints(&mut self, point1: &super::super::Foundation::Point, point2: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn FromLocationAndSize(&mut self, location: &super::super::Foundation::Point, size: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn GetIsEmpty(&mut self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
    fn GetBottom(&mut self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetLeft(&mut self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetRight(&mut self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn GetTop(&mut self, target: &super::super::Foundation::Rect) -> ::windows::core::Result<f32>;
    fn Contains(&mut self, target: &super::super::Foundation::Rect, point: &super::super::Foundation::Point) -> ::windows::core::Result<bool>;
    fn Equals(&mut self, target: &super::super::Foundation::Rect, value: &super::super::Foundation::Rect) -> ::windows::core::Result<bool>;
    fn Intersect(&mut self, target: &super::super::Foundation::Rect, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn UnionWithPoint(&mut self, target: &super::super::Foundation::Rect, point: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn UnionWithRect(&mut self, target: &super::super::Foundation::Rect, rect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRectHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IRectHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRectHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRectHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRectHelperStatics_Vtbl {
        unsafe extern "system" fn Empty<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromCoordinatesAndDimensions<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, width: f32, height: f32, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromCoordinatesAndDimensions(x, y, width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromPoints<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point1: super::super::Foundation::Point, point2: super::super::Foundation::Point, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromPoints(&*(&point1 as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&point2 as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromLocationAndSize<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, location: super::super::Foundation::Point, size: super::super::Foundation::Size, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromLocationAndSize(&*(&location as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&size as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsEmpty<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsEmpty(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBottom<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBottom(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLeft<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLeft(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRight<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRight(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTop<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTop(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Contains<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, point: super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Contains(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, value: super::super::Foundation::Rect, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Intersect<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Intersect(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnionWithPoint<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, point: super::super::Foundation::Point, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnionWithPoint(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnionWithRect<Impl: IRectHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Rect, rect: super::super::Foundation::Rect, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnionWithRect(&*(&target as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType), &*(&rect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRectHelperStatics, BASE_OFFSET>(),
            Empty: Empty::<Impl, IMPL_OFFSET>,
            FromCoordinatesAndDimensions: FromCoordinatesAndDimensions::<Impl, IMPL_OFFSET>,
            FromPoints: FromPoints::<Impl, IMPL_OFFSET>,
            FromLocationAndSize: FromLocationAndSize::<Impl, IMPL_OFFSET>,
            GetIsEmpty: GetIsEmpty::<Impl, IMPL_OFFSET>,
            GetBottom: GetBottom::<Impl, IMPL_OFFSET>,
            GetLeft: GetLeft::<Impl, IMPL_OFFSET>,
            GetRight: GetRight::<Impl, IMPL_OFFSET>,
            GetTop: GetTop::<Impl, IMPL_OFFSET>,
            Contains: Contains::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
            Intersect: Intersect::<Impl, IMPL_OFFSET>,
            UnionWithPoint: UnionWithPoint::<Impl, IMPL_OFFSET>,
            UnionWithRect: UnionWithRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRectHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IResourceDictionary_Impl: Sized {
    fn Source(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSource(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn MergedDictionaries(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ResourceDictionary>>;
    fn ThemeDictionaries(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::IInspectable, ::windows::core::IInspectable>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IResourceDictionary {
    const NAME: &'static str = "Windows.UI.Xaml.IResourceDictionary";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IResourceDictionary_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceDictionary_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceDictionary_Vtbl {
        unsafe extern "system" fn Source<Impl: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Source() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSource<Impl: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSource(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MergedDictionaries<Impl: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MergedDictionaries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ThemeDictionaries<Impl: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ThemeDictionaries() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceDictionary, BASE_OFFSET>(),
            Source: Source::<Impl, IMPL_OFFSET>,
            SetSource: SetSource::<Impl, IMPL_OFFSET>,
            MergedDictionaries: MergedDictionaries::<Impl, IMPL_OFFSET>,
            ThemeDictionaries: ThemeDictionaries::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceDictionary as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IResourceDictionaryFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ResourceDictionary>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IResourceDictionaryFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IResourceDictionaryFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IResourceDictionaryFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IResourceDictionaryFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IResourceDictionaryFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IResourceDictionaryFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IResourceDictionaryFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IResourceDictionaryFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEvent_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutedEvent {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEvent";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutedEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutedEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoutedEvent_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRoutedEvent, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoutedEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventArgs_Impl: Sized {
    fn OriginalSource(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoutedEventArgs_Vtbl {
        unsafe extern "system" fn OriginalSource<Impl: IRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRoutedEventArgs, BASE_OFFSET>(), OriginalSource: OriginalSource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRoutedEventArgsFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRoutedEventArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoutedEventArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoutedEventArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRoutedEventArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRoutedEventArgsFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoutedEventArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IScalarTransition_Impl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScalarTransition {
    const NAME: &'static str = "Windows.UI.Xaml.IScalarTransition";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScalarTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarTransition_Vtbl {
        unsafe extern "system" fn Duration<Impl: IScalarTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IScalarTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScalarTransition, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScalarTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScalarTransitionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScalarTransition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScalarTransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IScalarTransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScalarTransitionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScalarTransitionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScalarTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IScalarTransitionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScalarTransitionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScalarTransitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetter_Impl: Sized {
    fn Property(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn SetProperty(&mut self, value: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetter {
    const NAME: &'static str = "Windows.UI.Xaml.ISetter";
}
#[cfg(feature = "implement_exclusive")]
impl ISetter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetter_Vtbl {
        unsafe extern "system" fn Property<Impl: ISetter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: ISetter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(&*(&value as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Value<Impl: ISetter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISetter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISetter, BASE_OFFSET>(),
            Property: Property::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetter2_Impl: Sized {
    fn Target(&mut self) -> ::windows::core::Result<TargetPropertyPath>;
    fn SetTarget(&mut self, value: &::core::option::Option<TargetPropertyPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetter2 {
    const NAME: &'static str = "Windows.UI.Xaml.ISetter2";
}
#[cfg(feature = "implement_exclusive")]
impl ISetter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetter2_Vtbl {
        unsafe extern "system" fn Target<Impl: ISetter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTarget<Impl: ISetter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&value as *const <TargetPropertyPath as ::windows::core::Abi>::Abi as *const <TargetPropertyPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISetter2, BASE_OFFSET>(),
            Target: Target::<Impl, IMPL_OFFSET>,
            SetTarget: SetTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBase_Impl: Sized {
    fn IsSealed(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterBase {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterBase";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetterBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetterBase_Vtbl {
        unsafe extern "system" fn IsSealed<Impl: ISetterBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSealed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISetterBase, BASE_OFFSET>(), IsSealed: IsSealed::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetterBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseCollection_Impl: Sized {
    fn IsSealed(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterBaseCollection {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterBaseCollection";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterBaseCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetterBaseCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetterBaseCollection_Vtbl {
        unsafe extern "system" fn IsSealed<Impl: ISetterBaseCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSealed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISetterBaseCollection, BASE_OFFSET>(), IsSealed: IsSealed::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetterBaseCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterBaseFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetterBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetterBaseFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISetterBaseFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetterBaseFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISetterFactory_Impl: Sized {
    fn CreateInstance(&mut self, targetproperty: &::core::option::Option<DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Setter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISetterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ISetterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISetterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISetterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISetterFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISetterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetproperty: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&targetproperty as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISetterFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISetterFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISizeChangedEventArgs_Impl: Sized {
    fn PreviousSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn NewSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.ISizeChangedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISizeChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISizeChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISizeChangedEventArgs_Vtbl {
        unsafe extern "system" fn PreviousSize<Impl: ISizeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviousSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewSize<Impl: ISizeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISizeChangedEventArgs, BASE_OFFSET>(),
            PreviousSize: PreviousSize::<Impl, IMPL_OFFSET>,
            NewSize: NewSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISizeChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISizeHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISizeHelper {
    const NAME: &'static str = "Windows.UI.Xaml.ISizeHelper";
}
#[cfg(feature = "implement_exclusive")]
impl ISizeHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISizeHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISizeHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISizeHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISizeHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ISizeHelperStatics_Impl: Sized {
    fn Empty(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn FromDimensions(&mut self, width: f32, height: f32) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn GetIsEmpty(&mut self, target: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
    fn Equals(&mut self, target: &super::super::Foundation::Size, value: &super::super::Foundation::Size) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISizeHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.ISizeHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISizeHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISizeHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISizeHelperStatics_Vtbl {
        unsafe extern "system" fn Empty<Impl: ISizeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromDimensions<Impl: ISizeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: f32, height: f32, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromDimensions(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsEmpty<Impl: ISizeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsEmpty(&*(&target as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: ISizeHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: super::super::Foundation::Size, value: super::super::Foundation::Size, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&target as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISizeHelperStatics, BASE_OFFSET>(),
            Empty: Empty::<Impl, IMPL_OFFSET>,
            FromDimensions: FromDimensions::<Impl, IMPL_OFFSET>,
            GetIsEmpty: GetIsEmpty::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISizeHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTrigger_Impl: Sized {
    fn IsActive(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsActive(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTrigger {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTrigger";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStateTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStateTrigger_Vtbl {
        unsafe extern "system" fn IsActive<Impl: IStateTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsActive<Impl: IStateTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsActive(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStateTrigger, BASE_OFFSET>(),
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            SetIsActive: SetIsActive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStateTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerBase {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerBase";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStateTriggerBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStateTriggerBase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStateTriggerBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStateTriggerBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StateTriggerBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStateTriggerBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStateTriggerBaseFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStateTriggerBaseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStateTriggerBaseFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStateTriggerBaseFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerBaseProtected_Impl: Sized {
    fn SetActive(&mut self, isactive: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerBaseProtected {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerBaseProtected";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerBaseProtected_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStateTriggerBaseProtected_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStateTriggerBaseProtected_Vtbl {
        unsafe extern "system" fn SetActive<Impl: IStateTriggerBaseProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActive(isactive).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStateTriggerBaseProtected, BASE_OFFSET>(), SetActive: SetActive::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStateTriggerBaseProtected as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStateTriggerStatics_Impl: Sized {
    fn IsActiveProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStateTriggerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IStateTriggerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStateTriggerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStateTriggerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStateTriggerStatics_Vtbl {
        unsafe extern "system" fn IsActiveProperty<Impl: IStateTriggerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActiveProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStateTriggerStatics, BASE_OFFSET>(),
            IsActiveProperty: IsActiveProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStateTriggerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait IStyle_Impl: Sized {
    fn IsSealed(&mut self) -> ::windows::core::Result<bool>;
    fn Setters(&mut self) -> ::windows::core::Result<SetterBaseCollection>;
    fn TargetType(&mut self) -> ::windows::core::Result<Interop::TypeName>;
    fn SetTargetType(&mut self, value: &Interop::TypeName) -> ::windows::core::Result<()>;
    fn BasedOn(&mut self) -> ::windows::core::Result<Style>;
    fn SetBasedOn(&mut self, value: &::core::option::Option<Style>) -> ::windows::core::Result<()>;
    fn Seal(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStyle {
    const NAME: &'static str = "Windows.UI.Xaml.IStyle";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl IStyle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStyle_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStyle_Vtbl {
        unsafe extern "system" fn IsSealed<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSealed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setters<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetType<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetType<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetType(&*(&value as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn BasedOn<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BasedOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBasedOn<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBasedOn(&*(&value as *const <Style as ::windows::core::Abi>::Abi as *const <Style as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Seal<Impl: IStyle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seal().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStyle, BASE_OFFSET>(),
            IsSealed: IsSealed::<Impl, IMPL_OFFSET>,
            Setters: Setters::<Impl, IMPL_OFFSET>,
            TargetType: TargetType::<Impl, IMPL_OFFSET>,
            SetTargetType: SetTargetType::<Impl, IMPL_OFFSET>,
            BasedOn: BasedOn::<Impl, IMPL_OFFSET>,
            SetBasedOn: SetBasedOn::<Impl, IMPL_OFFSET>,
            Seal: Seal::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStyle as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait IStyleFactory_Impl: Sized {
    fn CreateInstance(&mut self, targettype: &Interop::TypeName) -> ::windows::core::Result<Style>;
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStyleFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IStyleFactory";
}
#[cfg(all(feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl IStyleFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStyleFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStyleFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStyleFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&targettype as *const <Interop::TypeName as ::windows::core::Abi>::Abi as *const <Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStyleFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStyleFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetPropertyPath_Impl: Sized {
    fn Path(&mut self) -> ::windows::core::Result<PropertyPath>;
    fn SetPath(&mut self, value: &::core::option::Option<PropertyPath>) -> ::windows::core::Result<()>;
    fn Target(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTarget(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetPropertyPath {
    const NAME: &'static str = "Windows.UI.Xaml.ITargetPropertyPath";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetPropertyPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetPropertyPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetPropertyPath_Vtbl {
        unsafe extern "system" fn Path<Impl: ITargetPropertyPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: ITargetPropertyPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(&*(&value as *const <PropertyPath as ::windows::core::Abi>::Abi as *const <PropertyPath as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Target<Impl: ITargetPropertyPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTarget<Impl: ITargetPropertyPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTarget(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetPropertyPath, BASE_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            Target: Target::<Impl, IMPL_OFFSET>,
            SetTarget: SetTarget::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetPropertyPath as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetPropertyPathFactory_Impl: Sized {
    fn CreateInstance(&mut self, targetproperty: &::core::option::Option<DependencyProperty>) -> ::windows::core::Result<TargetPropertyPath>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITargetPropertyPathFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ITargetPropertyPathFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITargetPropertyPathFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITargetPropertyPathFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITargetPropertyPathFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITargetPropertyPathFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targetproperty: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&targetproperty as *const <DependencyProperty as ::windows::core::Abi>::Abi as *const <DependencyProperty as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITargetPropertyPathFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITargetPropertyPathFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThicknessHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThicknessHelper {
    const NAME: &'static str = "Windows.UI.Xaml.IThicknessHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IThicknessHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThicknessHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThicknessHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IThicknessHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThicknessHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThicknessHelperStatics_Impl: Sized {
    fn FromLengths(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> ::windows::core::Result<Thickness>;
    fn FromUniformLength(&mut self, uniformlength: f64) -> ::windows::core::Result<Thickness>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThicknessHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IThicknessHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IThicknessHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThicknessHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThicknessHelperStatics_Vtbl {
        unsafe extern "system" fn FromLengths<Impl: IThicknessHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f64, top: f64, right: f64, bottom: f64, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromLengths(left, top, right, bottom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromUniformLength<Impl: IThicknessHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uniformlength: f64, result__: *mut Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromUniformLength(uniformlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThicknessHelperStatics, BASE_OFFSET>(),
            FromLengths: FromLengths::<Impl, IMPL_OFFSET>,
            FromUniformLength: FromUniformLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThicknessHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerAction_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerAction {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerAction";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerAction_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITriggerAction, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerActionFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerActionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerActionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerActionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerActionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerActionFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITriggerActionFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerActionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerBase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerBase {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerBase";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerBase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITriggerBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITriggerBaseFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITriggerBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.ITriggerBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITriggerBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITriggerBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITriggerBaseFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITriggerBaseFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITriggerBaseFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IUIElement_Impl: Sized {
    fn DesiredSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn AllowDrop(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowDrop(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Opacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Clip(&mut self) -> ::windows::core::Result<Media::RectangleGeometry>;
    fn SetClip(&mut self, value: &::core::option::Option<Media::RectangleGeometry>) -> ::windows::core::Result<()>;
    fn RenderTransform(&mut self) -> ::windows::core::Result<Media::Transform>;
    fn SetRenderTransform(&mut self, value: &::core::option::Option<Media::Transform>) -> ::windows::core::Result<()>;
    fn Projection(&mut self) -> ::windows::core::Result<Media::Projection>;
    fn SetProjection(&mut self, value: &::core::option::Option<Media::Projection>) -> ::windows::core::Result<()>;
    fn RenderTransformOrigin(&mut self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetRenderTransformOrigin(&mut self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn IsHitTestVisible(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsHitTestVisible(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Visibility(&mut self) -> ::windows::core::Result<Visibility>;
    fn SetVisibility(&mut self, value: Visibility) -> ::windows::core::Result<()>;
    fn RenderSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn UseLayoutRounding(&mut self) -> ::windows::core::Result<bool>;
    fn SetUseLayoutRounding(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Transitions(&mut self) -> ::windows::core::Result<Media::Animation::TransitionCollection>;
    fn SetTransitions(&mut self, value: &::core::option::Option<Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn CacheMode(&mut self) -> ::windows::core::Result<Media::CacheMode>;
    fn SetCacheMode(&mut self, value: &::core::option::Option<Media::CacheMode>) -> ::windows::core::Result<()>;
    fn IsTapEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsTapEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsDoubleTapEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsDoubleTapEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsRightTapEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsRightTapEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsHoldingEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsHoldingEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ManipulationMode(&mut self) -> ::windows::core::Result<Input::ManipulationModes>;
    fn SetManipulationMode(&mut self, value: Input::ManipulationModes) -> ::windows::core::Result<()>;
    fn PointerCaptures(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Input::Pointer>>;
    fn KeyUp(&mut self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn KeyDown(&mut self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GotFocus(&mut self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&mut self, handler: &::core::option::Option<RoutedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragEnter(&mut self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragEnter(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragLeave(&mut self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragLeave(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragOver(&mut self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragOver(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Drop(&mut self, handler: &::core::option::Option<DragEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrop(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerPressed(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerMoved(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerReleased(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerEntered(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerExited(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCaptureLost(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerCanceled(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCanceled(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PointerWheelChanged(&mut self, handler: &::core::option::Option<Input::PointerEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Tapped(&mut self, handler: &::core::option::Option<Input::TappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTapped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DoubleTapped(&mut self, handler: &::core::option::Option<Input::DoubleTappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDoubleTapped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Holding(&mut self, handler: &::core::option::Option<Input::HoldingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHolding(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RightTapped(&mut self, handler: &::core::option::Option<Input::RightTappedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRightTapped(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarting(&mut self, handler: &::core::option::Option<Input::ManipulationStartingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationInertiaStarting(&mut self, handler: &::core::option::Option<Input::ManipulationInertiaStartingEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationInertiaStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationStarted(&mut self, handler: &::core::option::Option<Input::ManipulationStartedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationStarted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationDelta(&mut self, handler: &::core::option::Option<Input::ManipulationDeltaEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationDelta(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ManipulationCompleted(&mut self, handler: &::core::option::Option<Input::ManipulationCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveManipulationCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Measure(&mut self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn Arrange(&mut self, finalrect: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn CapturePointer(&mut self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<bool>;
    fn ReleasePointerCapture(&mut self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<()>;
    fn ReleasePointerCaptures(&mut self) -> ::windows::core::Result<()>;
    fn AddHandler(&mut self, routedevent: &::core::option::Option<RoutedEvent>, handler: &::core::option::Option<::windows::core::IInspectable>, handledeventstoo: bool) -> ::windows::core::Result<()>;
    fn RemoveHandler(&mut self, routedevent: &::core::option::Option<RoutedEvent>, handler: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransformToVisual(&mut self, visual: &::core::option::Option<UIElement>) -> ::windows::core::Result<Media::GeneralTransform>;
    fn InvalidateMeasure(&mut self) -> ::windows::core::Result<()>;
    fn InvalidateArrange(&mut self) -> ::windows::core::Result<()>;
    fn UpdateLayout(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IUIElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement_Vtbl {
        unsafe extern "system" fn DesiredSize<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowDrop<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowDrop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowDrop<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowDrop(value).into()
        }
        unsafe extern "system" fn Opacity<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacity<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacity(value).into()
        }
        unsafe extern "system" fn Clip<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClip<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClip(&*(&value as *const <Media::RectangleGeometry as ::windows::core::Abi>::Abi as *const <Media::RectangleGeometry as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RenderTransform<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTransform<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenderTransform(&*(&value as *const <Media::Transform as ::windows::core::Abi>::Abi as *const <Media::Transform as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Projection<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Projection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProjection<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProjection(&*(&value as *const <Media::Projection as ::windows::core::Abi>::Abi as *const <Media::Projection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RenderTransformOrigin<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTransformOrigin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRenderTransformOrigin<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRenderTransformOrigin(&*(&value as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsHitTestVisible<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHitTestVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHitTestVisible<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHitTestVisible(value).into()
        }
        unsafe extern "system" fn Visibility<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Visibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVisibility<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Visibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVisibility(value).into()
        }
        unsafe extern "system" fn RenderSize<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseLayoutRounding<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseLayoutRounding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseLayoutRounding<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseLayoutRounding(value).into()
        }
        unsafe extern "system" fn Transitions<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransitions<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransitions(&*(&value as *const <Media::Animation::TransitionCollection as ::windows::core::Abi>::Abi as *const <Media::Animation::TransitionCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CacheMode<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCacheMode<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCacheMode(&*(&value as *const <Media::CacheMode as ::windows::core::Abi>::Abi as *const <Media::CacheMode as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsTapEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTapEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTapEnabled(value).into()
        }
        unsafe extern "system" fn IsDoubleTapEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleTapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsDoubleTapEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsDoubleTapEnabled(value).into()
        }
        unsafe extern "system" fn IsRightTapEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRightTapEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRightTapEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRightTapEnabled(value).into()
        }
        unsafe extern "system" fn IsHoldingEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHoldingEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHoldingEnabled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHoldingEnabled(value).into()
        }
        unsafe extern "system" fn ManipulationMode<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManipulationMode<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManipulationMode(value).into()
        }
        unsafe extern "system" fn PointerCaptures<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCaptures() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUp<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUp(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyUp(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyDown<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDown(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveKeyDown(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GotFocus<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <RoutedEventHandler as ::windows::core::Abi>::Abi as *const <RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragEnter<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragEnter(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragEnter<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragEnter(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragLeave<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragLeave(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragLeave<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragLeave(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragOver<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOver(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragOver<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragOver(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Drop<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Drop(&*(&handler as *const <DragEventHandler as ::windows::core::Abi>::Abi as *const <DragEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDrop<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDrop(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerPressed<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPressed(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerPressed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerMoved<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerMoved(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerMoved(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerReleased<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerReleased(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerReleased(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerEntered<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerEntered(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerEntered(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerExited<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerExited(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerExited<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerExited(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerCaptureLost<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCaptureLost(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerCaptureLost(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerCanceled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCanceled(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerCanceled<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerWheelChanged<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerWheelChanged(&*(&handler as *const <Input::PointerEventHandler as ::windows::core::Abi>::Abi as *const <Input::PointerEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePointerWheelChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Tapped<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tapped(&*(&handler as *const <Input::TappedEventHandler as ::windows::core::Abi>::Abi as *const <Input::TappedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTapped<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DoubleTapped<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoubleTapped(&*(&handler as *const <Input::DoubleTappedEventHandler as ::windows::core::Abi>::Abi as *const <Input::DoubleTappedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDoubleTapped<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDoubleTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Holding<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Holding(&*(&handler as *const <Input::HoldingEventHandler as ::windows::core::Abi>::Abi as *const <Input::HoldingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHolding<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHolding(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RightTapped<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightTapped(&*(&handler as *const <Input::RightTappedEventHandler as ::windows::core::Abi>::Abi as *const <Input::RightTappedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRightTapped<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRightTapped(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarting<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStarting(&*(&handler as *const <Input::ManipulationStartingEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationStartingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarting<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationInertiaStarting<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationInertiaStarting(&*(&handler as *const <Input::ManipulationInertiaStartingEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationInertiaStartingEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationInertiaStarting<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationInertiaStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationStarted<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStarted(&*(&handler as *const <Input::ManipulationStartedEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationStartedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationStarted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationDelta<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationDelta(&*(&handler as *const <Input::ManipulationDeltaEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationDeltaEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationDelta<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationDelta(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ManipulationCompleted<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationCompleted(&*(&handler as *const <Input::ManipulationCompletedEventHandler as ::windows::core::Abi>::Abi as *const <Input::ManipulationCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveManipulationCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Measure<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablesize: super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Measure(&*(&availablesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Arrange<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalrect: super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Arrange(&*(&finalrect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CapturePointer<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CapturePointer(&*(&value as *const <Input::Pointer as ::windows::core::Abi>::Abi as *const <Input::Pointer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleasePointerCapture<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleasePointerCapture(&*(&value as *const <Input::Pointer as ::windows::core::Abi>::Abi as *const <Input::Pointer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ReleasePointerCaptures<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleasePointerCaptures().into()
        }
        unsafe extern "system" fn AddHandler<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, routedevent: ::windows::core::RawPtr, handler: *mut ::core::ffi::c_void, handledeventstoo: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddHandler(&*(&routedevent as *const <RoutedEvent as ::windows::core::Abi>::Abi as *const <RoutedEvent as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), handledeventstoo).into()
        }
        unsafe extern "system" fn RemoveHandler<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, routedevent: ::windows::core::RawPtr, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHandler(&*(&routedevent as *const <RoutedEvent as ::windows::core::Abi>::Abi as *const <RoutedEvent as ::windows::core::DefaultType>::DefaultType), &*(&handler as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformToVisual<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformToVisual(&*(&visual as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidateMeasure<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateMeasure().into()
        }
        unsafe extern "system" fn InvalidateArrange<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidateArrange().into()
        }
        unsafe extern "system" fn UpdateLayout<Impl: IUIElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateLayout().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement, BASE_OFFSET>(),
            DesiredSize: DesiredSize::<Impl, IMPL_OFFSET>,
            AllowDrop: AllowDrop::<Impl, IMPL_OFFSET>,
            SetAllowDrop: SetAllowDrop::<Impl, IMPL_OFFSET>,
            Opacity: Opacity::<Impl, IMPL_OFFSET>,
            SetOpacity: SetOpacity::<Impl, IMPL_OFFSET>,
            Clip: Clip::<Impl, IMPL_OFFSET>,
            SetClip: SetClip::<Impl, IMPL_OFFSET>,
            RenderTransform: RenderTransform::<Impl, IMPL_OFFSET>,
            SetRenderTransform: SetRenderTransform::<Impl, IMPL_OFFSET>,
            Projection: Projection::<Impl, IMPL_OFFSET>,
            SetProjection: SetProjection::<Impl, IMPL_OFFSET>,
            RenderTransformOrigin: RenderTransformOrigin::<Impl, IMPL_OFFSET>,
            SetRenderTransformOrigin: SetRenderTransformOrigin::<Impl, IMPL_OFFSET>,
            IsHitTestVisible: IsHitTestVisible::<Impl, IMPL_OFFSET>,
            SetIsHitTestVisible: SetIsHitTestVisible::<Impl, IMPL_OFFSET>,
            Visibility: Visibility::<Impl, IMPL_OFFSET>,
            SetVisibility: SetVisibility::<Impl, IMPL_OFFSET>,
            RenderSize: RenderSize::<Impl, IMPL_OFFSET>,
            UseLayoutRounding: UseLayoutRounding::<Impl, IMPL_OFFSET>,
            SetUseLayoutRounding: SetUseLayoutRounding::<Impl, IMPL_OFFSET>,
            Transitions: Transitions::<Impl, IMPL_OFFSET>,
            SetTransitions: SetTransitions::<Impl, IMPL_OFFSET>,
            CacheMode: CacheMode::<Impl, IMPL_OFFSET>,
            SetCacheMode: SetCacheMode::<Impl, IMPL_OFFSET>,
            IsTapEnabled: IsTapEnabled::<Impl, IMPL_OFFSET>,
            SetIsTapEnabled: SetIsTapEnabled::<Impl, IMPL_OFFSET>,
            IsDoubleTapEnabled: IsDoubleTapEnabled::<Impl, IMPL_OFFSET>,
            SetIsDoubleTapEnabled: SetIsDoubleTapEnabled::<Impl, IMPL_OFFSET>,
            IsRightTapEnabled: IsRightTapEnabled::<Impl, IMPL_OFFSET>,
            SetIsRightTapEnabled: SetIsRightTapEnabled::<Impl, IMPL_OFFSET>,
            IsHoldingEnabled: IsHoldingEnabled::<Impl, IMPL_OFFSET>,
            SetIsHoldingEnabled: SetIsHoldingEnabled::<Impl, IMPL_OFFSET>,
            ManipulationMode: ManipulationMode::<Impl, IMPL_OFFSET>,
            SetManipulationMode: SetManipulationMode::<Impl, IMPL_OFFSET>,
            PointerCaptures: PointerCaptures::<Impl, IMPL_OFFSET>,
            KeyUp: KeyUp::<Impl, IMPL_OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Impl, IMPL_OFFSET>,
            KeyDown: KeyDown::<Impl, IMPL_OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Impl, IMPL_OFFSET>,
            GotFocus: GotFocus::<Impl, IMPL_OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Impl, IMPL_OFFSET>,
            LostFocus: LostFocus::<Impl, IMPL_OFFSET>,
            RemoveLostFocus: RemoveLostFocus::<Impl, IMPL_OFFSET>,
            DragEnter: DragEnter::<Impl, IMPL_OFFSET>,
            RemoveDragEnter: RemoveDragEnter::<Impl, IMPL_OFFSET>,
            DragLeave: DragLeave::<Impl, IMPL_OFFSET>,
            RemoveDragLeave: RemoveDragLeave::<Impl, IMPL_OFFSET>,
            DragOver: DragOver::<Impl, IMPL_OFFSET>,
            RemoveDragOver: RemoveDragOver::<Impl, IMPL_OFFSET>,
            Drop: Drop::<Impl, IMPL_OFFSET>,
            RemoveDrop: RemoveDrop::<Impl, IMPL_OFFSET>,
            PointerPressed: PointerPressed::<Impl, IMPL_OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Impl, IMPL_OFFSET>,
            PointerMoved: PointerMoved::<Impl, IMPL_OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Impl, IMPL_OFFSET>,
            PointerReleased: PointerReleased::<Impl, IMPL_OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Impl, IMPL_OFFSET>,
            PointerEntered: PointerEntered::<Impl, IMPL_OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Impl, IMPL_OFFSET>,
            PointerExited: PointerExited::<Impl, IMPL_OFFSET>,
            RemovePointerExited: RemovePointerExited::<Impl, IMPL_OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Impl, IMPL_OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Impl, IMPL_OFFSET>,
            PointerCanceled: PointerCanceled::<Impl, IMPL_OFFSET>,
            RemovePointerCanceled: RemovePointerCanceled::<Impl, IMPL_OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Impl, IMPL_OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Impl, IMPL_OFFSET>,
            Tapped: Tapped::<Impl, IMPL_OFFSET>,
            RemoveTapped: RemoveTapped::<Impl, IMPL_OFFSET>,
            DoubleTapped: DoubleTapped::<Impl, IMPL_OFFSET>,
            RemoveDoubleTapped: RemoveDoubleTapped::<Impl, IMPL_OFFSET>,
            Holding: Holding::<Impl, IMPL_OFFSET>,
            RemoveHolding: RemoveHolding::<Impl, IMPL_OFFSET>,
            RightTapped: RightTapped::<Impl, IMPL_OFFSET>,
            RemoveRightTapped: RemoveRightTapped::<Impl, IMPL_OFFSET>,
            ManipulationStarting: ManipulationStarting::<Impl, IMPL_OFFSET>,
            RemoveManipulationStarting: RemoveManipulationStarting::<Impl, IMPL_OFFSET>,
            ManipulationInertiaStarting: ManipulationInertiaStarting::<Impl, IMPL_OFFSET>,
            RemoveManipulationInertiaStarting: RemoveManipulationInertiaStarting::<Impl, IMPL_OFFSET>,
            ManipulationStarted: ManipulationStarted::<Impl, IMPL_OFFSET>,
            RemoveManipulationStarted: RemoveManipulationStarted::<Impl, IMPL_OFFSET>,
            ManipulationDelta: ManipulationDelta::<Impl, IMPL_OFFSET>,
            RemoveManipulationDelta: RemoveManipulationDelta::<Impl, IMPL_OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Impl, IMPL_OFFSET>,
            RemoveManipulationCompleted: RemoveManipulationCompleted::<Impl, IMPL_OFFSET>,
            Measure: Measure::<Impl, IMPL_OFFSET>,
            Arrange: Arrange::<Impl, IMPL_OFFSET>,
            CapturePointer: CapturePointer::<Impl, IMPL_OFFSET>,
            ReleasePointerCapture: ReleasePointerCapture::<Impl, IMPL_OFFSET>,
            ReleasePointerCaptures: ReleasePointerCaptures::<Impl, IMPL_OFFSET>,
            AddHandler: AddHandler::<Impl, IMPL_OFFSET>,
            RemoveHandler: RemoveHandler::<Impl, IMPL_OFFSET>,
            TransformToVisual: TransformToVisual::<Impl, IMPL_OFFSET>,
            InvalidateMeasure: InvalidateMeasure::<Impl, IMPL_OFFSET>,
            InvalidateArrange: InvalidateArrange::<Impl, IMPL_OFFSET>,
            UpdateLayout: UpdateLayout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IUIElement10_Impl: Sized {
    fn ActualOffset(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn ActualSize(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector2>;
    fn XamlRoot(&mut self) -> ::windows::core::Result<XamlRoot>;
    fn SetXamlRoot(&mut self, value: &::core::option::Option<XamlRoot>) -> ::windows::core::Result<()>;
    fn UIContext(&mut self) -> ::windows::core::Result<super::UIContext>;
    fn Shadow(&mut self) -> ::windows::core::Result<Media::Shadow>;
    fn SetShadow(&mut self, value: &::core::option::Option<Media::Shadow>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement10 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement10";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IUIElement10_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement10_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement10_Vtbl {
        unsafe extern "system" fn ActualOffset<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualSize<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XamlRoot<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XamlRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXamlRoot<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXamlRoot(&*(&value as *const <XamlRoot as ::windows::core::Abi>::Abi as *const <XamlRoot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UIContext<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shadow<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shadow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShadow<Impl: IUIElement10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShadow(&*(&value as *const <Media::Shadow as ::windows::core::Abi>::Abi as *const <Media::Shadow as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement10, BASE_OFFSET>(),
            ActualOffset: ActualOffset::<Impl, IMPL_OFFSET>,
            ActualSize: ActualSize::<Impl, IMPL_OFFSET>,
            XamlRoot: XamlRoot::<Impl, IMPL_OFFSET>,
            SetXamlRoot: SetXamlRoot::<Impl, IMPL_OFFSET>,
            UIContext: UIContext::<Impl, IMPL_OFFSET>,
            Shadow: Shadow::<Impl, IMPL_OFFSET>,
            SetShadow: SetShadow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement10 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IUIElement2_Impl: Sized {
    fn CompositeMode(&mut self) -> ::windows::core::Result<Media::ElementCompositeMode>;
    fn SetCompositeMode(&mut self, value: Media::ElementCompositeMode) -> ::windows::core::Result<()>;
    fn CancelDirectManipulations(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement2 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement2";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IUIElement2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement2_Vtbl {
        unsafe extern "system" fn CompositeMode<Impl: IUIElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Media::ElementCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositeMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompositeMode<Impl: IUIElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Media::ElementCompositeMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompositeMode(value).into()
        }
        unsafe extern "system" fn CancelDirectManipulations<Impl: IUIElement2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelDirectManipulations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement2, BASE_OFFSET>(),
            CompositeMode: CompositeMode::<Impl, IMPL_OFFSET>,
            SetCompositeMode: SetCompositeMode::<Impl, IMPL_OFFSET>,
            CancelDirectManipulations: CancelDirectManipulations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "UI_Input", feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
pub trait IUIElement3_Impl: Sized {
    fn Transform3D(&mut self) -> ::windows::core::Result<Media::Media3D::Transform3D>;
    fn SetTransform3D(&mut self, value: &::core::option::Option<Media::Media3D::Transform3D>) -> ::windows::core::Result<()>;
    fn CanDrag(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanDrag(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DragStarting(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragStarting(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DropCompleted(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDropCompleted(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartDragAsync(&mut self, pointerpoint: &::core::option::Option<super::Input::PointerPoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackageOperation>>;
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "UI_Input", feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement3 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement3";
}
#[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "UI_Input", feature = "UI_Xaml_Media_Media3D", feature = "implement_exclusive"))]
impl IUIElement3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement3_Vtbl {
        unsafe extern "system" fn Transform3D<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform3D() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform3D<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform3D(&*(&value as *const <Media::Media3D::Transform3D as ::windows::core::Abi>::Abi as *const <Media::Media3D::Transform3D as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanDrag<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDrag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanDrag<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanDrag(value).into()
        }
        unsafe extern "system" fn DragStarting<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragStarting(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragStarting<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragStarting(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DropCompleted<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropCompleted(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDropCompleted<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDropCompleted(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDragAsync<Impl: IUIElement3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDragAsync(&*(&pointerpoint as *const <super::Input::PointerPoint as ::windows::core::Abi>::Abi as *const <super::Input::PointerPoint as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement3, BASE_OFFSET>(),
            Transform3D: Transform3D::<Impl, IMPL_OFFSET>,
            SetTransform3D: SetTransform3D::<Impl, IMPL_OFFSET>,
            CanDrag: CanDrag::<Impl, IMPL_OFFSET>,
            SetCanDrag: SetCanDrag::<Impl, IMPL_OFFSET>,
            DragStarting: DragStarting::<Impl, IMPL_OFFSET>,
            RemoveDragStarting: RemoveDragStarting::<Impl, IMPL_OFFSET>,
            DropCompleted: DropCompleted::<Impl, IMPL_OFFSET>,
            RemoveDropCompleted: RemoveDropCompleted::<Impl, IMPL_OFFSET>,
            StartDragAsync: StartDragAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IUIElement4_Impl: Sized {
    fn ContextFlyout(&mut self) -> ::windows::core::Result<Controls::Primitives::FlyoutBase>;
    fn SetContextFlyout(&mut self, value: &::core::option::Option<Controls::Primitives::FlyoutBase>) -> ::windows::core::Result<()>;
    fn ExitDisplayModeOnAccessKeyInvoked(&mut self) -> ::windows::core::Result<bool>;
    fn SetExitDisplayModeOnAccessKeyInvoked(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsAccessKeyScope(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsAccessKeyScope(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AccessKeyScopeOwner(&mut self) -> ::windows::core::Result<DependencyObject>;
    fn SetAccessKeyScopeOwner(&mut self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn AccessKey(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContextRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ContextCanceled(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContextCanceled(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyDisplayDismissed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyDisplayDismissed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AccessKeyInvoked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccessKeyInvoked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement4 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement4";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Controls_Primitives", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IUIElement4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement4_Vtbl {
        unsafe extern "system" fn ContextFlyout<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextFlyout() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContextFlyout<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContextFlyout(&*(&value as *const <Controls::Primitives::FlyoutBase as ::windows::core::Abi>::Abi as *const <Controls::Primitives::FlyoutBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvoked<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitDisplayModeOnAccessKeyInvoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExitDisplayModeOnAccessKeyInvoked<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExitDisplayModeOnAccessKeyInvoked(value).into()
        }
        unsafe extern "system" fn IsAccessKeyScope<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAccessKeyScope() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsAccessKeyScope<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsAccessKeyScope(value).into()
        }
        unsafe extern "system" fn AccessKeyScopeOwner<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyScopeOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKeyScopeOwner<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKeyScopeOwner(&*(&value as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKey<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKey<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContextRequested<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContextRequested<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContextRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContextCanceled<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextCanceled(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, RoutedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContextCanceled<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContextCanceled(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyDisplayRequested<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyDisplayRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayRequested<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyDisplayRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyDisplayDismissed<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyDisplayDismissed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyDisplayDismissed<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyDisplayDismissed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AccessKeyInvoked<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyInvoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccessKeyInvoked<Impl: IUIElement4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessKeyInvoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement4, BASE_OFFSET>(),
            ContextFlyout: ContextFlyout::<Impl, IMPL_OFFSET>,
            SetContextFlyout: SetContextFlyout::<Impl, IMPL_OFFSET>,
            ExitDisplayModeOnAccessKeyInvoked: ExitDisplayModeOnAccessKeyInvoked::<Impl, IMPL_OFFSET>,
            SetExitDisplayModeOnAccessKeyInvoked: SetExitDisplayModeOnAccessKeyInvoked::<Impl, IMPL_OFFSET>,
            IsAccessKeyScope: IsAccessKeyScope::<Impl, IMPL_OFFSET>,
            SetIsAccessKeyScope: SetIsAccessKeyScope::<Impl, IMPL_OFFSET>,
            AccessKeyScopeOwner: AccessKeyScopeOwner::<Impl, IMPL_OFFSET>,
            SetAccessKeyScopeOwner: SetAccessKeyScopeOwner::<Impl, IMPL_OFFSET>,
            AccessKey: AccessKey::<Impl, IMPL_OFFSET>,
            SetAccessKey: SetAccessKey::<Impl, IMPL_OFFSET>,
            ContextRequested: ContextRequested::<Impl, IMPL_OFFSET>,
            RemoveContextRequested: RemoveContextRequested::<Impl, IMPL_OFFSET>,
            ContextCanceled: ContextCanceled::<Impl, IMPL_OFFSET>,
            RemoveContextCanceled: RemoveContextCanceled::<Impl, IMPL_OFFSET>,
            AccessKeyDisplayRequested: AccessKeyDisplayRequested::<Impl, IMPL_OFFSET>,
            RemoveAccessKeyDisplayRequested: RemoveAccessKeyDisplayRequested::<Impl, IMPL_OFFSET>,
            AccessKeyDisplayDismissed: AccessKeyDisplayDismissed::<Impl, IMPL_OFFSET>,
            RemoveAccessKeyDisplayDismissed: RemoveAccessKeyDisplayDismissed::<Impl, IMPL_OFFSET>,
            AccessKeyInvoked: AccessKeyInvoked::<Impl, IMPL_OFFSET>,
            RemoveAccessKeyInvoked: RemoveAccessKeyInvoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IUIElement5_Impl: Sized {
    fn Lights(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Media::XamlLight>>;
    fn KeyTipPlacementMode(&mut self) -> ::windows::core::Result<Input::KeyTipPlacementMode>;
    fn SetKeyTipPlacementMode(&mut self, value: Input::KeyTipPlacementMode) -> ::windows::core::Result<()>;
    fn KeyTipHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetKeyTipHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn KeyTipVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetKeyTipVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn XYFocusKeyboardNavigation(&mut self) -> ::windows::core::Result<Input::XYFocusKeyboardNavigationMode>;
    fn SetXYFocusKeyboardNavigation(&mut self, value: Input::XYFocusKeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn XYFocusUpNavigationStrategy(&mut self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusUpNavigationStrategy(&mut self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusDownNavigationStrategy(&mut self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusDownNavigationStrategy(&mut self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusLeftNavigationStrategy(&mut self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusLeftNavigationStrategy(&mut self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn XYFocusRightNavigationStrategy(&mut self) -> ::windows::core::Result<Input::XYFocusNavigationStrategy>;
    fn SetXYFocusRightNavigationStrategy(&mut self, value: Input::XYFocusNavigationStrategy) -> ::windows::core::Result<()>;
    fn HighContrastAdjustment(&mut self) -> ::windows::core::Result<ElementHighContrastAdjustment>;
    fn SetHighContrastAdjustment(&mut self, value: ElementHighContrastAdjustment) -> ::windows::core::Result<()>;
    fn TabFocusNavigation(&mut self) -> ::windows::core::Result<Input::KeyboardNavigationMode>;
    fn SetTabFocusNavigation(&mut self, value: Input::KeyboardNavigationMode) -> ::windows::core::Result<()>;
    fn GettingFocus(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveGettingFocus(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LosingFocus(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLosingFocus(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NoFocusCandidateFound(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveNoFocusCandidateFound(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartBringIntoView(&mut self) -> ::windows::core::Result<()>;
    fn StartBringIntoViewWithOptions(&mut self, options: &::core::option::Option<BringIntoViewOptions>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement5 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement5";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IUIElement5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement5_Vtbl {
        unsafe extern "system" fn Lights<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Lights() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipPlacementMode<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::KeyTipPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipPlacementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipPlacementMode<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::KeyTipPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipPlacementMode(value).into()
        }
        unsafe extern "system" fn KeyTipHorizontalOffset<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipHorizontalOffset<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipHorizontalOffset(value).into()
        }
        unsafe extern "system" fn KeyTipVerticalOffset<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipVerticalOffset<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipVerticalOffset(value).into()
        }
        unsafe extern "system" fn XYFocusKeyboardNavigation<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusKeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusKeyboardNavigation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusKeyboardNavigation<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusKeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusKeyboardNavigation(value).into()
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusUpNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusUpNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusDownNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusDownNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusLeftNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusLeftNavigationStrategy(value).into()
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusRightNavigationStrategy<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::XYFocusNavigationStrategy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusRightNavigationStrategy(value).into()
        }
        unsafe extern "system" fn HighContrastAdjustment<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ElementHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHighContrastAdjustment<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ElementHighContrastAdjustment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHighContrastAdjustment(value).into()
        }
        unsafe extern "system" fn TabFocusNavigation<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::KeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabFocusNavigation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTabFocusNavigation<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::KeyboardNavigationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTabFocusNavigation(value).into()
        }
        unsafe extern "system" fn GettingFocus<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GettingFocus(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGettingFocus<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGettingFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LosingFocus<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LosingFocus(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLosingFocus<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLosingFocus(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NoFocusCandidateFound<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoFocusCandidateFound(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveNoFocusCandidateFound<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveNoFocusCandidateFound(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartBringIntoView<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartBringIntoView().into()
        }
        unsafe extern "system" fn StartBringIntoViewWithOptions<Impl: IUIElement5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartBringIntoViewWithOptions(&*(&options as *const <BringIntoViewOptions as ::windows::core::Abi>::Abi as *const <BringIntoViewOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement5, BASE_OFFSET>(),
            Lights: Lights::<Impl, IMPL_OFFSET>,
            KeyTipPlacementMode: KeyTipPlacementMode::<Impl, IMPL_OFFSET>,
            SetKeyTipPlacementMode: SetKeyTipPlacementMode::<Impl, IMPL_OFFSET>,
            KeyTipHorizontalOffset: KeyTipHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetKeyTipHorizontalOffset: SetKeyTipHorizontalOffset::<Impl, IMPL_OFFSET>,
            KeyTipVerticalOffset: KeyTipVerticalOffset::<Impl, IMPL_OFFSET>,
            SetKeyTipVerticalOffset: SetKeyTipVerticalOffset::<Impl, IMPL_OFFSET>,
            XYFocusKeyboardNavigation: XYFocusKeyboardNavigation::<Impl, IMPL_OFFSET>,
            SetXYFocusKeyboardNavigation: SetXYFocusKeyboardNavigation::<Impl, IMPL_OFFSET>,
            XYFocusUpNavigationStrategy: XYFocusUpNavigationStrategy::<Impl, IMPL_OFFSET>,
            SetXYFocusUpNavigationStrategy: SetXYFocusUpNavigationStrategy::<Impl, IMPL_OFFSET>,
            XYFocusDownNavigationStrategy: XYFocusDownNavigationStrategy::<Impl, IMPL_OFFSET>,
            SetXYFocusDownNavigationStrategy: SetXYFocusDownNavigationStrategy::<Impl, IMPL_OFFSET>,
            XYFocusLeftNavigationStrategy: XYFocusLeftNavigationStrategy::<Impl, IMPL_OFFSET>,
            SetXYFocusLeftNavigationStrategy: SetXYFocusLeftNavigationStrategy::<Impl, IMPL_OFFSET>,
            XYFocusRightNavigationStrategy: XYFocusRightNavigationStrategy::<Impl, IMPL_OFFSET>,
            SetXYFocusRightNavigationStrategy: SetXYFocusRightNavigationStrategy::<Impl, IMPL_OFFSET>,
            HighContrastAdjustment: HighContrastAdjustment::<Impl, IMPL_OFFSET>,
            SetHighContrastAdjustment: SetHighContrastAdjustment::<Impl, IMPL_OFFSET>,
            TabFocusNavigation: TabFocusNavigation::<Impl, IMPL_OFFSET>,
            SetTabFocusNavigation: SetTabFocusNavigation::<Impl, IMPL_OFFSET>,
            GettingFocus: GettingFocus::<Impl, IMPL_OFFSET>,
            RemoveGettingFocus: RemoveGettingFocus::<Impl, IMPL_OFFSET>,
            LosingFocus: LosingFocus::<Impl, IMPL_OFFSET>,
            RemoveLosingFocus: RemoveLosingFocus::<Impl, IMPL_OFFSET>,
            NoFocusCandidateFound: NoFocusCandidateFound::<Impl, IMPL_OFFSET>,
            RemoveNoFocusCandidateFound: RemoveNoFocusCandidateFound::<Impl, IMPL_OFFSET>,
            StartBringIntoView: StartBringIntoView::<Impl, IMPL_OFFSET>,
            StartBringIntoViewWithOptions: StartBringIntoViewWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IUIElement7_Impl: Sized {
    fn KeyboardAccelerators(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Input::KeyboardAccelerator>>;
    fn CharacterReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProcessKeyboardAccelerators(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProcessKeyboardAccelerators(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewKeyDown(&mut self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewKeyDown(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PreviewKeyUp(&mut self, handler: &::core::option::Option<Input::KeyEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewKeyUp(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryInvokeKeyboardAccelerator(&mut self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement7";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IUIElement7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement7_Vtbl {
        unsafe extern "system" fn KeyboardAccelerators<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAccelerators() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterReceived<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCharacterReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProcessKeyboardAccelerators<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessKeyboardAccelerators(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProcessKeyboardAccelerators<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProcessKeyboardAccelerators(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewKeyDown<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewKeyDown(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewKeyDown<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviewKeyDown(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PreviewKeyUp<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewKeyUp(&*(&handler as *const <Input::KeyEventHandler as ::windows::core::Abi>::Abi as *const <Input::KeyEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemovePreviewKeyUp<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePreviewKeyUp(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TryInvokeKeyboardAccelerator<Impl: IUIElement7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryInvokeKeyboardAccelerator(&*(&args as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement7, BASE_OFFSET>(),
            KeyboardAccelerators: KeyboardAccelerators::<Impl, IMPL_OFFSET>,
            CharacterReceived: CharacterReceived::<Impl, IMPL_OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Impl, IMPL_OFFSET>,
            ProcessKeyboardAccelerators: ProcessKeyboardAccelerators::<Impl, IMPL_OFFSET>,
            RemoveProcessKeyboardAccelerators: RemoveProcessKeyboardAccelerators::<Impl, IMPL_OFFSET>,
            PreviewKeyDown: PreviewKeyDown::<Impl, IMPL_OFFSET>,
            RemovePreviewKeyDown: RemovePreviewKeyDown::<Impl, IMPL_OFFSET>,
            PreviewKeyUp: PreviewKeyUp::<Impl, IMPL_OFFSET>,
            RemovePreviewKeyUp: RemovePreviewKeyUp::<Impl, IMPL_OFFSET>,
            TryInvokeKeyboardAccelerator: TryInvokeKeyboardAccelerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IUIElement8_Impl: Sized {
    fn KeyTipTarget(&mut self) -> ::windows::core::Result<DependencyObject>;
    fn SetKeyTipTarget(&mut self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyboardAcceleratorPlacementTarget(&mut self) -> ::windows::core::Result<DependencyObject>;
    fn SetKeyboardAcceleratorPlacementTarget(&mut self, value: &::core::option::Option<DependencyObject>) -> ::windows::core::Result<()>;
    fn KeyboardAcceleratorPlacementMode(&mut self) -> ::windows::core::Result<Input::KeyboardAcceleratorPlacementMode>;
    fn SetKeyboardAcceleratorPlacementMode(&mut self, value: Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::Result<()>;
    fn BringIntoViewRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBringIntoViewRequested(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement8";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IUIElement8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement8_Vtbl {
        unsafe extern "system" fn KeyTipTarget<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyTipTarget<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyTipTarget(&*(&value as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementTarget<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyboardAcceleratorPlacementTarget<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyboardAcceleratorPlacementTarget(&*(&value as *const <DependencyObject as ::windows::core::Abi>::Abi as *const <DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementMode<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyboardAcceleratorPlacementMode<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Input::KeyboardAcceleratorPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeyboardAcceleratorPlacementMode(value).into()
        }
        unsafe extern "system" fn BringIntoViewRequested<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BringIntoViewRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBringIntoViewRequested<Impl: IUIElement8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBringIntoViewRequested(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement8, BASE_OFFSET>(),
            KeyTipTarget: KeyTipTarget::<Impl, IMPL_OFFSET>,
            SetKeyTipTarget: SetKeyTipTarget::<Impl, IMPL_OFFSET>,
            KeyboardAcceleratorPlacementTarget: KeyboardAcceleratorPlacementTarget::<Impl, IMPL_OFFSET>,
            SetKeyboardAcceleratorPlacementTarget: SetKeyboardAcceleratorPlacementTarget::<Impl, IMPL_OFFSET>,
            KeyboardAcceleratorPlacementMode: KeyboardAcceleratorPlacementMode::<Impl, IMPL_OFFSET>,
            SetKeyboardAcceleratorPlacementMode: SetKeyboardAcceleratorPlacementMode::<Impl, IMPL_OFFSET>,
            BringIntoViewRequested: BringIntoViewRequested::<Impl, IMPL_OFFSET>,
            RemoveBringIntoViewRequested: RemoveBringIntoViewRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Numerics", feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IUIElement9_Impl: Sized {
    fn CanBeScrollAnchor(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanBeScrollAnchor(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OpacityTransition(&mut self) -> ::windows::core::Result<ScalarTransition>;
    fn SetOpacityTransition(&mut self, value: &::core::option::Option<ScalarTransition>) -> ::windows::core::Result<()>;
    fn Translation(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetTranslation(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn TranslationTransition(&mut self) -> ::windows::core::Result<Vector3Transition>;
    fn SetTranslationTransition(&mut self, value: &::core::option::Option<Vector3Transition>) -> ::windows::core::Result<()>;
    fn Rotation(&mut self) -> ::windows::core::Result<f32>;
    fn SetRotation(&mut self, value: f32) -> ::windows::core::Result<()>;
    fn RotationTransition(&mut self) -> ::windows::core::Result<ScalarTransition>;
    fn SetRotationTransition(&mut self, value: &::core::option::Option<ScalarTransition>) -> ::windows::core::Result<()>;
    fn Scale(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetScale(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn ScaleTransition(&mut self) -> ::windows::core::Result<Vector3Transition>;
    fn SetScaleTransition(&mut self, value: &::core::option::Option<Vector3Transition>) -> ::windows::core::Result<()>;
    fn TransformMatrix(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4>;
    fn SetTransformMatrix(&mut self, value: &super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()>;
    fn CenterPoint(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetCenterPoint(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn RotationAxis(&mut self) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn SetRotationAxis(&mut self, value: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn StartAnimation(&mut self, animation: &::core::option::Option<super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
    fn StopAnimation(&mut self, animation: &::core::option::Option<super::Composition::ICompositionAnimationBase>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Numerics", feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElement9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElement9";
}
#[cfg(all(feature = "Foundation_Numerics", feature = "UI_Composition", feature = "implement_exclusive"))]
impl IUIElement9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElement9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElement9_Vtbl {
        unsafe extern "system" fn CanBeScrollAnchor<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBeScrollAnchor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanBeScrollAnchor<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanBeScrollAnchor(value).into()
        }
        unsafe extern "system" fn OpacityTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpacityTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOpacityTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOpacityTransition(&*(&value as *const <ScalarTransition as ::windows::core::Abi>::Abi as *const <ScalarTransition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Translation<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Translation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslation<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslation(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslationTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslationTransition(&*(&value as *const <Vector3Transition as ::windows::core::Abi>::Abi as *const <Vector3Transition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Rotation<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Rotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(value).into()
        }
        unsafe extern "system" fn RotationTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationTransition(&*(&value as *const <ScalarTransition as ::windows::core::Abi>::Abi as *const <ScalarTransition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Scale<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScale<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScale(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ScaleTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScaleTransition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScaleTransition<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScaleTransition(&*(&value as *const <Vector3Transition as ::windows::core::Abi>::Abi as *const <Vector3Transition as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransformMatrix<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransformMatrix() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransformMatrix<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransformMatrix(&*(&value as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Matrix4x4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CenterPoint<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenterPoint<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenterPoint(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationAxis<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationAxis() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationAxis<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Vector3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationAxis(&*(&value as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartAnimation<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAnimation(&*(&animation as *const <super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StopAnimation<Impl: IUIElement9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopAnimation(&*(&animation as *const <super::Composition::ICompositionAnimationBase as ::windows::core::Abi>::Abi as *const <super::Composition::ICompositionAnimationBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElement9, BASE_OFFSET>(),
            CanBeScrollAnchor: CanBeScrollAnchor::<Impl, IMPL_OFFSET>,
            SetCanBeScrollAnchor: SetCanBeScrollAnchor::<Impl, IMPL_OFFSET>,
            OpacityTransition: OpacityTransition::<Impl, IMPL_OFFSET>,
            SetOpacityTransition: SetOpacityTransition::<Impl, IMPL_OFFSET>,
            Translation: Translation::<Impl, IMPL_OFFSET>,
            SetTranslation: SetTranslation::<Impl, IMPL_OFFSET>,
            TranslationTransition: TranslationTransition::<Impl, IMPL_OFFSET>,
            SetTranslationTransition: SetTranslationTransition::<Impl, IMPL_OFFSET>,
            Rotation: Rotation::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            RotationTransition: RotationTransition::<Impl, IMPL_OFFSET>,
            SetRotationTransition: SetRotationTransition::<Impl, IMPL_OFFSET>,
            Scale: Scale::<Impl, IMPL_OFFSET>,
            SetScale: SetScale::<Impl, IMPL_OFFSET>,
            ScaleTransition: ScaleTransition::<Impl, IMPL_OFFSET>,
            SetScaleTransition: SetScaleTransition::<Impl, IMPL_OFFSET>,
            TransformMatrix: TransformMatrix::<Impl, IMPL_OFFSET>,
            SetTransformMatrix: SetTransformMatrix::<Impl, IMPL_OFFSET>,
            CenterPoint: CenterPoint::<Impl, IMPL_OFFSET>,
            SetCenterPoint: SetCenterPoint::<Impl, IMPL_OFFSET>,
            RotationAxis: RotationAxis::<Impl, IMPL_OFFSET>,
            SetRotationAxis: SetRotationAxis::<Impl, IMPL_OFFSET>,
            StartAnimation: StartAnimation::<Impl, IMPL_OFFSET>,
            StopAnimation: StopAnimation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElement9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
pub trait IUIElementOverrides_Impl: Sized {
    fn OnCreateAutomationPeer(&mut self) -> ::windows::core::Result<Automation::Peers::AutomationPeer>;
    fn OnDisconnectVisualChildren(&mut self) -> ::windows::core::Result<()>;
    fn FindSubElementsForTouchTargeting(&mut self, point: &super::super::Foundation::Point, boundingrect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Point>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers", feature = "implement_exclusive"))]
impl IUIElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides_Vtbl {
        unsafe extern "system" fn OnCreateAutomationPeer<Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCreateAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisconnectVisualChildren<Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnectVisualChildren().into()
        }
        unsafe extern "system" fn FindSubElementsForTouchTargeting<Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, boundingrect: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSubElementsForTouchTargeting(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&boundingrect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides, BASE_OFFSET>(),
            OnCreateAutomationPeer: OnCreateAutomationPeer::<Impl, IMPL_OFFSET>,
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Impl, IMPL_OFFSET>,
            FindSubElementsForTouchTargeting: FindSubElementsForTouchTargeting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IUIElementOverrides7_Impl: Sized {
    fn GetChildrenInTabFocusOrder(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<DependencyObject>>;
    fn OnProcessKeyboardAccelerators(&mut self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElementOverrides7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides7";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IUIElementOverrides7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides7_Vtbl {
        unsafe extern "system" fn GetChildrenInTabFocusOrder<Impl: IUIElementOverrides7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenInTabFocusOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Impl: IUIElementOverrides7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(&*(&args as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides7, BASE_OFFSET>(),
            GetChildrenInTabFocusOrder: GetChildrenInTabFocusOrder::<Impl, IMPL_OFFSET>,
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IUIElementOverrides8_Impl: Sized {
    fn OnKeyboardAcceleratorInvoked(&mut self, args: &::core::option::Option<Input::KeyboardAcceleratorInvokedEventArgs>) -> ::windows::core::Result<()>;
    fn OnBringIntoViewRequested(&mut self, e: &::core::option::Option<BringIntoViewRequestedEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElementOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides8";
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IUIElementOverrides8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides8_Vtbl {
        unsafe extern "system" fn OnKeyboardAcceleratorInvoked<Impl: IUIElementOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyboardAcceleratorInvoked(&*(&args as *const <Input::KeyboardAcceleratorInvokedEventArgs as ::windows::core::Abi>::Abi as *const <Input::KeyboardAcceleratorInvokedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnBringIntoViewRequested<Impl: IUIElementOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBringIntoViewRequested(&*(&e as *const <BringIntoViewRequestedEventArgs as ::windows::core::Abi>::Abi as *const <BringIntoViewRequestedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides8, BASE_OFFSET>(),
            OnKeyboardAcceleratorInvoked: OnKeyboardAcceleratorInvoked::<Impl, IMPL_OFFSET>,
            OnBringIntoViewRequested: OnBringIntoViewRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides8 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IUIElementOverrides9_Impl: Sized {
    fn PopulatePropertyInfoOverride(&mut self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElementOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides9";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IUIElementOverrides9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides9_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Impl: IUIElementOverrides9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfoOverride(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animationpropertyinfo as *const <super::Composition::AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <super::Composition::AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides9, BASE_OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics_Impl: Sized {
    fn KeyDownEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn KeyUpEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerEnteredEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerPressedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerMovedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerReleasedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerExitedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerCaptureLostEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerCanceledEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PointerWheelChangedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn TappedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn DoubleTappedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn HoldingEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn RightTappedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationStartingEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationInertiaStartingEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationStartedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationDeltaEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn ManipulationCompletedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn DragEnterEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn DragLeaveEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn DragOverEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn DropEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn AllowDropProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn OpacityProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn ClipProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn RenderTransformProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn ProjectionProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn RenderTransformOriginProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn IsHitTestVisibleProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn VisibilityProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn UseLayoutRoundingProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn TransitionsProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn CacheModeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn IsTapEnabledProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn IsDoubleTapEnabledProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn IsRightTapEnabledProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn IsHoldingEnabledProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn ManipulationModeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn PointerCapturesProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics_Vtbl {
        unsafe extern "system" fn KeyDownEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyDownEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyUpEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyUpEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerEnteredEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerEnteredEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerPressedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerPressedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerMovedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerMovedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerReleasedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerReleasedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerExitedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerExitedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCaptureLostEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCaptureLostEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCanceledEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCanceledEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerWheelChangedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerWheelChangedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TappedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TappedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoubleTappedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoubleTappedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldingEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldingEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RightTappedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RightTappedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationStartingEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStartingEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationInertiaStartingEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationInertiaStartingEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationStartedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationStartedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationDeltaEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationDeltaEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationCompletedEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationCompletedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragEnterEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragEnterEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragLeaveEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragLeaveEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOverEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOverEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropEvent<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowDropProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowDropProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpacityProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTransformProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTransformProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProjectionProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProjectionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenderTransformOriginProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenderTransformOriginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHitTestVisibleProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHitTestVisibleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VisibilityProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibilityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseLayoutRoundingProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseLayoutRoundingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransitionsProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransitionsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CacheModeProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CacheModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTapEnabledProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTapEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDoubleTapEnabledProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDoubleTapEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRightTapEnabledProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRightTapEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHoldingEnabledProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHoldingEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ManipulationModeProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ManipulationModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerCapturesProperty<Impl: IUIElementStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerCapturesProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics, BASE_OFFSET>(),
            KeyDownEvent: KeyDownEvent::<Impl, IMPL_OFFSET>,
            KeyUpEvent: KeyUpEvent::<Impl, IMPL_OFFSET>,
            PointerEnteredEvent: PointerEnteredEvent::<Impl, IMPL_OFFSET>,
            PointerPressedEvent: PointerPressedEvent::<Impl, IMPL_OFFSET>,
            PointerMovedEvent: PointerMovedEvent::<Impl, IMPL_OFFSET>,
            PointerReleasedEvent: PointerReleasedEvent::<Impl, IMPL_OFFSET>,
            PointerExitedEvent: PointerExitedEvent::<Impl, IMPL_OFFSET>,
            PointerCaptureLostEvent: PointerCaptureLostEvent::<Impl, IMPL_OFFSET>,
            PointerCanceledEvent: PointerCanceledEvent::<Impl, IMPL_OFFSET>,
            PointerWheelChangedEvent: PointerWheelChangedEvent::<Impl, IMPL_OFFSET>,
            TappedEvent: TappedEvent::<Impl, IMPL_OFFSET>,
            DoubleTappedEvent: DoubleTappedEvent::<Impl, IMPL_OFFSET>,
            HoldingEvent: HoldingEvent::<Impl, IMPL_OFFSET>,
            RightTappedEvent: RightTappedEvent::<Impl, IMPL_OFFSET>,
            ManipulationStartingEvent: ManipulationStartingEvent::<Impl, IMPL_OFFSET>,
            ManipulationInertiaStartingEvent: ManipulationInertiaStartingEvent::<Impl, IMPL_OFFSET>,
            ManipulationStartedEvent: ManipulationStartedEvent::<Impl, IMPL_OFFSET>,
            ManipulationDeltaEvent: ManipulationDeltaEvent::<Impl, IMPL_OFFSET>,
            ManipulationCompletedEvent: ManipulationCompletedEvent::<Impl, IMPL_OFFSET>,
            DragEnterEvent: DragEnterEvent::<Impl, IMPL_OFFSET>,
            DragLeaveEvent: DragLeaveEvent::<Impl, IMPL_OFFSET>,
            DragOverEvent: DragOverEvent::<Impl, IMPL_OFFSET>,
            DropEvent: DropEvent::<Impl, IMPL_OFFSET>,
            AllowDropProperty: AllowDropProperty::<Impl, IMPL_OFFSET>,
            OpacityProperty: OpacityProperty::<Impl, IMPL_OFFSET>,
            ClipProperty: ClipProperty::<Impl, IMPL_OFFSET>,
            RenderTransformProperty: RenderTransformProperty::<Impl, IMPL_OFFSET>,
            ProjectionProperty: ProjectionProperty::<Impl, IMPL_OFFSET>,
            RenderTransformOriginProperty: RenderTransformOriginProperty::<Impl, IMPL_OFFSET>,
            IsHitTestVisibleProperty: IsHitTestVisibleProperty::<Impl, IMPL_OFFSET>,
            VisibilityProperty: VisibilityProperty::<Impl, IMPL_OFFSET>,
            UseLayoutRoundingProperty: UseLayoutRoundingProperty::<Impl, IMPL_OFFSET>,
            TransitionsProperty: TransitionsProperty::<Impl, IMPL_OFFSET>,
            CacheModeProperty: CacheModeProperty::<Impl, IMPL_OFFSET>,
            IsTapEnabledProperty: IsTapEnabledProperty::<Impl, IMPL_OFFSET>,
            IsDoubleTapEnabledProperty: IsDoubleTapEnabledProperty::<Impl, IMPL_OFFSET>,
            IsRightTapEnabledProperty: IsRightTapEnabledProperty::<Impl, IMPL_OFFSET>,
            IsHoldingEnabledProperty: IsHoldingEnabledProperty::<Impl, IMPL_OFFSET>,
            ManipulationModeProperty: ManipulationModeProperty::<Impl, IMPL_OFFSET>,
            PointerCapturesProperty: PointerCapturesProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics10_Impl: Sized {
    fn ShadowProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics10 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics10";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics10_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics10_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics10_Vtbl {
        unsafe extern "system" fn ShadowProperty<Impl: IUIElementStatics10_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShadowProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics10, BASE_OFFSET>(),
            ShadowProperty: ShadowProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics10 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics2_Impl: Sized {
    fn CompositeModeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics2_Vtbl {
        unsafe extern "system" fn CompositeModeProperty<Impl: IUIElementStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompositeModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics2, BASE_OFFSET>(),
            CompositeModeProperty: CompositeModeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IUIElementStatics3_Impl: Sized {
    fn Transform3DProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn CanDragProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn TryStartDirectManipulation(&mut self, value: &::core::option::Option<Input::Pointer>) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUIElementStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics3";
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IUIElementStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics3_Vtbl {
        unsafe extern "system" fn Transform3DProperty<Impl: IUIElementStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transform3DProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanDragProperty<Impl: IUIElementStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanDragProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryStartDirectManipulation<Impl: IUIElementStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryStartDirectManipulation(&*(&value as *const <Input::Pointer as ::windows::core::Abi>::Abi as *const <Input::Pointer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics3, BASE_OFFSET>(),
            Transform3DProperty: Transform3DProperty::<Impl, IMPL_OFFSET>,
            CanDragProperty: CanDragProperty::<Impl, IMPL_OFFSET>,
            TryStartDirectManipulation: TryStartDirectManipulation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics4_Impl: Sized {
    fn ContextFlyoutProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn ExitDisplayModeOnAccessKeyInvokedProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn IsAccessKeyScopeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn AccessKeyScopeOwnerProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn AccessKeyProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics4_Vtbl {
        unsafe extern "system" fn ContextFlyoutProperty<Impl: IUIElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextFlyoutProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExitDisplayModeOnAccessKeyInvokedProperty<Impl: IUIElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExitDisplayModeOnAccessKeyInvokedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAccessKeyScopeProperty<Impl: IUIElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAccessKeyScopeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyScopeOwnerProperty<Impl: IUIElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyScopeOwnerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IUIElementStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics4, BASE_OFFSET>(),
            ContextFlyoutProperty: ContextFlyoutProperty::<Impl, IMPL_OFFSET>,
            ExitDisplayModeOnAccessKeyInvokedProperty: ExitDisplayModeOnAccessKeyInvokedProperty::<Impl, IMPL_OFFSET>,
            IsAccessKeyScopeProperty: IsAccessKeyScopeProperty::<Impl, IMPL_OFFSET>,
            AccessKeyScopeOwnerProperty: AccessKeyScopeOwnerProperty::<Impl, IMPL_OFFSET>,
            AccessKeyProperty: AccessKeyProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics5_Impl: Sized {
    fn LightsProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipPlacementModeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipHorizontalOffsetProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyTipVerticalOffsetProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusKeyboardNavigationProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusUpNavigationStrategyProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusDownNavigationStrategyProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusLeftNavigationStrategyProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn XYFocusRightNavigationStrategyProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn HighContrastAdjustmentProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn TabFocusNavigationProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics5_Vtbl {
        unsafe extern "system" fn LightsProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipPlacementModeProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipPlacementModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipHorizontalOffsetProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipHorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipVerticalOffsetProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipVerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusKeyboardNavigationProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusKeyboardNavigationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusUpNavigationStrategyProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusUpNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusDownNavigationStrategyProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusDownNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusLeftNavigationStrategyProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusLeftNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XYFocusRightNavigationStrategyProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusRightNavigationStrategyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HighContrastAdjustmentProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HighContrastAdjustmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TabFocusNavigationProperty<Impl: IUIElementStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TabFocusNavigationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics5, BASE_OFFSET>(),
            LightsProperty: LightsProperty::<Impl, IMPL_OFFSET>,
            KeyTipPlacementModeProperty: KeyTipPlacementModeProperty::<Impl, IMPL_OFFSET>,
            KeyTipHorizontalOffsetProperty: KeyTipHorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            KeyTipVerticalOffsetProperty: KeyTipVerticalOffsetProperty::<Impl, IMPL_OFFSET>,
            XYFocusKeyboardNavigationProperty: XYFocusKeyboardNavigationProperty::<Impl, IMPL_OFFSET>,
            XYFocusUpNavigationStrategyProperty: XYFocusUpNavigationStrategyProperty::<Impl, IMPL_OFFSET>,
            XYFocusDownNavigationStrategyProperty: XYFocusDownNavigationStrategyProperty::<Impl, IMPL_OFFSET>,
            XYFocusLeftNavigationStrategyProperty: XYFocusLeftNavigationStrategyProperty::<Impl, IMPL_OFFSET>,
            XYFocusRightNavigationStrategyProperty: XYFocusRightNavigationStrategyProperty::<Impl, IMPL_OFFSET>,
            HighContrastAdjustmentProperty: HighContrastAdjustmentProperty::<Impl, IMPL_OFFSET>,
            TabFocusNavigationProperty: TabFocusNavigationProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics6_Impl: Sized {
    fn GettingFocusEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn LosingFocusEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn NoFocusCandidateFoundEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics6_Vtbl {
        unsafe extern "system" fn GettingFocusEvent<Impl: IUIElementStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GettingFocusEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LosingFocusEvent<Impl: IUIElementStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LosingFocusEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoFocusCandidateFoundEvent<Impl: IUIElementStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoFocusCandidateFoundEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics6, BASE_OFFSET>(),
            GettingFocusEvent: GettingFocusEvent::<Impl, IMPL_OFFSET>,
            LosingFocusEvent: LosingFocusEvent::<Impl, IMPL_OFFSET>,
            NoFocusCandidateFoundEvent: NoFocusCandidateFoundEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics7_Impl: Sized {
    fn PreviewKeyDownEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn CharacterReceivedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn PreviewKeyUpEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics7_Vtbl {
        unsafe extern "system" fn PreviewKeyDownEvent<Impl: IUIElementStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewKeyDownEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CharacterReceivedEvent<Impl: IUIElementStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CharacterReceivedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviewKeyUpEvent<Impl: IUIElementStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PreviewKeyUpEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics7, BASE_OFFSET>(),
            PreviewKeyDownEvent: PreviewKeyDownEvent::<Impl, IMPL_OFFSET>,
            CharacterReceivedEvent: CharacterReceivedEvent::<Impl, IMPL_OFFSET>,
            PreviewKeyUpEvent: PreviewKeyUpEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics8_Impl: Sized {
    fn BringIntoViewRequestedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn ContextRequestedEvent(&mut self) -> ::windows::core::Result<RoutedEvent>;
    fn KeyTipTargetProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyboardAcceleratorPlacementTargetProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn KeyboardAcceleratorPlacementModeProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn RegisterAsScrollPort(&mut self, element: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics8";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics8_Vtbl {
        unsafe extern "system" fn BringIntoViewRequestedEvent<Impl: IUIElementStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BringIntoViewRequestedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextRequestedEvent<Impl: IUIElementStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextRequestedEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyTipTargetProperty<Impl: IUIElementStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyTipTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementTargetProperty<Impl: IUIElementStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorPlacementModeProperty<Impl: IUIElementStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorPlacementModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAsScrollPort<Impl: IUIElementStatics8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterAsScrollPort(&*(&element as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics8, BASE_OFFSET>(),
            BringIntoViewRequestedEvent: BringIntoViewRequestedEvent::<Impl, IMPL_OFFSET>,
            ContextRequestedEvent: ContextRequestedEvent::<Impl, IMPL_OFFSET>,
            KeyTipTargetProperty: KeyTipTargetProperty::<Impl, IMPL_OFFSET>,
            KeyboardAcceleratorPlacementTargetProperty: KeyboardAcceleratorPlacementTargetProperty::<Impl, IMPL_OFFSET>,
            KeyboardAcceleratorPlacementModeProperty: KeyboardAcceleratorPlacementModeProperty::<Impl, IMPL_OFFSET>,
            RegisterAsScrollPort: RegisterAsScrollPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementStatics9_Impl: Sized {
    fn CanBeScrollAnchorProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementStatics9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementStatics9";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementStatics9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementStatics9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementStatics9_Vtbl {
        unsafe extern "system" fn CanBeScrollAnchorProperty<Impl: IUIElementStatics9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBeScrollAnchorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementStatics9, BASE_OFFSET>(),
            CanBeScrollAnchorProperty: CanBeScrollAnchorProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementStatics9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementWeakCollection_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementWeakCollection {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementWeakCollection";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementWeakCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementWeakCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementWeakCollection_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementWeakCollection, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementWeakCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUIElementWeakCollectionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<UIElementWeakCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUIElementWeakCollectionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementWeakCollectionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUIElementWeakCollectionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementWeakCollectionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementWeakCollectionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IUIElementWeakCollectionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementWeakCollectionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementWeakCollectionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnhandledExceptionEventArgs_Impl: Sized {
    fn Exception(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Message(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUnhandledExceptionEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IUnhandledExceptionEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IUnhandledExceptionEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUnhandledExceptionEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUnhandledExceptionEventArgs_Vtbl {
        unsafe extern "system" fn Exception<Impl: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exception() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUnhandledExceptionEventArgs, BASE_OFFSET>(),
            Exception: Exception::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUnhandledExceptionEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IVector3Transition_Impl: Sized {
    fn Duration(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDuration(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Components(&mut self) -> ::windows::core::Result<Vector3TransitionComponents>;
    fn SetComponents(&mut self, value: Vector3TransitionComponents) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVector3Transition {
    const NAME: &'static str = "Windows.UI.Xaml.IVector3Transition";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IVector3Transition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3Transition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3Transition_Vtbl {
        unsafe extern "system" fn Duration<Impl: IVector3Transition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Duration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuration<Impl: IVector3Transition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuration(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Components<Impl: IVector3Transition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Vector3TransitionComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Impl: IVector3Transition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Vector3TransitionComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComponents(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector3Transition, BASE_OFFSET>(),
            Duration: Duration::<Impl, IMPL_OFFSET>,
            SetDuration: SetDuration::<Impl, IMPL_OFFSET>,
            Components: Components::<Impl, IMPL_OFFSET>,
            SetComponents: SetComponents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector3Transition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVector3TransitionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<Vector3Transition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVector3TransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IVector3TransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVector3TransitionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVector3TransitionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVector3TransitionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IVector3TransitionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVector3TransitionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVector3TransitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IVisualState_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Storyboard(&mut self) -> ::windows::core::Result<Media::Animation::Storyboard>;
    fn SetStoryboard(&mut self, value: &::core::option::Option<Media::Animation::Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualState {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualState";
}
#[cfg(all(feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IVisualState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualState_Vtbl {
        unsafe extern "system" fn Name<Impl: IVisualState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Storyboard<Impl: IVisualState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Storyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboard<Impl: IVisualState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Media::Animation::Storyboard as ::windows::core::Abi>::Abi as *const <Media::Animation::Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualState, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Storyboard: Storyboard::<Impl, IMPL_OFFSET>,
            SetStoryboard: SetStoryboard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualState as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVisualState2_Impl: Sized {
    fn Setters(&mut self) -> ::windows::core::Result<SetterBaseCollection>;
    fn StateTriggers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StateTriggerBase>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualState2 {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualState2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVisualState2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualState2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualState2_Vtbl {
        unsafe extern "system" fn Setters<Impl: IVisualState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Setters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StateTriggers<Impl: IVisualState2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StateTriggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualState2, BASE_OFFSET>(),
            Setters: Setters::<Impl, IMPL_OFFSET>,
            StateTriggers: StateTriggers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualState2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IVisualStateChangedEventArgs_Impl: Sized {
    fn OldState(&mut self) -> ::windows::core::Result<VisualState>;
    fn SetOldState(&mut self, value: &::core::option::Option<VisualState>) -> ::windows::core::Result<()>;
    fn NewState(&mut self) -> ::windows::core::Result<VisualState>;
    fn SetNewState(&mut self, value: &::core::option::Option<VisualState>) -> ::windows::core::Result<()>;
    fn Control(&mut self) -> ::windows::core::Result<Controls::Control>;
    fn SetControl(&mut self, value: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualStateChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateChangedEventArgs";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IVisualStateChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateChangedEventArgs_Vtbl {
        unsafe extern "system" fn OldState<Impl: IVisualStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldState<Impl: IVisualStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOldState(&*(&value as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NewState<Impl: IVisualStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewState<Impl: IVisualStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNewState(&*(&value as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Control<Impl: IVisualStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControl<Impl: IVisualStateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControl(&*(&value as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateChangedEventArgs, BASE_OFFSET>(),
            OldState: OldState::<Impl, IMPL_OFFSET>,
            SetOldState: SetOldState::<Impl, IMPL_OFFSET>,
            NewState: NewState::<Impl, IMPL_OFFSET>,
            SetNewState: SetNewState::<Impl, IMPL_OFFSET>,
            Control: Control::<Impl, IMPL_OFFSET>,
            SetControl: SetControl::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVisualStateGroup_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Transitions(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualTransition>>;
    fn States(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualState>>;
    fn CurrentState(&mut self) -> ::windows::core::Result<VisualState>;
    fn CurrentStateChanged(&mut self, handler: &::core::option::Option<VisualStateChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CurrentStateChanging(&mut self, handler: &::core::option::Option<VisualStateChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCurrentStateChanging(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualStateGroup {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateGroup";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVisualStateGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Transitions<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Transitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn States<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).States() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentState<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentStateChanged<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStateChanged(&*(&handler as *const <VisualStateChangedEventHandler as ::windows::core::Abi>::Abi as *const <VisualStateChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentStateChanged<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentStateChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CurrentStateChanging<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentStateChanging(&*(&handler as *const <VisualStateChangedEventHandler as ::windows::core::Abi>::Abi as *const <VisualStateChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCurrentStateChanging<Impl: IVisualStateGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCurrentStateChanging(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateGroup, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Transitions: Transitions::<Impl, IMPL_OFFSET>,
            States: States::<Impl, IMPL_OFFSET>,
            CurrentState: CurrentState::<Impl, IMPL_OFFSET>,
            CurrentStateChanged: CurrentStateChanged::<Impl, IMPL_OFFSET>,
            RemoveCurrentStateChanged: RemoveCurrentStateChanged::<Impl, IMPL_OFFSET>,
            CurrentStateChanging: CurrentStateChanging::<Impl, IMPL_OFFSET>,
            RemoveCurrentStateChanging: RemoveCurrentStateChanging::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManager_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManager {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManager";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateManager_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualStateManagerFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VisualStateManager>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualStateManagerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualStateManagerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateManagerFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IVisualStateManagerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManagerFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManagerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IVisualStateManagerOverrides_Impl: Sized {
    fn GoToStateCore(&mut self, control: &::core::option::Option<Controls::Control>, templateroot: &::core::option::Option<FrameworkElement>, statename: &::windows::core::HSTRING, group: &::core::option::Option<VisualStateGroup>, state: &::core::option::Option<VisualState>, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualStateManagerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerOverrides";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IVisualStateManagerOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateManagerOverrides_Vtbl {
        unsafe extern "system" fn GoToStateCore<Impl: IVisualStateManagerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: ::windows::core::RawPtr, templateroot: ::windows::core::RawPtr, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::windows::core::RawPtr, state: ::windows::core::RawPtr, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManagerOverrides, BASE_OFFSET>(),
            GoToStateCore: GoToStateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManagerOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IVisualStateManagerProtected_Impl: Sized {
    fn RaiseCurrentStateChanging(&mut self, stategroup: &::core::option::Option<VisualStateGroup>, oldstate: &::core::option::Option<VisualState>, newstate: &::core::option::Option<VisualState>, control: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
    fn RaiseCurrentStateChanged(&mut self, stategroup: &::core::option::Option<VisualStateGroup>, oldstate: &::core::option::Option<VisualState>, newstate: &::core::option::Option<VisualState>, control: &::core::option::Option<Controls::Control>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualStateManagerProtected {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerProtected";
}
#[cfg(all(feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IVisualStateManagerProtected_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerProtected_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateManagerProtected_Vtbl {
        unsafe extern "system" fn RaiseCurrentStateChanging<Impl: IVisualStateManagerProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stategroup: ::windows::core::RawPtr, oldstate: ::windows::core::RawPtr, newstate: ::windows::core::RawPtr, control: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RaiseCurrentStateChanging(
                    &*(&stategroup as *const <VisualStateGroup as ::windows::core::Abi>::Abi as *const <VisualStateGroup as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&newstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RaiseCurrentStateChanged<Impl: IVisualStateManagerProtected_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stategroup: ::windows::core::RawPtr, oldstate: ::windows::core::RawPtr, newstate: ::windows::core::RawPtr, control: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RaiseCurrentStateChanged(
                    &*(&stategroup as *const <VisualStateGroup as ::windows::core::Abi>::Abi as *const <VisualStateGroup as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&newstate as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                    &*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManagerProtected, BASE_OFFSET>(),
            RaiseCurrentStateChanging: RaiseCurrentStateChanging::<Impl, IMPL_OFFSET>,
            RaiseCurrentStateChanged: RaiseCurrentStateChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManagerProtected as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IVisualStateManagerStatics_Impl: Sized {
    fn GetVisualStateGroups(&mut self, obj: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VisualStateGroup>>;
    fn CustomVisualStateManagerProperty(&mut self) -> ::windows::core::Result<DependencyProperty>;
    fn GetCustomVisualStateManager(&mut self, obj: &::core::option::Option<FrameworkElement>) -> ::windows::core::Result<VisualStateManager>;
    fn SetCustomVisualStateManager(&mut self, obj: &::core::option::Option<FrameworkElement>, value: &::core::option::Option<VisualStateManager>) -> ::windows::core::Result<()>;
    fn GoToState(&mut self, control: &::core::option::Option<Controls::Control>, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualStateManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IVisualStateManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateManagerStatics_Vtbl {
        unsafe extern "system" fn GetVisualStateGroups<Impl: IVisualStateManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVisualStateGroups(&*(&obj as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomVisualStateManagerProperty<Impl: IVisualStateManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomVisualStateManagerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustomVisualStateManager<Impl: IVisualStateManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCustomVisualStateManager(&*(&obj as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomVisualStateManager<Impl: IVisualStateManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomVisualStateManager(&*(&obj as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <VisualStateManager as ::windows::core::Abi>::Abi as *const <VisualStateManager as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GoToState<Impl: IVisualStateManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: ::windows::core::RawPtr, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoToState(&*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType), &*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManagerStatics, BASE_OFFSET>(),
            GetVisualStateGroups: GetVisualStateGroups::<Impl, IMPL_OFFSET>,
            CustomVisualStateManagerProperty: CustomVisualStateManagerProperty::<Impl, IMPL_OFFSET>,
            GetCustomVisualStateManager: GetCustomVisualStateManager::<Impl, IMPL_OFFSET>,
            SetCustomVisualStateManager: SetCustomVisualStateManager::<Impl, IMPL_OFFSET>,
            GoToState: GoToState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IVisualTransition_Impl: Sized {
    fn GeneratedDuration(&mut self) -> ::windows::core::Result<Duration>;
    fn SetGeneratedDuration(&mut self, value: &Duration) -> ::windows::core::Result<()>;
    fn GeneratedEasingFunction(&mut self) -> ::windows::core::Result<Media::Animation::EasingFunctionBase>;
    fn SetGeneratedEasingFunction(&mut self, value: &::core::option::Option<Media::Animation::EasingFunctionBase>) -> ::windows::core::Result<()>;
    fn To(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTo(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn From(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrom(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Storyboard(&mut self) -> ::windows::core::Result<Media::Animation::Storyboard>;
    fn SetStoryboard(&mut self, value: &::core::option::Option<Media::Animation::Storyboard>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVisualTransition {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualTransition";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IVisualTransition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTransition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTransition_Vtbl {
        unsafe extern "system" fn GeneratedDuration<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneratedDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedDuration<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Duration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeneratedDuration(&*(&value as *const <Duration as ::windows::core::Abi>::Abi as *const <Duration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GeneratedEasingFunction<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GeneratedEasingFunction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGeneratedEasingFunction<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGeneratedEasingFunction(&*(&value as *const <Media::Animation::EasingFunctionBase as ::windows::core::Abi>::Abi as *const <Media::Animation::EasingFunctionBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn To<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).To() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTo<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTo(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn From<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).From() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrom<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrom(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Storyboard<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Storyboard() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStoryboard<Impl: IVisualTransition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStoryboard(&*(&value as *const <Media::Animation::Storyboard as ::windows::core::Abi>::Abi as *const <Media::Animation::Storyboard as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualTransition, BASE_OFFSET>(),
            GeneratedDuration: GeneratedDuration::<Impl, IMPL_OFFSET>,
            SetGeneratedDuration: SetGeneratedDuration::<Impl, IMPL_OFFSET>,
            GeneratedEasingFunction: GeneratedEasingFunction::<Impl, IMPL_OFFSET>,
            SetGeneratedEasingFunction: SetGeneratedEasingFunction::<Impl, IMPL_OFFSET>,
            To: To::<Impl, IMPL_OFFSET>,
            SetTo: SetTo::<Impl, IMPL_OFFSET>,
            From: From::<Impl, IMPL_OFFSET>,
            SetFrom: SetFrom::<Impl, IMPL_OFFSET>,
            Storyboard: Storyboard::<Impl, IMPL_OFFSET>,
            SetStoryboard: SetStoryboard::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTransition as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVisualTransitionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<VisualTransition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVisualTransitionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualTransitionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVisualTransitionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualTransitionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualTransitionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IVisualTransitionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualTransitionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualTransitionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IWindow_Impl: Sized {
    fn Bounds(&mut self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn Visible(&mut self) -> ::windows::core::Result<bool>;
    fn Content(&mut self) -> ::windows::core::Result<UIElement>;
    fn SetContent(&mut self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
    fn CoreWindow(&mut self) -> ::windows::core::Result<super::Core::CoreWindow>;
    fn Dispatcher(&mut self) -> ::windows::core::Result<super::Core::CoreDispatcher>;
    fn Activated(&mut self, handler: &::core::option::Option<WindowActivatedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&mut self, handler: &::core::option::Option<WindowClosedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SizeChanged(&mut self, handler: &::core::option::Option<WindowSizeChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VisibilityChanged(&mut self, handler: &::core::option::Option<WindowVisibilityChangedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Activate(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindow {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow";
}
#[cfg(all(feature = "Foundation", feature = "UI_Core", feature = "implement_exclusive"))]
impl IWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindow_Vtbl {
        unsafe extern "system" fn Bounds<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Visible<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Visible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Content<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContent<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContent(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CoreWindow<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CoreWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Dispatcher<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Activated<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activated(&*(&handler as *const <WindowActivatedEventHandler as ::windows::core::Abi>::Abi as *const <WindowActivatedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivated<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivated(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <WindowClosedEventHandler as ::windows::core::Abi>::Abi as *const <WindowClosedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SizeChanged<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SizeChanged(&*(&handler as *const <WindowSizeChangedEventHandler as ::windows::core::Abi>::Abi as *const <WindowSizeChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSizeChanged<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSizeChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VisibilityChanged<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VisibilityChanged(&*(&handler as *const <WindowVisibilityChangedEventHandler as ::windows::core::Abi>::Abi as *const <WindowVisibilityChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVisibilityChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Activate<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate().into()
        }
        unsafe extern "system" fn Close<Impl: IWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWindow, BASE_OFFSET>(),
            Bounds: Bounds::<Impl, IMPL_OFFSET>,
            Visible: Visible::<Impl, IMPL_OFFSET>,
            Content: Content::<Impl, IMPL_OFFSET>,
            SetContent: SetContent::<Impl, IMPL_OFFSET>,
            CoreWindow: CoreWindow::<Impl, IMPL_OFFSET>,
            Dispatcher: Dispatcher::<Impl, IMPL_OFFSET>,
            Activated: Activated::<Impl, IMPL_OFFSET>,
            RemoveActivated: RemoveActivated::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            SizeChanged: SizeChanged::<Impl, IMPL_OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Impl, IMPL_OFFSET>,
            VisibilityChanged: VisibilityChanged::<Impl, IMPL_OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindow as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow2_Impl: Sized {
    fn SetTitleBar(&mut self, value: &::core::option::Option<UIElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindow2 {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow2";
}
#[cfg(feature = "implement_exclusive")]
impl IWindow2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindow2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindow2_Vtbl {
        unsafe extern "system" fn SetTitleBar<Impl: IWindow2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitleBar(&*(&value as *const <UIElement as ::windows::core::Abi>::Abi as *const <UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindow2, BASE_OFFSET>(), SetTitleBar: SetTitleBar::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindow2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
pub trait IWindow3_Impl: Sized {
    fn Compositor(&mut self) -> ::windows::core::Result<super::Composition::Compositor>;
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWindow3 {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow3";
}
#[cfg(all(feature = "UI_Composition", feature = "implement_exclusive"))]
impl IWindow3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindow3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindow3_Vtbl {
        unsafe extern "system" fn Compositor<Impl: IWindow3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compositor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindow3, BASE_OFFSET>(), Compositor: Compositor::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindow3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindow4_Impl: Sized {
    fn UIContext(&mut self) -> ::windows::core::Result<super::UIContext>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindow4 {
    const NAME: &'static str = "Windows.UI.Xaml.IWindow4";
}
#[cfg(feature = "implement_exclusive")]
impl IWindow4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindow4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindow4_Vtbl {
        unsafe extern "system" fn UIContext<Impl: IWindow4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindow4, BASE_OFFSET>(), UIContext: UIContext::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindow4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowCreatedEventArgs_Impl: Sized {
    fn Window(&mut self) -> ::windows::core::Result<Window>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IWindowCreatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowCreatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowCreatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowCreatedEventArgs_Vtbl {
        unsafe extern "system" fn Window<Impl: IWindowCreatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Window() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowCreatedEventArgs, BASE_OFFSET>(), Window: Window::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowCreatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWindowStatics_Impl: Sized {
    fn Current(&mut self) -> ::windows::core::Result<Window>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWindowStatics {
    const NAME: &'static str = "Windows.UI.Xaml.IWindowStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IWindowStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowStatics_Vtbl {
        unsafe extern "system" fn Current<Impl: IWindowStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Current() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWindowStatics, BASE_OFFSET>(), Current: Current::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IXamlRoot_Impl: Sized {
    fn Content(&mut self) -> ::windows::core::Result<UIElement>;
    fn Size(&mut self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RasterizationScale(&mut self) -> ::windows::core::Result<f64>;
    fn IsHostVisible(&mut self) -> ::windows::core::Result<bool>;
    fn UIContext(&mut self) -> ::windows::core::Result<super::UIContext>;
    fn Changed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlRoot {
    const NAME: &'static str = "Windows.UI.Xaml.IXamlRoot";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IXamlRoot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRoot_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRoot_Vtbl {
        unsafe extern "system" fn Content<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Content() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RasterizationScale<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RasterizationScale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHostVisible<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHostVisible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UIContext<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UIContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Changed<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Changed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChanged<Impl: IXamlRoot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRoot, BASE_OFFSET>(),
            Content: Content::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
            RasterizationScale: RasterizationScale::<Impl, IMPL_OFFSET>,
            IsHostVisible: IsHostVisible::<Impl, IMPL_OFFSET>,
            UIContext: UIContext::<Impl, IMPL_OFFSET>,
            Changed: Changed::<Impl, IMPL_OFFSET>,
            RemoveChanged: RemoveChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRoot as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlRootChangedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlRootChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.IXamlRootChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlRootChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlRootChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlRootChangedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlRootChangedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlRootChangedEventArgs as ::windows::core::Interface>::IID
    }
}
