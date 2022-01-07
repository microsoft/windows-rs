#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarAutomationPeerImpl, const OFFSET: isize>() -> IAppBarAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppBarAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AppBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarAutomationPeerFactoryImpl, const OFFSET: isize>() -> IAppBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AppBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::AppBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppBarAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonAutomationPeerImpl, const OFFSET: isize>() -> IAppBarButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppBarButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AppBarButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IAppBarButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AppBarButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::AppBarButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppBarButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarToggleButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarToggleButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarToggleButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonAutomationPeerImpl, const OFFSET: isize>() -> IAppBarToggleButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppBarToggleButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AppBarToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AppBarToggleButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarToggleButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarToggleButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarToggleButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IAppBarToggleButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarToggleButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AppBarToggleButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::AppBarToggleButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppBarToggleButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutoSuggestBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutoSuggestBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAutoSuggestBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoSuggestBoxAutomationPeerImpl, const OFFSET: isize>() -> IAutoSuggestBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutoSuggestBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutoSuggestBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::AutoSuggestBox>) -> ::windows::core::Result<AutoSuggestBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutoSuggestBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutoSuggestBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutoSuggestBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutoSuggestBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> IAutoSuggestBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAutoSuggestBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AutoSuggestBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::AutoSuggestBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutoSuggestBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerImpl: Sized {
    fn EventsSource(&self) -> ::windows::core::Result<AutomationPeer>;
    fn SetEventsSource(&self, value: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
    fn GetPattern(&self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn RaiseAutomationEvent(&self, eventid: AutomationEvents) -> ::windows::core::Result<()>;
    fn RaisePropertyChangedEvent(&self, automationproperty: &::core::option::Option<super::AutomationProperty>, oldvalue: &::core::option::Option<::windows::core::IInspectable>, newvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetAcceleratorKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlType(&self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangle(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildren(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePoint(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatus(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledBy(&self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientation(&self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocus(&self) -> ::windows::core::Result<bool>;
    fn IsContentElement(&self) -> ::windows::core::Result<bool>;
    fn IsControlElement(&self) -> ::windows::core::Result<bool>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusable(&self) -> ::windows::core::Result<bool>;
    fn IsOffscreen(&self) -> ::windows::core::Result<bool>;
    fn IsPassword(&self) -> ::windows::core::Result<bool>;
    fn IsRequiredForForm(&self) -> ::windows::core::Result<bool>;
    fn SetFocus(&self) -> ::windows::core::Result<()>;
    fn GetParent(&self) -> ::windows::core::Result<AutomationPeer>;
    fn InvalidatePeer(&self) -> ::windows::core::Result<()>;
    fn GetPeerFromPoint(&self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSetting(&self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerImpl, const OFFSET: isize>() -> IAutomationPeerVtbl {
        unsafe extern "system" fn EventsSource<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventsSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsSource<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventsSource(&*(&value as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPattern<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPattern(patterninterface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaiseAutomationEvent<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: AutomationEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseAutomationEvent(eventid).into()
        }
        unsafe extern "system" fn RaisePropertyChangedEvent<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automationproperty: ::windows::core::RawPtr, oldvalue: *mut ::core::ffi::c_void, newvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RaisePropertyChangedEvent(
                    &*(&automationproperty as *const <super::AutomationProperty as ::windows::core::Abi>::Abi as *const <super::AutomationProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&newvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn GetAcceleratorKey<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKey<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlType<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationId<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangle<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassName<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePoint<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClickablePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpText<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatus<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemType<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledBy<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlType<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientation<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocus<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElement<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElement<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsKeyboardFocusable<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreen<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPassword<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForForm<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn GetParent<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidatePeer<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InvalidatePeer().into()
        }
        unsafe extern "system" fn GetPeerFromPoint<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeerFromPoint(&*(&point as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSetting<Impl: IAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLiveSetting() {
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
            ::windows::core::GetRuntimeClassName::<IAutomationPeer>,
            ::windows::core::GetTrustLevel,
            EventsSource::<Impl, OFFSET>,
            SetEventsSource::<Impl, OFFSET>,
            GetPattern::<Impl, OFFSET>,
            RaiseAutomationEvent::<Impl, OFFSET>,
            RaisePropertyChangedEvent::<Impl, OFFSET>,
            GetAcceleratorKey::<Impl, OFFSET>,
            GetAccessKey::<Impl, OFFSET>,
            GetAutomationControlType::<Impl, OFFSET>,
            GetAutomationId::<Impl, OFFSET>,
            GetBoundingRectangle::<Impl, OFFSET>,
            GetChildren::<Impl, OFFSET>,
            GetClassName::<Impl, OFFSET>,
            GetClickablePoint::<Impl, OFFSET>,
            GetHelpText::<Impl, OFFSET>,
            GetItemStatus::<Impl, OFFSET>,
            GetItemType::<Impl, OFFSET>,
            GetLabeledBy::<Impl, OFFSET>,
            GetLocalizedControlType::<Impl, OFFSET>,
            GetName::<Impl, OFFSET>,
            GetOrientation::<Impl, OFFSET>,
            HasKeyboardFocus::<Impl, OFFSET>,
            IsContentElement::<Impl, OFFSET>,
            IsControlElement::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            IsKeyboardFocusable::<Impl, OFFSET>,
            IsOffscreen::<Impl, OFFSET>,
            IsPassword::<Impl, OFFSET>,
            IsRequiredForForm::<Impl, OFFSET>,
            SetFocus::<Impl, OFFSET>,
            GetParent::<Impl, OFFSET>,
            InvalidatePeer::<Impl, OFFSET>,
            GetPeerFromPoint::<Impl, OFFSET>,
            GetLiveSetting::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer2Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer2";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer2Impl, const OFFSET: isize>() -> IAutomationPeer2Vtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer2>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer3Impl: Sized {
    fn Navigate(&self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPoint(&self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElement(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ShowContextMenu(&self) -> ::windows::core::Result<()>;
    fn GetControlledPeers(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
    fn GetAnnotations(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn SetParent(&self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
    fn RaiseTextEditTextChangedEvent(&self, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: &::core::option::Option<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn GetPositionInSet(&self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSet(&self) -> ::windows::core::Result<i32>;
    fn GetLevel(&self) -> ::windows::core::Result<i32>;
    fn RaiseStructureChangedEvent(&self, structurechangetype: AutomationStructureChangeType, child: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer3Impl, const OFFSET: isize>() -> IAutomationPeer3Vtbl {
        unsafe extern "system" fn Navigate<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Navigate(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementFromPoint<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElementFromPoint(&*(&pointinwindowcoordinates as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElement<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowContextMenu<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        unsafe extern "system" fn GetControlledPeers<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlledPeers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotations<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParent<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RaiseTextEditTextChangedEvent<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseTextEditTextChangedEvent(automationtexteditchangetype, &*(&changeddata as *const <super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPositionInSet<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSet<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaiseStructureChangedEvent<Impl: IAutomationPeer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, structurechangetype: AutomationStructureChangeType, child: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseStructureChangedEvent(structurechangetype, &*(&child as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAutomationPeer3>,
            ::windows::core::GetTrustLevel,
            Navigate::<Impl, OFFSET>,
            GetElementFromPoint::<Impl, OFFSET>,
            GetFocusedElement::<Impl, OFFSET>,
            ShowContextMenu::<Impl, OFFSET>,
            GetControlledPeers::<Impl, OFFSET>,
            GetAnnotations::<Impl, OFFSET>,
            SetParent::<Impl, OFFSET>,
            RaiseTextEditTextChangedEvent::<Impl, OFFSET>,
            GetPositionInSet::<Impl, OFFSET>,
            GetSizeOfSet::<Impl, OFFSET>,
            GetLevel::<Impl, OFFSET>,
            RaiseStructureChangedEvent::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer4Impl: Sized {
    fn GetLandmarkType(&self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer4Impl, const OFFSET: isize>() -> IAutomationPeer4Vtbl {
        unsafe extern "system" fn GetLandmarkType<Impl: IAutomationPeer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkType<Impl: IAutomationPeer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer4>, ::windows::core::GetTrustLevel, GetLandmarkType::<Impl, OFFSET>, GetLocalizedLandmarkType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer5Impl: Sized {
    fn IsPeripheral(&self) -> ::windows::core::Result<bool>;
    fn IsDataValidForForm(&self) -> ::windows::core::Result<bool>;
    fn GetFullDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer5";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer5Impl, const OFFSET: isize>() -> IAutomationPeer5Vtbl {
        unsafe extern "system" fn IsPeripheral<Impl: IAutomationPeer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForForm<Impl: IAutomationPeer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescription<Impl: IAutomationPeer5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer5>, ::windows::core::GetTrustLevel, IsPeripheral::<Impl, OFFSET>, IsDataValidForForm::<Impl, OFFSET>, GetFullDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer6Impl: Sized {
    fn GetCulture(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer6Impl, const OFFSET: isize>() -> IAutomationPeer6Vtbl {
        unsafe extern "system" fn GetCulture<Impl: IAutomationPeer6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCulture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer6>, ::windows::core::GetTrustLevel, GetCulture::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer7Impl: Sized {
    fn RaiseNotificationEvent(&self, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: &::windows::core::HSTRING, activityid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer7 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer7";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer7Impl, const OFFSET: isize>() -> IAutomationPeer7Vtbl {
        unsafe extern "system" fn RaiseNotificationEvent<Impl: IAutomationPeer7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaiseNotificationEvent(notificationkind, notificationprocessing, &*(&displaystring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer7>, ::windows::core::GetTrustLevel, RaiseNotificationEvent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer8Impl: Sized {
    fn GetHeadingLevel(&self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer8Impl, const OFFSET: isize>() -> IAutomationPeer8Vtbl {
        unsafe extern "system" fn GetHeadingLevel<Impl: IAutomationPeer8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer8>, ::windows::core::GetTrustLevel, GetHeadingLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeer9Impl: Sized {
    fn IsDialog(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeer9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeer9";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeer9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeer9Impl, const OFFSET: isize>() -> IAutomationPeer9Vtbl {
        unsafe extern "system" fn IsDialog<Impl: IAutomationPeer9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeer9>, ::windows::core::GetTrustLevel, IsDialog::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<super::AnnotationType>;
    fn SetType(&self, value: super::AnnotationType) -> ::windows::core::Result<()>;
    fn Peer(&self) -> ::windows::core::Result<AutomationPeer>;
    fn SetPeer(&self, value: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotation {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotation";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationImpl, const OFFSET: isize>() -> IAutomationPeerAnnotationVtbl {
        unsafe extern "system" fn Type<Impl: IAutomationPeerAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::AnnotationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IAutomationPeerAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Peer<Impl: IAutomationPeerAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Peer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPeer<Impl: IAutomationPeerAnnotationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPeer(&*(&value as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerAnnotation>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Peer::<Impl, OFFSET>, SetPeer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationFactoryImpl: Sized {
    fn CreateInstance(&self, r#type: super::AnnotationType) -> ::windows::core::Result<AutomationPeerAnnotation>;
    fn CreateWithPeerParameter(&self, r#type: super::AnnotationType, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<AutomationPeerAnnotation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotationFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotationFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationFactoryImpl, const OFFSET: isize>() -> IAutomationPeerAnnotationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationPeerAnnotationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPeerParameter<Impl: IAutomationPeerAnnotationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: super::AnnotationType, peer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithPeerParameter(r#type, &*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerAnnotationFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, CreateWithPeerParameter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerAnnotationStaticsImpl: Sized {
    fn TypeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PeerProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerAnnotationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerAnnotationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerAnnotationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerAnnotationStaticsImpl, const OFFSET: isize>() -> IAutomationPeerAnnotationStaticsVtbl {
        unsafe extern "system" fn TypeProperty<Impl: IAutomationPeerAnnotationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerProperty<Impl: IAutomationPeerAnnotationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerAnnotationStatics>, ::windows::core::GetTrustLevel, TypeProperty::<Impl, OFFSET>, PeerProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerFactoryImpl, const OFFSET: isize>() -> IAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverridesImpl: Sized {
    fn GetPatternCore(&self, patterninterface: PatternInterface) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAcceleratorKeyCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAccessKeyCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetAutomationControlTypeCore(&self) -> ::windows::core::Result<AutomationControlType>;
    fn GetAutomationIdCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetBoundingRectangleCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn GetChildrenCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeer>>;
    fn GetClassNameCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetClickablePointCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point>;
    fn GetHelpTextCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemStatusCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetItemTypeCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLabeledByCore(&self) -> ::windows::core::Result<AutomationPeer>;
    fn GetLocalizedControlTypeCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetNameCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetOrientationCore(&self) -> ::windows::core::Result<AutomationOrientation>;
    fn HasKeyboardFocusCore(&self) -> ::windows::core::Result<bool>;
    fn IsContentElementCore(&self) -> ::windows::core::Result<bool>;
    fn IsControlElementCore(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledCore(&self) -> ::windows::core::Result<bool>;
    fn IsKeyboardFocusableCore(&self) -> ::windows::core::Result<bool>;
    fn IsOffscreenCore(&self) -> ::windows::core::Result<bool>;
    fn IsPasswordCore(&self) -> ::windows::core::Result<bool>;
    fn IsRequiredForFormCore(&self) -> ::windows::core::Result<bool>;
    fn SetFocusCore(&self) -> ::windows::core::Result<()>;
    fn GetPeerFromPointCore(&self, point: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<AutomationPeer>;
    fn GetLiveSettingCore(&self) -> ::windows::core::Result<AutomationLiveSetting>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>() -> IAutomationPeerOverridesVtbl {
        unsafe extern "system" fn GetPatternCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPatternCore(patterninterface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcceleratorKeyCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcceleratorKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKeyCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlTypeCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationIdCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutomationIdCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangleCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangleCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassNameCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePointCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClickablePointCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpTextCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpTextCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatusCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemStatusCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemTypeCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledByCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLabeledByCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlTypeCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientationCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOrientationCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocusCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasKeyboardFocusCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElementCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsContentElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElementCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsControlElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabledCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyboardFocusableCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsKeyboardFocusableCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreenCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffscreenCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPasswordCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPasswordCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForFormCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRequiredForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusCore().into()
        }
        unsafe extern "system" fn GetPeerFromPointCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeerFromPointCore(&*(&point as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSettingCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLiveSettingCore() {
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
            ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides>,
            ::windows::core::GetTrustLevel,
            GetPatternCore::<Impl, OFFSET>,
            GetAcceleratorKeyCore::<Impl, OFFSET>,
            GetAccessKeyCore::<Impl, OFFSET>,
            GetAutomationControlTypeCore::<Impl, OFFSET>,
            GetAutomationIdCore::<Impl, OFFSET>,
            GetBoundingRectangleCore::<Impl, OFFSET>,
            GetChildrenCore::<Impl, OFFSET>,
            GetClassNameCore::<Impl, OFFSET>,
            GetClickablePointCore::<Impl, OFFSET>,
            GetHelpTextCore::<Impl, OFFSET>,
            GetItemStatusCore::<Impl, OFFSET>,
            GetItemTypeCore::<Impl, OFFSET>,
            GetLabeledByCore::<Impl, OFFSET>,
            GetLocalizedControlTypeCore::<Impl, OFFSET>,
            GetNameCore::<Impl, OFFSET>,
            GetOrientationCore::<Impl, OFFSET>,
            HasKeyboardFocusCore::<Impl, OFFSET>,
            IsContentElementCore::<Impl, OFFSET>,
            IsControlElementCore::<Impl, OFFSET>,
            IsEnabledCore::<Impl, OFFSET>,
            IsKeyboardFocusableCore::<Impl, OFFSET>,
            IsOffscreenCore::<Impl, OFFSET>,
            IsPasswordCore::<Impl, OFFSET>,
            IsRequiredForFormCore::<Impl, OFFSET>,
            SetFocusCore::<Impl, OFFSET>,
            GetPeerFromPointCore::<Impl, OFFSET>,
            GetLiveSettingCore::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides2Impl: Sized {
    fn ShowContextMenuCore(&self) -> ::windows::core::Result<()>;
    fn GetControlledPeersCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AutomationPeer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides2Impl, const OFFSET: isize>() -> IAutomationPeerOverrides2Vtbl {
        unsafe extern "system" fn ShowContextMenuCore<Impl: IAutomationPeerOverrides2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowContextMenuCore().into()
        }
        unsafe extern "system" fn GetControlledPeersCore<Impl: IAutomationPeerOverrides2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlledPeersCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides2>, ::windows::core::GetTrustLevel, ShowContextMenuCore::<Impl, OFFSET>, GetControlledPeersCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides3Impl: Sized {
    fn NavigateCore(&self, direction: AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetElementFromPointCore(&self, pointinwindowcoordinates: &super::super::super::super::Foundation::Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetFocusedElementCore(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetAnnotationsCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<AutomationPeerAnnotation>>;
    fn GetPositionInSetCore(&self) -> ::windows::core::Result<i32>;
    fn GetSizeOfSetCore(&self) -> ::windows::core::Result<i32>;
    fn GetLevelCore(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>() -> IAutomationPeerOverrides3Vtbl {
        unsafe extern "system" fn NavigateCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NavigateCore(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementFromPointCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElementFromPointCore(&*(&pointinwindowcoordinates as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationsCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAnnotationsCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionInSetCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPositionInSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSetCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSizeOfSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevelCore() {
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
            ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides3>,
            ::windows::core::GetTrustLevel,
            NavigateCore::<Impl, OFFSET>,
            GetElementFromPointCore::<Impl, OFFSET>,
            GetFocusedElementCore::<Impl, OFFSET>,
            GetAnnotationsCore::<Impl, OFFSET>,
            GetPositionInSetCore::<Impl, OFFSET>,
            GetSizeOfSetCore::<Impl, OFFSET>,
            GetLevelCore::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides4Impl: Sized {
    fn GetLandmarkTypeCore(&self) -> ::windows::core::Result<AutomationLandmarkType>;
    fn GetLocalizedLandmarkTypeCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides4";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides4Impl, const OFFSET: isize>() -> IAutomationPeerOverrides4Vtbl {
        unsafe extern "system" fn GetLandmarkTypeCore<Impl: IAutomationPeerOverrides4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkTypeCore<Impl: IAutomationPeerOverrides4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalizedLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides4>, ::windows::core::GetTrustLevel, GetLandmarkTypeCore::<Impl, OFFSET>, GetLocalizedLandmarkTypeCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides5Impl: Sized {
    fn IsPeripheralCore(&self) -> ::windows::core::Result<bool>;
    fn IsDataValidForFormCore(&self) -> ::windows::core::Result<bool>;
    fn GetFullDescriptionCore(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDescribedByCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsToCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
    fn GetFlowsFromCore(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IIterable<AutomationPeer>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides5 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides5";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>() -> IAutomationPeerOverrides5Vtbl {
        unsafe extern "system" fn IsPeripheralCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPeripheralCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForFormCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDataValidForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescriptionCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullDescriptionCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescribedByCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescribedByCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsToCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowsToCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsFromCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFlowsFromCore() {
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
            ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides5>,
            ::windows::core::GetTrustLevel,
            IsPeripheralCore::<Impl, OFFSET>,
            IsDataValidForFormCore::<Impl, OFFSET>,
            GetFullDescriptionCore::<Impl, OFFSET>,
            GetDescribedByCore::<Impl, OFFSET>,
            GetFlowsToCore::<Impl, OFFSET>,
            GetFlowsFromCore::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides6Impl: Sized {
    fn GetCultureCore(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides6 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides6";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides6Impl, const OFFSET: isize>() -> IAutomationPeerOverrides6Vtbl {
        unsafe extern "system" fn GetCultureCore<Impl: IAutomationPeerOverrides6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCultureCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides6>, ::windows::core::GetTrustLevel, GetCultureCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides8Impl: Sized {
    fn GetHeadingLevelCore(&self) -> ::windows::core::Result<AutomationHeadingLevel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides8";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides8Impl, const OFFSET: isize>() -> IAutomationPeerOverrides8Vtbl {
        unsafe extern "system" fn GetHeadingLevelCore<Impl: IAutomationPeerOverrides8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeadingLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides8>, ::windows::core::GetTrustLevel, GetHeadingLevelCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerOverrides9Impl: Sized {
    fn IsDialogCore(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerOverrides9";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerOverrides9Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerOverrides9Impl, const OFFSET: isize>() -> IAutomationPeerOverrides9Vtbl {
        unsafe extern "system" fn IsDialogCore<Impl: IAutomationPeerOverrides9Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDialogCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides9>, ::windows::core::GetTrustLevel, IsDialogCore::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerProtectedImpl: Sized {
    fn PeerFromProvider(&self, provider: &::core::option::Option<super::Provider::IRawElementProviderSimple>) -> ::windows::core::Result<AutomationPeer>;
    fn ProviderFromPeer(&self, peer: &::core::option::Option<AutomationPeer>) -> ::windows::core::Result<super::Provider::IRawElementProviderSimple>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerProtected {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerProtected";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerProtectedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerProtectedImpl, const OFFSET: isize>() -> IAutomationPeerProtectedVtbl {
        unsafe extern "system" fn PeerFromProvider<Impl: IAutomationPeerProtectedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PeerFromProvider(&*(&provider as *const <super::Provider::IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <super::Provider::IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderFromPeer<Impl: IAutomationPeerProtectedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProviderFromPeer(&*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerProtected>, ::windows::core::GetTrustLevel, PeerFromProvider::<Impl, OFFSET>, ProviderFromPeer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerStaticsImpl: Sized {
    fn ListenerExists(&self, eventid: AutomationEvents) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerStaticsImpl, const OFFSET: isize>() -> IAutomationPeerStaticsVtbl {
        unsafe extern "system" fn ListenerExists<Impl: IAutomationPeerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventid: AutomationEvents, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListenerExists(eventid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerStatics>, ::windows::core::GetTrustLevel, ListenerExists::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationPeerStatics3Impl: Sized {
    fn GenerateRawElementProviderRuntimeId(&self) -> ::windows::core::Result<RawElementProviderRuntimeId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationPeerStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAutomationPeerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationPeerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationPeerStatics3Impl, const OFFSET: isize>() -> IAutomationPeerStatics3Vtbl {
        unsafe extern "system" fn GenerateRawElementProviderRuntimeId<Impl: IAutomationPeerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RawElementProviderRuntimeId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateRawElementProviderRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationPeerStatics3>, ::windows::core::GetTrustLevel, GenerateRawElementProviderRuntimeId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonAutomationPeerImpl, const OFFSET: isize>() -> IButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Button>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Button as ::windows::core::Abi>::Abi as *const <super::super::Controls::Button as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseAutomationPeerImpl, const OFFSET: isize>() -> IButtonBaseAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IButtonBaseAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ButtonBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBaseAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IButtonBaseAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseAutomationPeerFactoryImpl, const OFFSET: isize>() -> IButtonBaseAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IButtonBaseAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ButtonBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ButtonBase as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IButtonBaseAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarDatePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICalendarDatePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarDatePickerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarDatePickerAutomationPeerImpl, const OFFSET: isize>() -> ICalendarDatePickerAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICalendarDatePickerAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarDatePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::CalendarDatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CalendarDatePickerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarDatePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICalendarDatePickerAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarDatePickerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarDatePickerAutomationPeerFactoryImpl, const OFFSET: isize>() -> ICalendarDatePickerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICalendarDatePickerAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::CalendarDatePicker as ::windows::core::Abi>::Abi as *const <super::super::Controls::CalendarDatePicker as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICalendarDatePickerAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICaptureElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICaptureElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICaptureElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICaptureElementAutomationPeerImpl, const OFFSET: isize>() -> ICaptureElementAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICaptureElementAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICaptureElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::CaptureElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CaptureElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICaptureElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICaptureElementAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICaptureElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICaptureElementAutomationPeerFactoryImpl, const OFFSET: isize>() -> ICaptureElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICaptureElementAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::CaptureElement as ::windows::core::Abi>::Abi as *const <super::super::Controls::CaptureElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICaptureElementAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICheckBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICheckBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ICheckBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckBoxAutomationPeerImpl, const OFFSET: isize>() -> ICheckBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICheckBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICheckBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::CheckBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CheckBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICheckBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ICheckBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICheckBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> ICheckBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICheckBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::CheckBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::CheckBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICheckBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorPickerSliderAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderAutomationPeerImpl, const OFFSET: isize>() -> IColorPickerSliderAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorPickerSliderAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorPickerSlider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSliderAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorPickerSliderAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderAutomationPeerFactoryImpl, const OFFSET: isize>() -> IColorPickerSliderAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IColorPickerSliderAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ColorPickerSlider as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ColorPickerSlider as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorPickerSliderAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorSpectrumAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumAutomationPeerImpl, const OFFSET: isize>() -> IColorSpectrumAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorSpectrumAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ColorSpectrum>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrumAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IColorSpectrumAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumAutomationPeerFactoryImpl, const OFFSET: isize>() -> IColorSpectrumAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IColorSpectrumAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ColorSpectrum as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ColorSpectrum as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IColorSpectrumAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxAutomationPeerImpl, const OFFSET: isize>() -> IComboBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComboBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ComboBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> IComboBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IComboBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ComboBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::ComboBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComboBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemAutomationPeerImpl, const OFFSET: isize>() -> IComboBoxItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComboBoxItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ComboBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IComboBoxItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IComboBoxItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ComboBoxItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ComboBoxItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComboBoxItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemDataAutomationPeerImpl, const OFFSET: isize>() -> IComboBoxItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComboBoxItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ComboBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ComboBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IComboBoxItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxItemDataAutomationPeerFactoryImpl, const OFFSET: isize>() -> IComboBoxItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IComboBoxItemDataAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <ComboBoxAutomationPeer as ::windows::core::Abi>::Abi as *const <ComboBoxAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComboBoxItemDataAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerAutomationPeerImpl, const OFFSET: isize>() -> IDatePickerAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatePickerAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::DatePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DatePickerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerAutomationPeerFactoryImpl, const OFFSET: isize>() -> IDatePickerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IDatePickerAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::DatePicker as ::windows::core::Abi>::Abi as *const <super::super::Controls::DatePicker as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatePickerAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatePickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatePickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IDatePickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IDatePickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatePickerFlyoutPresenterAutomationPeerImpl, const OFFSET: isize>() -> IDatePickerFlyoutPresenterAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatePickerFlyoutPresenterAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewAutomationPeerImpl, const OFFSET: isize>() -> IFlipViewAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlipViewAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::FlipView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewAutomationPeerFactoryImpl, const OFFSET: isize>() -> IFlipViewAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlipViewAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::FlipView as ::windows::core::Abi>::Abi as *const <super::super::Controls::FlipView as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlipViewAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemAutomationPeerImpl, const OFFSET: isize>() -> IFlipViewItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlipViewItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::FlipViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IFlipViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlipViewItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::FlipViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::FlipViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlipViewItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemDataAutomationPeerImpl, const OFFSET: isize>() -> IFlipViewItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlipViewItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlipViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<FlipViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlipViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlipViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlipViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlipViewItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlipViewItemDataAutomationPeerFactoryImpl, const OFFSET: isize>() -> IFlipViewItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IFlipViewItemDataAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <FlipViewAutomationPeer as ::windows::core::Abi>::Abi as *const <FlipViewAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlipViewItemDataAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutPresenterAutomationPeerImpl, const OFFSET: isize>() -> IFlyoutPresenterAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlyoutPresenterAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutPresenterAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::FlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutPresenterAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutPresenterAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFlyoutPresenterAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutPresenterAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: isize>() -> IFlyoutPresenterAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::FlyoutPresenter as ::windows::core::Abi>::Abi as *const <super::super::Controls::FlyoutPresenter as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFlyoutPresenterAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerImpl: Sized {
    fn Owner(&self) -> ::windows::core::Result<super::super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerImpl, const OFFSET: isize>() -> IFrameworkElementAutomationPeerVtbl {
        unsafe extern "system" fn Owner<Impl: IFrameworkElementAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFrameworkElementAutomationPeer>, ::windows::core::GetTrustLevel, Owner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::FrameworkElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FrameworkElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerFactoryImpl, const OFFSET: isize>() -> IFrameworkElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFrameworkElementAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFrameworkElementAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameworkElementAutomationPeerStaticsImpl: Sized {
    fn FromElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
    fn CreatePeerForElement(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<AutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFrameworkElementAutomationPeerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IFrameworkElementAutomationPeerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFrameworkElementAutomationPeerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementAutomationPeerStaticsImpl, const OFFSET: isize>() -> IFrameworkElementAutomationPeerStaticsVtbl {
        unsafe extern "system" fn FromElement<Impl: IFrameworkElementAutomationPeerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePeerForElement<Impl: IFrameworkElementAutomationPeerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePeerForElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFrameworkElementAutomationPeerStatics>, ::windows::core::GetTrustLevel, FromElement::<Impl, OFFSET>, CreatePeerForElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewAutomationPeerImpl, const OFFSET: isize>() -> IGridViewAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GridView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewAutomationPeerFactoryImpl, const OFFSET: isize>() -> IGridViewAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GridView as ::windows::core::Abi>::Abi as *const <super::super::Controls::GridView as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewHeaderItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewHeaderItemAutomationPeerImpl, const OFFSET: isize>() -> IGridViewHeaderItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewHeaderItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GridViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewHeaderItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewHeaderItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewHeaderItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IGridViewHeaderItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GridViewHeaderItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::GridViewHeaderItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewHeaderItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemAutomationPeerImpl, const OFFSET: isize>() -> IGridViewItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GridViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IGridViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GridViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::GridViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemDataAutomationPeerImpl, const OFFSET: isize>() -> IGridViewItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<GridViewAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGridViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemDataAutomationPeerFactoryImpl, const OFFSET: isize>() -> IGridViewItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IGridViewItemDataAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <GridViewAutomationPeer as ::windows::core::Abi>::Abi as *const <GridViewAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGridViewItemDataAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGroupItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGroupItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IGroupItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupItemAutomationPeerImpl, const OFFSET: isize>() -> IGroupItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGroupItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGroupItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::GroupItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GroupItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGroupItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IGroupItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGroupItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGroupItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IGroupItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGroupItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GroupItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::GroupItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGroupItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHubAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubAutomationPeerImpl, const OFFSET: isize>() -> IHubAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHubAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Hub>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHubAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubAutomationPeerFactoryImpl, const OFFSET: isize>() -> IHubAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHubAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Hub as ::windows::core::Abi>::Abi as *const <super::super::Controls::Hub as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHubAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubSectionAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubSectionAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHubSectionAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubSectionAutomationPeerImpl, const OFFSET: isize>() -> IHubSectionAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHubSectionAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHubSectionAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::HubSection>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HubSectionAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHubSectionAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHubSectionAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHubSectionAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHubSectionAutomationPeerFactoryImpl, const OFFSET: isize>() -> IHubSectionAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHubSectionAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::HubSection as ::windows::core::Abi>::Abi as *const <super::super::Controls::HubSection as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHubSectionAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHyperlinkButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkButtonAutomationPeerImpl, const OFFSET: isize>() -> IHyperlinkButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHyperlinkButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::HyperlinkButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<HyperlinkButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHyperlinkButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IHyperlinkButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IHyperlinkButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHyperlinkButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IHyperlinkButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHyperlinkButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::HyperlinkButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::HyperlinkButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHyperlinkButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IImageAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IImageAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageAutomationPeerImpl, const OFFSET: isize>() -> IImageAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImageAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IImageAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Image>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ImageAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IImageAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IImageAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IImageAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IImageAutomationPeerFactoryImpl, const OFFSET: isize>() -> IImageAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IImageAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Image as ::windows::core::Abi>::Abi as *const <super::super::Controls::Image as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IImageAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInkToolbarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInkToolbarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IInkToolbarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IInkToolbarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInkToolbarAutomationPeerImpl, const OFFSET: isize>() -> IInkToolbarAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IInkToolbarAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeerImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn ItemsControlAutomationPeer(&self) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemAutomationPeerImpl, const OFFSET: isize>() -> IItemAutomationPeerVtbl {
        unsafe extern "system" fn Item<Impl: IItemAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsControlAutomationPeer<Impl: IItemAutomationPeerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsControlAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemAutomationPeer>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, ItemsControlAutomationPeer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ItemsControlAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <ItemsControlAutomationPeer as ::windows::core::Abi>::Abi as *const <ItemsControlAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerImpl, const OFFSET: isize>() -> IItemsControlAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeer2Impl: Sized {
    fn CreateItemAutomationPeer(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeer2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeer2";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeer2Impl, const OFFSET: isize>() -> IItemsControlAutomationPeer2Vtbl {
        unsafe extern "system" fn CreateItemAutomationPeer<Impl: IItemsControlAutomationPeer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateItemAutomationPeer(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeer2>, ::windows::core::GetTrustLevel, CreateItemAutomationPeer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ItemsControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemsControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerFactoryImpl, const OFFSET: isize>() -> IItemsControlAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IItemsControlAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ItemsControl as ::windows::core::Abi>::Abi as *const <super::super::Controls::ItemsControl as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsControlAutomationPeerOverrides2Impl: Sized {
    fn OnCreateItemAutomationPeer(&self, item: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsControlAutomationPeerOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IItemsControlAutomationPeerOverrides2";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsControlAutomationPeerOverrides2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsControlAutomationPeerOverrides2Impl, const OFFSET: isize>() -> IItemsControlAutomationPeerOverrides2Vtbl {
        unsafe extern "system" fn OnCreateItemAutomationPeer<Impl: IItemsControlAutomationPeerOverrides2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCreateItemAutomationPeer(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeerOverrides2>, ::windows::core::GetTrustLevel, OnCreateItemAutomationPeer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxAutomationPeerImpl, const OFFSET: isize>() -> IListBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemAutomationPeerImpl, const OFFSET: isize>() -> IListBoxItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListBoxItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListBoxItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListBoxItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListBoxItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListBoxItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListBoxItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListBoxItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemDataAutomationPeerImpl, const OFFSET: isize>() -> IListBoxItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListBoxItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListBoxItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListBoxAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListBoxItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListBoxItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListBoxItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListBoxItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListBoxItemDataAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListBoxItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IListBoxItemDataAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <ListBoxAutomationPeer as ::windows::core::Abi>::Abi as *const <ListBoxAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListBoxItemDataAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListPickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListPickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListPickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListPickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListPickerFlyoutPresenterAutomationPeerImpl, const OFFSET: isize>() -> IListPickerFlyoutPresenterAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListPickerFlyoutPresenterAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewAutomationPeerImpl, const OFFSET: isize>() -> IListViewAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListView>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListViewAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListView as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListView as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseAutomationPeerImpl, const OFFSET: isize>() -> IListViewBaseAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewBaseAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListViewBaseAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewBaseAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewBaseAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseHeaderItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseHeaderItemAutomationPeerImpl, const OFFSET: isize>() -> IListViewBaseHeaderItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewBaseHeaderItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewBaseHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewBaseHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewBaseHeaderItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewBaseHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewBaseHeaderItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewBaseHeaderItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewBaseHeaderItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListViewBaseHeaderItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewBaseHeaderItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewBaseHeaderItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBaseHeaderItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewBaseHeaderItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewHeaderItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewHeaderItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewHeaderItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewHeaderItemAutomationPeerImpl, const OFFSET: isize>() -> IListViewHeaderItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewHeaderItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewHeaderItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewHeaderItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewHeaderItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewHeaderItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewHeaderItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewHeaderItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListViewHeaderItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewHeaderItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewHeaderItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewHeaderItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemAutomationPeerImpl, const OFFSET: isize>() -> IListViewItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ListViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemDataAutomationPeerImpl, const OFFSET: isize>() -> IListViewItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<ListViewBaseAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IListViewItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemDataAutomationPeerFactoryImpl, const OFFSET: isize>() -> IListViewItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IListViewItemDataAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <ListViewBaseAutomationPeer as ::windows::core::Abi>::Abi as *const <ListViewBaseAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IListViewItemDataAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorAutomationPeerImpl, const OFFSET: isize>() -> ILoopingSelectorAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoopingSelectorAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemAutomationPeerImpl, const OFFSET: isize>() -> ILoopingSelectorItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoopingSelectorItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ILoopingSelectorItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemDataAutomationPeerImpl, const OFFSET: isize>() -> ILoopingSelectorItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILoopingSelectorItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMapControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMapControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMapControlAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMapControlAutomationPeerImpl, const OFFSET: isize>() -> IMapControlAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMapControlAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaElementAutomationPeerImpl, const OFFSET: isize>() -> IMediaElementAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaElementAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MediaElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaElementAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaElementAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMediaElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaElementAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MediaElement as ::windows::core::Abi>::Abi as *const <super::super::Controls::MediaElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaElementAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerElementAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaPlayerElementAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerElementAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerElementAutomationPeerImpl, const OFFSET: isize>() -> IMediaPlayerElementAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerElementAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaPlayerElementAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MediaPlayerElement>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaPlayerElementAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaPlayerElementAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaPlayerElementAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaPlayerElementAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaPlayerElementAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMediaPlayerElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaPlayerElementAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MediaPlayerElement as ::windows::core::Abi>::Abi as *const <super::super::Controls::MediaPlayerElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaPlayerElementAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTransportControlsAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaTransportControlsAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTransportControlsAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsAutomationPeerImpl, const OFFSET: isize>() -> IMediaTransportControlsAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaTransportControlsAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaTransportControlsAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MediaTransportControls>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MediaTransportControlsAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaTransportControlsAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMediaTransportControlsAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaTransportControlsAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaTransportControlsAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMediaTransportControlsAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaTransportControlsAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MediaTransportControls as ::windows::core::Abi>::Abi as *const <super::super::Controls::MediaTransportControls as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaTransportControlsAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarAutomationPeerImpl, const OFFSET: isize>() -> IMenuBarAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuBarAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<super::super::Controls::MenuBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMenuBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMenuBarAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&owner as *const <super::super::Controls::MenuBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuBarAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarItemAutomationPeerImpl, const OFFSET: isize>() -> IMenuBarItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuBarItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuBarItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstance(&self, owner: &::core::option::Option<super::super::Controls::MenuBarItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuBarItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuBarItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuBarItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuBarItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuBarItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMenuBarItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMenuBarItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&owner as *const <super::super::Controls::MenuBarItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuBarItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuBarItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemAutomationPeerImpl, const OFFSET: isize>() -> IMenuFlyoutItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuFlyoutItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMenuFlyoutItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MenuFlyoutItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuFlyoutItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuFlyoutItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterAutomationPeerImpl, const OFFSET: isize>() -> IMenuFlyoutPresenterAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuFlyoutPresenterAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::MenuFlyoutPresenter>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MenuFlyoutPresenterAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IMenuFlyoutPresenterAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutPresenterAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: isize>() -> IMenuFlyoutPresenterAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMenuFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MenuFlyoutPresenter as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuFlyoutPresenter as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMenuFlyoutPresenterAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.INavigationViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemAutomationPeerImpl, const OFFSET: isize>() -> INavigationViewItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationViewItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::NavigationViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.INavigationViewItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> INavigationViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: INavigationViewItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::NavigationViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::NavigationViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INavigationViewItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPasswordBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPasswordBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPasswordBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordBoxAutomationPeerImpl, const OFFSET: isize>() -> IPasswordBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPasswordBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPasswordBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::PasswordBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PasswordBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPasswordBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPasswordBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPasswordBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPasswordBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> IPasswordBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPasswordBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::PasswordBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::PasswordBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPasswordBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPersonPictureAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPersonPictureAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPersonPictureAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersonPictureAutomationPeerImpl, const OFFSET: isize>() -> IPersonPictureAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersonPictureAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPersonPictureAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::PersonPicture>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PersonPictureAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPersonPictureAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPersonPictureAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPersonPictureAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersonPictureAutomationPeerFactoryImpl, const OFFSET: isize>() -> IPersonPictureAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPersonPictureAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::PersonPicture as ::windows::core::Abi>::Abi as *const <super::super::Controls::PersonPicture as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPersonPictureAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutPresenterAutomationPeerImpl, const OFFSET: isize>() -> IPickerFlyoutPresenterAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPickerFlyoutPresenterAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotAutomationPeerImpl, const OFFSET: isize>() -> IPivotAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPivotAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Pivot>) -> ::windows::core::Result<PivotAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotAutomationPeerFactoryImpl, const OFFSET: isize>() -> IPivotAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPivotAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Pivot as ::windows::core::Abi>::Abi as *const <super::super::Controls::Pivot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPivotAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemAutomationPeerImpl, const OFFSET: isize>() -> IPivotItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPivotItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::PivotItem>) -> ::windows::core::Result<PivotItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IPivotItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPivotItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::PivotItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::PivotItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPivotItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemDataAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemDataAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemDataAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemDataAutomationPeerImpl, const OFFSET: isize>() -> IPivotItemDataAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPivotItemDataAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotItemDataAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<PivotAutomationPeer>) -> ::windows::core::Result<PivotItemDataAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotItemDataAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IPivotItemDataAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotItemDataAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotItemDataAutomationPeerFactoryImpl, const OFFSET: isize>() -> IPivotItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IPivotItemDataAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&parent as *const <PivotAutomationPeer as ::windows::core::Abi>::Abi as *const <PivotAutomationPeer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPivotItemDataAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarAutomationPeerImpl, const OFFSET: isize>() -> IProgressBarAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressBarAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ProgressBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressBarAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarAutomationPeerFactoryImpl, const OFFSET: isize>() -> IProgressBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IProgressBarAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ProgressBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::ProgressBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressBarAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressRingAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressRingAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressRingAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingAutomationPeerImpl, const OFFSET: isize>() -> IProgressRingAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressRingAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ProgressRing>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ProgressRingAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressRingAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IProgressRingAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressRingAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingAutomationPeerFactoryImpl, const OFFSET: isize>() -> IProgressRingAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IProgressRingAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ProgressRing as ::windows::core::Abi>::Abi as *const <super::super::Controls::ProgressRing as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressRingAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadioButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRadioButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRadioButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioButtonAutomationPeerImpl, const OFFSET: isize>() -> IRadioButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadioButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRadioButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RadioButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RadioButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRadioButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRadioButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRadioButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRadioButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRadioButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRadioButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RadioButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::RadioButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRadioButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRangeBaseAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseAutomationPeerImpl, const OFFSET: isize>() -> IRangeBaseAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRangeBaseAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::RangeBase>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBaseAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRangeBaseAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRangeBaseAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRangeBaseAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::RangeBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::RangeBase as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRangeBaseAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatingControlAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRatingControlAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRatingControlAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatingControlAutomationPeerImpl, const OFFSET: isize>() -> IRatingControlAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRatingControlAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRatingControlAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RatingControl>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RatingControlAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRatingControlAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRatingControlAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRatingControlAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRatingControlAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRatingControlAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRatingControlAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RatingControl as ::windows::core::Abi>::Abi as *const <super::super::Controls::RatingControl as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRatingControlAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRepeatButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonAutomationPeerImpl, const OFFSET: isize>() -> IRepeatButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepeatButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::RepeatButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RepeatButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRepeatButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRepeatButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRepeatButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::RepeatButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::RepeatButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRepeatButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichEditBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichEditBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichEditBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditBoxAutomationPeerImpl, const OFFSET: isize>() -> IRichEditBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichEditBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichEditBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RichEditBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichEditBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichEditBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichEditBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRichEditBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichEditBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRichEditBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichEditBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RichEditBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::RichEditBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichEditBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockAutomationPeerImpl, const OFFSET: isize>() -> IRichTextBlockAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichTextBlockAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RichTextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRichTextBlockAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichTextBlockAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RichTextBlock as ::windows::core::Abi>::Abi as *const <super::super::Controls::RichTextBlock as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichTextBlockAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockOverflowAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockOverflowAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockOverflowAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockOverflowAutomationPeerImpl, const OFFSET: isize>() -> IRichTextBlockOverflowAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichTextBlockOverflowAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRichTextBlockOverflowAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::RichTextBlockOverflow>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RichTextBlockOverflowAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRichTextBlockOverflowAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IRichTextBlockOverflowAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRichTextBlockOverflowAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRichTextBlockOverflowAutomationPeerFactoryImpl, const OFFSET: isize>() -> IRichTextBlockOverflowAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichTextBlockOverflowAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RichTextBlockOverflow as ::windows::core::Abi>::Abi as *const <super::super::Controls::RichTextBlockOverflow as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRichTextBlockOverflowAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarAutomationPeerImpl, const OFFSET: isize>() -> IScrollBarAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollBarAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ScrollBar>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollBarAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBarAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollBarAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarAutomationPeerFactoryImpl, const OFFSET: isize>() -> IScrollBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IScrollBarAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ScrollBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ScrollBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollBarAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollViewerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollViewerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollViewerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollViewerAutomationPeerImpl, const OFFSET: isize>() -> IScrollViewerAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollViewerAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollViewerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ScrollViewer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ScrollViewerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollViewerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IScrollViewerAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollViewerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollViewerAutomationPeerFactoryImpl, const OFFSET: isize>() -> IScrollViewerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IScrollViewerAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ScrollViewer as ::windows::core::Abi>::Abi as *const <super::super::Controls::ScrollViewer as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IScrollViewerAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISearchBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchBoxAutomationPeerImpl, const OFFSET: isize>() -> ISearchBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISearchBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISearchBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::SearchBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SearchBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISearchBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISearchBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISearchBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISearchBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> ISearchBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISearchBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::SearchBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::SearchBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISearchBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorAutomationPeerImpl, const OFFSET: isize>() -> ISelectorAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectorAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::Selector>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorAutomationPeerFactoryImpl, const OFFSET: isize>() -> ISelectorAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISelectorAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::Selector as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::Selector as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectorAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemAutomationPeerImpl, const OFFSET: isize>() -> ISelectorItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectorItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithParentAndItem(&self, item: &::core::option::Option<::windows::core::IInspectable>, parent: &::core::option::Option<SelectorAutomationPeer>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISelectorItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> ISelectorItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: ISelectorItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(
                &*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&parent as *const <SelectorAutomationPeer as ::windows::core::Abi>::Abi as *const <SelectorAutomationPeer as ::windows::core::DefaultType>::DefaultType),
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectorItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISemanticZoomAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISemanticZoomAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISemanticZoomAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomAutomationPeerImpl, const OFFSET: isize>() -> ISemanticZoomAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISemanticZoomAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISemanticZoomAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::SemanticZoom>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SemanticZoomAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISemanticZoomAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISemanticZoomAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISemanticZoomAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISemanticZoomAutomationPeerFactoryImpl, const OFFSET: isize>() -> ISemanticZoomAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISemanticZoomAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::SemanticZoom as ::windows::core::Abi>::Abi as *const <super::super::Controls::SemanticZoom as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISemanticZoomAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsFlyoutAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISettingsFlyoutAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsFlyoutAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutAutomationPeerImpl, const OFFSET: isize>() -> ISettingsFlyoutAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISettingsFlyoutAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::SettingsFlyout>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SettingsFlyoutAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsFlyoutAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISettingsFlyoutAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsFlyoutAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutAutomationPeerFactoryImpl, const OFFSET: isize>() -> ISettingsFlyoutAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISettingsFlyoutAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::SettingsFlyout as ::windows::core::Abi>::Abi as *const <super::super::Controls::SettingsFlyout as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISettingsFlyoutAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISliderAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISliderAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ISliderAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISliderAutomationPeerImpl, const OFFSET: isize>() -> ISliderAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISliderAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISliderAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Slider>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SliderAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISliderAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ISliderAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISliderAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISliderAutomationPeerFactoryImpl, const OFFSET: isize>() -> ISliderAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISliderAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Slider as ::windows::core::Abi>::Abi as *const <super::super::Controls::Slider as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISliderAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBlockAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBlockAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBlockAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBlockAutomationPeerImpl, const OFFSET: isize>() -> ITextBlockAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextBlockAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBlockAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TextBlock>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBlockAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBlockAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBlockAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBlockAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBlockAutomationPeerFactoryImpl, const OFFSET: isize>() -> ITextBlockAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITextBlockAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TextBlock as ::windows::core::Abi>::Abi as *const <super::super::Controls::TextBlock as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextBlockAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBoxAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBoxAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBoxAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBoxAutomationPeerImpl, const OFFSET: isize>() -> ITextBoxAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextBoxAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITextBoxAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TextBox>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TextBoxAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITextBoxAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITextBoxAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITextBoxAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITextBoxAutomationPeerFactoryImpl, const OFFSET: isize>() -> ITextBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITextBoxAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TextBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::TextBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITextBoxAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumbAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IThumbAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbAutomationPeerImpl, const OFFSET: isize>() -> IThumbAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IThumbAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::Thumb>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ThumbAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumbAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IThumbAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbAutomationPeerFactoryImpl, const OFFSET: isize>() -> IThumbAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IThumbAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::Thumb as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::Thumb as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IThumbAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerAutomationPeerImpl, const OFFSET: isize>() -> ITimePickerAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimePickerAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TimePicker>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TimePickerAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerAutomationPeerFactoryImpl, const OFFSET: isize>() -> ITimePickerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITimePickerAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TimePicker as ::windows::core::Abi>::Abi as *const <super::super::Controls::TimePicker as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimePickerAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimePickerFlyoutPresenterAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITimePickerFlyoutPresenterAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITimePickerFlyoutPresenterAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITimePickerFlyoutPresenterAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimePickerFlyoutPresenterAutomationPeerImpl, const OFFSET: isize>() -> ITimePickerFlyoutPresenterAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITimePickerFlyoutPresenterAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleButtonAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonAutomationPeerImpl, const OFFSET: isize>() -> IToggleButtonAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleButtonAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::Primitives::ToggleButton>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButtonAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleButtonAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonAutomationPeerFactoryImpl, const OFFSET: isize>() -> IToggleButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleButtonAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ToggleButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ToggleButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleButtonAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleMenuFlyoutItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleMenuFlyoutItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleMenuFlyoutItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleMenuFlyoutItemAutomationPeerImpl, const OFFSET: isize>() -> IToggleMenuFlyoutItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleMenuFlyoutItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleMenuFlyoutItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ToggleMenuFlyoutItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleMenuFlyoutItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleMenuFlyoutItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleMenuFlyoutItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleMenuFlyoutItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> IToggleMenuFlyoutItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ToggleMenuFlyoutItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ToggleMenuFlyoutItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleMenuFlyoutItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleSwitchAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleSwitchAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleSwitchAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchAutomationPeerImpl, const OFFSET: isize>() -> IToggleSwitchAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleSwitchAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::ToggleSwitch>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleSwitchAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleSwitchAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IToggleSwitchAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleSwitchAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchAutomationPeerFactoryImpl, const OFFSET: isize>() -> IToggleSwitchAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleSwitchAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ToggleSwitch as ::windows::core::Abi>::Abi as *const <super::super::Controls::ToggleSwitch as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IToggleSwitchAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewItemAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewItemAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewItemAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewItemAutomationPeerImpl, const OFFSET: isize>() -> ITreeViewItemAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITreeViewItemAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewItemAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TreeViewItem>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewItemAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewItemAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewItemAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewItemAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewItemAutomationPeerFactoryImpl, const OFFSET: isize>() -> ITreeViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITreeViewItemAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TreeViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::TreeViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITreeViewItemAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewListAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewListAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewListAutomationPeerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewListAutomationPeerImpl, const OFFSET: isize>() -> ITreeViewListAutomationPeerVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITreeViewListAutomationPeer>, ::windows::core::GetTrustLevel)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITreeViewListAutomationPeerFactoryImpl: Sized {
    fn CreateInstanceWithOwner(&self, owner: &::core::option::Option<super::super::Controls::TreeViewList>, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<TreeViewListAutomationPeer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITreeViewListAutomationPeerFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.ITreeViewListAutomationPeerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ITreeViewListAutomationPeerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITreeViewListAutomationPeerFactoryImpl, const OFFSET: isize>() -> ITreeViewListAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITreeViewListAutomationPeerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TreeViewList as ::windows::core::Abi>::Abi as *const <super::super::Controls::TreeViewList as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITreeViewListAutomationPeerFactory>, ::windows::core::GetTrustLevel, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
