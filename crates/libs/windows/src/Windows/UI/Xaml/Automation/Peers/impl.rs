#[cfg(feature = "implement_exclusive")]
pub trait IAppBarAutomationPeerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarAutomationPeer {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Peers.IAppBarAutomationPeer";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarAutomationPeerVtbl {
    pub const fn new<Impl: IAppBarAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarAutomationPeer>, base.5)
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
    pub const fn new<Impl: IAppBarAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AppBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::AppBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppBarButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IAppBarButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AppBarButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::AppBarButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAppBarToggleButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarToggleButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarToggleButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IAppBarToggleButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarToggleButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAppBarToggleButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AppBarToggleButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::AppBarToggleButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarToggleButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutoSuggestBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutoSuggestBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutoSuggestBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: IAutoSuggestBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutoSuggestBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IAutoSuggestBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::AutoSuggestBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::AutoSuggestBox as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutoSuggestBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerVtbl {
        unsafe extern "system" fn EventsSource<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EventsSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventsSource<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEventsSource(&*(&value as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPattern<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPattern(patterninterface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaiseAutomationEvent<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: AutomationEvents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RaiseAutomationEvent(eventid).into()
        }
        unsafe extern "system" fn RaisePropertyChangedEvent<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, automationproperty: ::windows::core::RawPtr, oldvalue: *mut ::core::ffi::c_void, newvalue: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .RaisePropertyChangedEvent(
                    &*(&automationproperty as *const <super::AutomationProperty as ::windows::core::Abi>::Abi as *const <super::AutomationProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&oldvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&newvalue as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn GetAcceleratorKey<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAcceleratorKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKey<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlType<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutomationControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationId<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutomationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangle<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildren<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildren() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassName<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePoint<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClickablePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpText<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatus<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemType<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledBy<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLabeledBy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlType<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocalizedControlType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientation<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocus<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasKeyboardFocus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElement<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContentElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElement<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsControlElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsKeyboardFocusable<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsKeyboardFocusable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreen<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOffscreen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPassword<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPassword() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForForm<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRequiredForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocus<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocus().into()
        }
        unsafe extern "system" fn GetParent<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetParent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InvalidatePeer<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).InvalidatePeer().into()
        }
        unsafe extern "system" fn GetPeerFromPoint<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPeerFromPoint(&*(&point as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSetting<Impl: IAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAutomationPeer>,
            base.5,
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
    pub const fn new<Impl: IAutomationPeer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer2Vtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer2>, base.5)
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
    pub const fn new<Impl: IAutomationPeer3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer3Vtbl {
        unsafe extern "system" fn Navigate<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Navigate(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementFromPoint<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElementFromPoint(&*(&pointinwindowcoordinates as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElement<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowContextMenu<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowContextMenu().into()
        }
        unsafe extern "system" fn GetControlledPeers<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetControlledPeers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotations<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnnotations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParent<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetParent(&*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RaiseTextEditTextChangedEvent<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, automationtexteditchangetype: super::AutomationTextEditChangeType, changeddata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RaiseTextEditTextChangedEvent(automationtexteditchangetype, &*(&changeddata as *const <super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetPositionInSet<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPositionInSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSet<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSizeOfSet() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaiseStructureChangedEvent<Impl: IAutomationPeer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, structurechangetype: AutomationStructureChangeType, child: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RaiseStructureChangedEvent(structurechangetype, &*(&child as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAutomationPeer3>,
            base.5,
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
    pub const fn new<Impl: IAutomationPeer4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer4Vtbl {
        unsafe extern "system" fn GetLandmarkType<Impl: IAutomationPeer4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkType<Impl: IAutomationPeer4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocalizedLandmarkType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer4>, base.5, GetLandmarkType::<Impl, OFFSET>, GetLocalizedLandmarkType::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeer5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer5Vtbl {
        unsafe extern "system" fn IsPeripheral<Impl: IAutomationPeer5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPeripheral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForForm<Impl: IAutomationPeer5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDataValidForForm() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescription<Impl: IAutomationPeer5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFullDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer5>, base.5, IsPeripheral::<Impl, OFFSET>, IsDataValidForForm::<Impl, OFFSET>, GetFullDescription::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeer6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer6Vtbl {
        unsafe extern "system" fn GetCulture<Impl: IAutomationPeer6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCulture() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer6>, base.5, GetCulture::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeer7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer7Vtbl {
        unsafe extern "system" fn RaiseNotificationEvent<Impl: IAutomationPeer7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationkind: AutomationNotificationKind, notificationprocessing: AutomationNotificationProcessing, displaystring: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, activityid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RaiseNotificationEvent(notificationkind, notificationprocessing, &*(&displaystring as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&activityid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer7>, base.5, RaiseNotificationEvent::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeer8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer8Vtbl {
        unsafe extern "system" fn GetHeadingLevel<Impl: IAutomationPeer8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHeadingLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer8>, base.5, GetHeadingLevel::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeer9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeer9Vtbl {
        unsafe extern "system" fn IsDialog<Impl: IAutomationPeer9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDialog() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeer9>, base.5, IsDialog::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerAnnotationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerAnnotationVtbl {
        unsafe extern "system" fn Type<Impl: IAutomationPeerAnnotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::AnnotationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IAutomationPeerAnnotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::AnnotationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Peer<Impl: IAutomationPeerAnnotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Peer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPeer<Impl: IAutomationPeerAnnotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPeer(&*(&value as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerAnnotation>, base.5, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Peer::<Impl, OFFSET>, SetPeer::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerAnnotationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerAnnotationFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationPeerAnnotationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: super::AnnotationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithPeerParameter<Impl: IAutomationPeerAnnotationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: super::AnnotationType, peer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWithPeerParameter(r#type, &*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerAnnotationFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateWithPeerParameter::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerAnnotationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerAnnotationStaticsVtbl {
        unsafe extern "system" fn TypeProperty<Impl: IAutomationPeerAnnotationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TypeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PeerProperty<Impl: IAutomationPeerAnnotationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerAnnotationStatics>, base.5, TypeProperty::<Impl, OFFSET>, PeerProperty::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverridesVtbl {
        unsafe extern "system" fn GetPatternCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patterninterface: PatternInterface, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPatternCore(patterninterface) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAcceleratorKeyCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAcceleratorKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessKeyCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAccessKeyCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationControlTypeCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationControlType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutomationControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAutomationIdCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAutomationIdCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoundingRectangleCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBoundingRectangleCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChildrenCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetChildrenCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassNameCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClassNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClickablePointCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetClickablePointCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpTextCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHelpTextCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemStatusCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemStatusCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemTypeCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetItemTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLabeledByCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLabeledByCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedControlTypeCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocalizedControlTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNameCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientationCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationOrientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOrientationCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasKeyboardFocusCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasKeyboardFocusCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsContentElementCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsContentElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsControlElementCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsControlElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabledCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsKeyboardFocusableCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsKeyboardFocusableCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOffscreenCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOffscreenCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPasswordCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPasswordCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRequiredForFormCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRequiredForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusCore().into()
        }
        unsafe extern "system" fn GetPeerFromPointCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: super::super::super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPeerFromPointCore(&*(&point as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLiveSettingCore<Impl: IAutomationPeerOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLiveSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides>,
            base.5,
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
    pub const fn new<Impl: IAutomationPeerOverrides2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides2Vtbl {
        unsafe extern "system" fn ShowContextMenuCore<Impl: IAutomationPeerOverrides2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowContextMenuCore().into()
        }
        unsafe extern "system" fn GetControlledPeersCore<Impl: IAutomationPeerOverrides2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetControlledPeersCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides2>, base.5, ShowContextMenuCore::<Impl, OFFSET>, GetControlledPeersCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides3Vtbl {
        unsafe extern "system" fn NavigateCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NavigateCore(direction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetElementFromPointCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointinwindowcoordinates: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetElementFromPointCore(&*(&pointinwindowcoordinates as *const <super::super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFocusedElementCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFocusedElementCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAnnotationsCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAnnotationsCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPositionInSetCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPositionInSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSizeOfSetCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSizeOfSetCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelCore<Impl: IAutomationPeerOverrides3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides3>, base.5, NavigateCore::<Impl, OFFSET>, GetElementFromPointCore::<Impl, OFFSET>, GetFocusedElementCore::<Impl, OFFSET>, GetAnnotationsCore::<Impl, OFFSET>, GetPositionInSetCore::<Impl, OFFSET>, GetSizeOfSetCore::<Impl, OFFSET>, GetLevelCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverrides4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides4Vtbl {
        unsafe extern "system" fn GetLandmarkTypeCore<Impl: IAutomationPeerOverrides4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationLandmarkType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalizedLandmarkTypeCore<Impl: IAutomationPeerOverrides4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLocalizedLandmarkTypeCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides4>, base.5, GetLandmarkTypeCore::<Impl, OFFSET>, GetLocalizedLandmarkTypeCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides5Vtbl {
        unsafe extern "system" fn IsPeripheralCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPeripheralCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDataValidForFormCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDataValidForFormCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullDescriptionCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFullDescriptionCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescribedByCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDescribedByCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsToCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlowsToCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlowsFromCore<Impl: IAutomationPeerOverrides5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFlowsFromCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides5>, base.5, IsPeripheralCore::<Impl, OFFSET>, IsDataValidForFormCore::<Impl, OFFSET>, GetFullDescriptionCore::<Impl, OFFSET>, GetDescribedByCore::<Impl, OFFSET>, GetFlowsToCore::<Impl, OFFSET>, GetFlowsFromCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverrides6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides6Vtbl {
        unsafe extern "system" fn GetCultureCore<Impl: IAutomationPeerOverrides6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCultureCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides6>, base.5, GetCultureCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverrides8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides8Vtbl {
        unsafe extern "system" fn GetHeadingLevelCore<Impl: IAutomationPeerOverrides8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationHeadingLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetHeadingLevelCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides8>, base.5, GetHeadingLevelCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerOverrides9Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerOverrides9Vtbl {
        unsafe extern "system" fn IsDialogCore<Impl: IAutomationPeerOverrides9Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDialogCore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerOverrides9>, base.5, IsDialogCore::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerProtectedImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerProtectedVtbl {
        unsafe extern "system" fn PeerFromProvider<Impl: IAutomationPeerProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PeerFromProvider(&*(&provider as *const <super::Provider::IRawElementProviderSimple as ::windows::core::Abi>::Abi as *const <super::Provider::IRawElementProviderSimple as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderFromPeer<Impl: IAutomationPeerProtectedImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peer: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProviderFromPeer(&*(&peer as *const <AutomationPeer as ::windows::core::Abi>::Abi as *const <AutomationPeer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerProtected>, base.5, PeerFromProvider::<Impl, OFFSET>, ProviderFromPeer::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerStaticsVtbl {
        unsafe extern "system" fn ListenerExists<Impl: IAutomationPeerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: AutomationEvents, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListenerExists(eventid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerStatics>, base.5, ListenerExists::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationPeerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationPeerStatics3Vtbl {
        unsafe extern "system" fn GenerateRawElementProviderRuntimeId<Impl: IAutomationPeerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RawElementProviderRuntimeId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GenerateRawElementProviderRuntimeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationPeerStatics3>, base.5, GenerateRawElementProviderRuntimeId::<Impl, OFFSET>)
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
    pub const fn new<Impl: IButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Button as ::windows::core::Abi>::Abi as *const <super::super::Controls::Button as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IButtonBaseAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonBaseAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonBaseAutomationPeer>, base.5)
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
    pub const fn new<Impl: IButtonBaseAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonBaseAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IButtonBaseAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ButtonBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ButtonBase as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonBaseAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICalendarDatePickerAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICalendarDatePickerAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICalendarDatePickerAutomationPeer>, base.5)
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
    pub const fn new<Impl: ICalendarDatePickerAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICalendarDatePickerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICalendarDatePickerAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::CalendarDatePicker as ::windows::core::Abi>::Abi as *const <super::super::Controls::CalendarDatePicker as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICalendarDatePickerAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICaptureElementAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICaptureElementAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICaptureElementAutomationPeer>, base.5)
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
    pub const fn new<Impl: ICaptureElementAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICaptureElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICaptureElementAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::CaptureElement as ::windows::core::Abi>::Abi as *const <super::super::Controls::CaptureElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICaptureElementAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ICheckBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICheckBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICheckBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: ICheckBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICheckBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ICheckBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::CheckBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::CheckBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICheckBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IColorPickerSliderAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPickerSliderAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorPickerSliderAutomationPeer>, base.5)
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
    pub const fn new<Impl: IColorPickerSliderAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPickerSliderAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IColorPickerSliderAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ColorPickerSlider as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ColorPickerSlider as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorPickerSliderAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IColorSpectrumAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorSpectrumAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorSpectrumAutomationPeer>, base.5)
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
    pub const fn new<Impl: IColorSpectrumAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorSpectrumAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IColorSpectrumAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ColorSpectrum as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ColorSpectrum as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorSpectrumAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IComboBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: IComboBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IComboBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ComboBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::ComboBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IComboBoxItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IComboBoxItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IComboBoxItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ComboBoxItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ComboBoxItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IComboBoxItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IComboBoxItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IComboBoxItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxItemDataAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDatePickerAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDatePickerAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDatePickerAutomationPeer>, base.5)
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
    pub const fn new<Impl: IDatePickerAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDatePickerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IDatePickerAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::DatePicker as ::windows::core::Abi>::Abi as *const <super::super::Controls::DatePicker as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDatePickerAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IDatePickerFlyoutPresenterAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDatePickerFlyoutPresenterAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDatePickerFlyoutPresenterAutomationPeer>, base.5)
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
    pub const fn new<Impl: IFlipViewAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlipViewAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlipViewAutomationPeer>, base.5)
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
    pub const fn new<Impl: IFlipViewAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlipViewAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlipViewAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::FlipView as ::windows::core::Abi>::Abi as *const <super::super::Controls::FlipView as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlipViewAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFlipViewItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlipViewItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlipViewItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IFlipViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlipViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlipViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::FlipViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::FlipViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlipViewItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFlipViewItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlipViewItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlipViewItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IFlipViewItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlipViewItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IFlipViewItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlipViewItemDataAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFlyoutPresenterAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutPresenterAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutPresenterAutomationPeer>, base.5)
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
    pub const fn new<Impl: IFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutPresenterAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::FlyoutPresenter as ::windows::core::Abi>::Abi as *const <super::super::Controls::FlyoutPresenter as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutPresenterAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFrameworkElementAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementAutomationPeerVtbl {
        unsafe extern "system" fn Owner<Impl: IFrameworkElementAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Owner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementAutomationPeer>, base.5, Owner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFrameworkElementAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IFrameworkElementAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IFrameworkElementAutomationPeerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFrameworkElementAutomationPeerStaticsVtbl {
        unsafe extern "system" fn FromElement<Impl: IFrameworkElementAutomationPeerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePeerForElement<Impl: IFrameworkElementAutomationPeerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePeerForElement(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFrameworkElementAutomationPeerStatics>, base.5, FromElement::<Impl, OFFSET>, CreatePeerForElement::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGridViewAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewAutomationPeer>, base.5)
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
    pub const fn new<Impl: IGridViewAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GridView as ::windows::core::Abi>::Abi as *const <super::super::Controls::GridView as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGridViewHeaderItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewHeaderItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewHeaderItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IGridViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewHeaderItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GridViewHeaderItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::GridViewHeaderItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewHeaderItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGridViewItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IGridViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGridViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GridViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::GridViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGridViewItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IGridViewItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IGridViewItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewItemDataAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: IGroupItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGroupItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGroupItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IGroupItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGroupItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IGroupItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::GroupItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::GroupItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGroupItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHubAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHubAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHubAutomationPeer>, base.5)
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
    pub const fn new<Impl: IHubAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHubAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHubAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Hub as ::windows::core::Abi>::Abi as *const <super::super::Controls::Hub as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHubAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHubSectionAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHubSectionAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHubSectionAutomationPeer>, base.5)
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
    pub const fn new<Impl: IHubSectionAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHubSectionAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHubSectionAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::HubSection as ::windows::core::Abi>::Abi as *const <super::super::Controls::HubSection as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHubSectionAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IHyperlinkButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHyperlinkButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHyperlinkButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IHyperlinkButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHyperlinkButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IHyperlinkButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::HyperlinkButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::HyperlinkButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHyperlinkButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IImageAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageAutomationPeer>, base.5)
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
    pub const fn new<Impl: IImageAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IImageAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IImageAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Image as ::windows::core::Abi>::Abi as *const <super::super::Controls::Image as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IImageAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IInkToolbarAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInkToolbarAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInkToolbarAutomationPeer>, base.5)
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
    pub const fn new<Impl: IItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemAutomationPeerVtbl {
        unsafe extern "system" fn Item<Impl: IItemAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Item() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsControlAutomationPeer<Impl: IItemAutomationPeerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsControlAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemAutomationPeer>, base.5, Item::<Impl, OFFSET>, ItemsControlAutomationPeer::<Impl, OFFSET>)
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
    pub const fn new<Impl: IItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: IItemsControlAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemsControlAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeer>, base.5)
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
    pub const fn new<Impl: IItemsControlAutomationPeer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemsControlAutomationPeer2Vtbl {
        unsafe extern "system" fn CreateItemAutomationPeer<Impl: IItemsControlAutomationPeer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateItemAutomationPeer(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeer2>, base.5, CreateItemAutomationPeer::<Impl, OFFSET>)
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
    pub const fn new<Impl: IItemsControlAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemsControlAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IItemsControlAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ItemsControl as ::windows::core::Abi>::Abi as *const <super::super::Controls::ItemsControl as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IItemsControlAutomationPeerOverrides2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemsControlAutomationPeerOverrides2Vtbl {
        unsafe extern "system" fn OnCreateItemAutomationPeer<Impl: IItemsControlAutomationPeerOverrides2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnCreateItemAutomationPeer(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemsControlAutomationPeerOverrides2>, base.5, OnCreateItemAutomationPeer::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListBoxItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListBoxItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListBoxItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListBoxItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListBoxItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListBoxItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListBoxItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListBoxItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListBoxItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListBoxItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListBoxItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListBoxItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListBoxItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListBoxItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IListBoxItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListBoxItemDataAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListPickerFlyoutPresenterAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListPickerFlyoutPresenterAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListPickerFlyoutPresenterAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListView as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListView as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListViewBaseAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewBaseAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewBaseAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewBaseAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewBaseAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewBaseAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBase as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewBaseAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListViewBaseHeaderItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewBaseHeaderItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewBaseHeaderItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewBaseHeaderItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewBaseHeaderItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewBaseHeaderItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewBaseHeaderItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewBaseHeaderItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewBaseHeaderItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListViewHeaderItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewHeaderItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewHeaderItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewHeaderItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewHeaderItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewHeaderItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewHeaderItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewHeaderItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListViewItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IListViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ListViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ListViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IListViewItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IListViewItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IListViewItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemDataAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: ILoopingSelectorAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoopingSelectorAutomationPeer>, base.5)
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
    pub const fn new<Impl: ILoopingSelectorItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoopingSelectorItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: ILoopingSelectorItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoopingSelectorItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMapControlAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMapControlAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMapControlAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMediaElementAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaElementAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaElementAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMediaElementAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaElementAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MediaElement as ::windows::core::Abi>::Abi as *const <super::super::Controls::MediaElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaElementAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMediaPlayerElementAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaPlayerElementAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaPlayerElementAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMediaPlayerElementAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaPlayerElementAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaPlayerElementAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MediaPlayerElement as ::windows::core::Abi>::Abi as *const <super::super::Controls::MediaPlayerElement as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaPlayerElementAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMediaTransportControlsAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTransportControlsAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTransportControlsAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMediaTransportControlsAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMediaTransportControlsAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMediaTransportControlsAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MediaTransportControls as ::windows::core::Abi>::Abi as *const <super::super::Controls::MediaTransportControls as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMediaTransportControlsAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMenuBarAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuBarAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuBarAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMenuBarAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMenuBarAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&owner as *const <super::super::Controls::MenuBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuBarAutomationPeerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMenuBarItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuBarItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuBarItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMenuBarItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuBarItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMenuBarItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&owner as *const <super::super::Controls::MenuBarItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuBarItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuBarItemAutomationPeerFactory>, base.5, CreateInstance::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMenuFlyoutItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuFlyoutItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuFlyoutItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuFlyoutItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MenuFlyoutItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuFlyoutItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuFlyoutItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IMenuFlyoutPresenterAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuFlyoutPresenterAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuFlyoutPresenterAutomationPeer>, base.5)
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
    pub const fn new<Impl: IMenuFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuFlyoutPresenterAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IMenuFlyoutPresenterAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::MenuFlyoutPresenter as ::windows::core::Abi>::Abi as *const <super::super::Controls::MenuFlyoutPresenter as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuFlyoutPresenterAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: INavigationViewItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationViewItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationViewItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: INavigationViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: INavigationViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::NavigationViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::NavigationViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationViewItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPasswordBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPasswordBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPasswordBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: IPasswordBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPasswordBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPasswordBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::PasswordBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::PasswordBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPasswordBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPersonPictureAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPersonPictureAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPersonPictureAutomationPeer>, base.5)
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
    pub const fn new<Impl: IPersonPictureAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPersonPictureAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPersonPictureAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::PersonPicture as ::windows::core::Abi>::Abi as *const <super::super::Controls::PersonPicture as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPersonPictureAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPickerFlyoutPresenterAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPickerFlyoutPresenterAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPickerFlyoutPresenterAutomationPeer>, base.5)
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
    pub const fn new<Impl: IPivotAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotAutomationPeer>, base.5)
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
    pub const fn new<Impl: IPivotAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPivotAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Pivot as ::windows::core::Abi>::Abi as *const <super::super::Controls::Pivot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPivotItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IPivotItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IPivotItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::PivotItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::PivotItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IPivotItemDataAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotItemDataAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotItemDataAutomationPeer>, base.5)
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
    pub const fn new<Impl: IPivotItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotItemDataAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: IPivotItemDataAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithParentAndItem(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&parent as *const <PivotAutomationPeer as ::windows::core::Abi>::Abi as *const <PivotAutomationPeer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotItemDataAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: IProgressBarAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProgressBarAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProgressBarAutomationPeer>, base.5)
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
    pub const fn new<Impl: IProgressBarAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProgressBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IProgressBarAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ProgressBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::ProgressBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProgressBarAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IProgressRingAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProgressRingAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProgressRingAutomationPeer>, base.5)
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
    pub const fn new<Impl: IProgressRingAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProgressRingAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IProgressRingAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ProgressRing as ::windows::core::Abi>::Abi as *const <super::super::Controls::ProgressRing as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProgressRingAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRadioButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadioButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadioButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRadioButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRadioButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRadioButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RadioButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::RadioButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRadioButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRangeBaseAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBaseAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRangeBaseAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRangeBaseAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::RangeBase as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::RangeBase as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBaseAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRatingControlAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRatingControlAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRatingControlAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRatingControlAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRatingControlAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRatingControlAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RatingControl as ::windows::core::Abi>::Abi as *const <super::super::Controls::RatingControl as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRatingControlAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRepeatButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepeatButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepeatButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRepeatButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepeatButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRepeatButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::RepeatButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::RepeatButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepeatButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRichEditBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichEditBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichEditBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRichEditBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichEditBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichEditBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RichEditBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::RichEditBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichEditBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRichTextBlockAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichTextBlockAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichTextBlockAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRichTextBlockAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichTextBlockAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichTextBlockAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RichTextBlock as ::windows::core::Abi>::Abi as *const <super::super::Controls::RichTextBlock as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichTextBlockAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IRichTextBlockOverflowAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichTextBlockOverflowAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichTextBlockOverflowAutomationPeer>, base.5)
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
    pub const fn new<Impl: IRichTextBlockOverflowAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRichTextBlockOverflowAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IRichTextBlockOverflowAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::RichTextBlockOverflow as ::windows::core::Abi>::Abi as *const <super::super::Controls::RichTextBlockOverflow as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRichTextBlockOverflowAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IScrollBarAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollBarAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollBarAutomationPeer>, base.5)
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
    pub const fn new<Impl: IScrollBarAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollBarAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IScrollBarAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ScrollBar as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ScrollBar as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollBarAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IScrollViewerAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollViewerAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollViewerAutomationPeer>, base.5)
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
    pub const fn new<Impl: IScrollViewerAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollViewerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IScrollViewerAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ScrollViewer as ::windows::core::Abi>::Abi as *const <super::super::Controls::ScrollViewer as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollViewerAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISearchBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: ISearchBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISearchBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISearchBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::SearchBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::SearchBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISearchBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISelectorAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorAutomationPeer>, base.5)
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
    pub const fn new<Impl: ISelectorAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISelectorAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::Selector as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::Selector as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISelectorItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: ISelectorItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithParentAndItem<Impl: ISelectorItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, parent: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorItemAutomationPeerFactory>, base.5, CreateInstanceWithParentAndItem::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISemanticZoomAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISemanticZoomAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISemanticZoomAutomationPeer>, base.5)
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
    pub const fn new<Impl: ISemanticZoomAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISemanticZoomAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISemanticZoomAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::SemanticZoom as ::windows::core::Abi>::Abi as *const <super::super::Controls::SemanticZoom as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISemanticZoomAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISettingsFlyoutAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsFlyoutAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsFlyoutAutomationPeer>, base.5)
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
    pub const fn new<Impl: ISettingsFlyoutAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsFlyoutAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISettingsFlyoutAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::SettingsFlyout as ::windows::core::Abi>::Abi as *const <super::super::Controls::SettingsFlyout as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsFlyoutAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ISliderAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISliderAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISliderAutomationPeer>, base.5)
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
    pub const fn new<Impl: ISliderAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISliderAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ISliderAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Slider as ::windows::core::Abi>::Abi as *const <super::super::Controls::Slider as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISliderAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITextBlockAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITextBlockAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITextBlockAutomationPeer>, base.5)
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
    pub const fn new<Impl: ITextBlockAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITextBlockAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITextBlockAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TextBlock as ::windows::core::Abi>::Abi as *const <super::super::Controls::TextBlock as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITextBlockAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITextBoxAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITextBoxAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITextBoxAutomationPeer>, base.5)
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
    pub const fn new<Impl: ITextBoxAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITextBoxAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITextBoxAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TextBox as ::windows::core::Abi>::Abi as *const <super::super::Controls::TextBox as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITextBoxAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IThumbAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThumbAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThumbAutomationPeer>, base.5)
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
    pub const fn new<Impl: IThumbAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThumbAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IThumbAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::Thumb as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::Thumb as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThumbAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITimePickerAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimePickerAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimePickerAutomationPeer>, base.5)
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
    pub const fn new<Impl: ITimePickerAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimePickerAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITimePickerAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TimePicker as ::windows::core::Abi>::Abi as *const <super::super::Controls::TimePicker as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimePickerAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITimePickerFlyoutPresenterAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITimePickerFlyoutPresenterAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITimePickerFlyoutPresenterAutomationPeer>, base.5)
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
    pub const fn new<Impl: IToggleButtonAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleButtonAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleButtonAutomationPeer>, base.5)
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
    pub const fn new<Impl: IToggleButtonAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleButtonAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleButtonAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::Primitives::ToggleButton as ::windows::core::Abi>::Abi as *const <super::super::Controls::Primitives::ToggleButton as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleButtonAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IToggleMenuFlyoutItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleMenuFlyoutItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleMenuFlyoutItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: IToggleMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleMenuFlyoutItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleMenuFlyoutItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ToggleMenuFlyoutItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::ToggleMenuFlyoutItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleMenuFlyoutItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: IToggleSwitchAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleSwitchAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleSwitchAutomationPeer>, base.5)
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
    pub const fn new<Impl: IToggleSwitchAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleSwitchAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: IToggleSwitchAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::ToggleSwitch as ::windows::core::Abi>::Abi as *const <super::super::Controls::ToggleSwitch as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleSwitchAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITreeViewItemAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITreeViewItemAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITreeViewItemAutomationPeer>, base.5)
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
    pub const fn new<Impl: ITreeViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITreeViewItemAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITreeViewItemAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TreeViewItem as ::windows::core::Abi>::Abi as *const <super::super::Controls::TreeViewItem as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITreeViewItemAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
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
    pub const fn new<Impl: ITreeViewListAutomationPeerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITreeViewListAutomationPeerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITreeViewListAutomationPeer>, base.5)
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
    pub const fn new<Impl: ITreeViewListAutomationPeerFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITreeViewListAutomationPeerFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithOwner<Impl: ITreeViewListAutomationPeerFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: ::windows::core::RawPtr, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithOwner(&*(&owner as *const <super::super::Controls::TreeViewList as ::windows::core::Abi>::Abi as *const <super::super::Controls::TreeViewList as ::windows::core::DefaultType>::DefaultType), &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITreeViewListAutomationPeerFactory>, base.5, CreateInstanceWithOwner::<Impl, OFFSET>)
    }
}
